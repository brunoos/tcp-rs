use std::ptr;
use std::mem;
use std::ffi::c_int;
use std::net::TcpStream;
use std::io::{Read,Write};
use std::os::fd::AsRawFd;
use std::vec;

use libc;

use lua54 as lua;
use lua_macro::lua_cfunction;

mod userdata;

//-----------------------------------------------------------------------------

#[lua_cfunction]
fn meth_gc(l: *mut lua::lua_State) -> c_int {
    let u = userdata::touserdata::<TcpStream>(l, 1);
    userdata::dropuserdata(u);
    return 0;
}

//-----------------------------------------------------------------------------

#[lua_cfunction]
fn meth_connect(l: *mut lua::lua_State) -> c_int {
    let addr = lua::lua_tostring(l, 1);
    let res = TcpStream::connect(addr);
    if res.is_err() {
        lua::lua_pushnil(l);
        return 1;
    }
    
    userdata::newuserdata(l, res.unwrap());
    lua::luaL_setmetatable(l, "Net::TCP");

    return 1;
}

//-----------------------------------------------------------------------------

#[lua_cfunction]
fn meth_send(l: *mut lua::lua_State) -> c_int {
    let u = userdata::testudata::<TcpStream>(l, 1, "Net::TCP");
    if u.is_none() {
        lua::lua_pushnil(l);
        return 1;
    }
    let u = u.unwrap();
    let data = lua::lua_toslice(l, 2);
    let res = u.data.write(data);
    if res.is_err() {
        lua::lua_pushnil(l);
        return 1;
    }
    lua::lua_pushinteger(l, res.unwrap() as i64);
    return 1;
}

//-----------------------------------------------------------------------------

#[lua_cfunction]
fn meth_recv(l: *mut lua::lua_State) -> c_int {
    let u = userdata::testudata::<TcpStream>(l, 1, "Net::TCP");
    if u.is_none() {
        lua::lua_pushnil(l);
        return 1;
    }
    let u = u.unwrap();
    let mut buf = vec![0; 4096];
    let r = u.data.read(buf.as_mut_slice());
    if r.is_err() {
        lua::lua_pushnil(l);
        return 1;
    }
    let size = r.unwrap();
    let data = buf.as_slice();
    lua::lua_pushbytes(l, &data[0..size]);
    return 1;
}

//-----------------------------------------------------------------------------

#[lua_cfunction]
fn meth_get_fd(l: *mut lua::lua_State) -> c_int {
    let u = userdata::testudata::<TcpStream>(l, 1, "Net::TCP");
    if u.is_none() {
        lua::lua_pushnil(l);
        return 1;
    }
    let u = u.unwrap();
    lua::lua_pushinteger(l, u.data.as_raw_fd() as i64);
    return 1;
}

//-----------------------------------------------------------------------------

fn new_fd_set() -> libc::fd_set {
    unsafe {
        let mut set = mem::MaybeUninit::<libc::fd_set>::uninit();
        libc::FD_ZERO(set.as_mut_ptr());
        set.assume_init()
    }
}

fn table_to_fd_set(l: *mut lua::lua_State, idx: c_int, nfds: &mut i32, set: *mut libc::fd_set)  {
    let mut i: i64 = 1;
    loop {
        lua::lua_pushinteger(l, i);
        lua::lua_rawget(l, idx);
        if lua::lua_isnil(l, -1) {
            lua::lua_pop(l, 1);
            break;
        }

        let fd = lua::lua_tointeger(l, -1) as i32;
        lua::lua_pop(l, 1);
        unsafe { libc::FD_SET(fd, set) };

        if fd > *nfds { *nfds = fd; }
        i += 1;
    }
}

fn set_active_fd(l: *mut lua::lua_State, idx: c_int, set: *mut libc::fd_set) {
    let mut i = 1;
    loop {
        lua::lua_pushinteger(l, i);
        lua::lua_rawget(l, idx);
        if lua::lua_isnil(l, -1) {
            lua::lua_pop(l, 1);
            break;
        }

        let fd = lua::lua_tointeger(l, -1) as i32;
        lua::lua_pop(l, 1);

        let b = unsafe{ libc::FD_ISSET(fd, set) };
        lua::lua_pushinteger(l, i);
        lua::lua_pushboolean(l, b);
        lua::lua_rawset(l, idx);

        i += 1;
    }
}

#[lua_cfunction]
fn meth_select(l: *mut lua::lua_State) -> c_int {
    let mut nfds: i32 = -1;

    let mut readers = new_fd_set();
    let mut writers = new_fd_set();
    let mut excepts = new_fd_set();

    let rset = if !lua::lua_isnil(l, 1) { 
        let set = &mut readers as *mut libc::fd_set;
        table_to_fd_set(l, 1, &mut nfds, set);
        set
     } else {
        ptr::null_mut()
    };

    let wset = if !lua::lua_isnil(l, 2) { 
        let set = &mut writers as *mut libc::fd_set;
        table_to_fd_set(l, 2, &mut nfds, set);
        set
     } else {
        ptr::null_mut()
    };

    let eset = if !lua::lua_isnil(l, 3) { 
        let set = &mut excepts as *mut libc::fd_set;
        table_to_fd_set(l, 3, &mut nfds, set);
        set
     } else {
        ptr::null_mut()
    };

    let res = unsafe{ libc::select(nfds+1, rset, wset, eset, ptr::null_mut()) };
    if res == -1 {
        lua::lua_pushnil(l);
        return 1;
    }

    if !rset.is_null() { set_active_fd(l, 1, rset); }
    if !wset.is_null() { set_active_fd(l, 2, wset); }
    if !eset.is_null() { set_active_fd(l, 3, eset); }

    lua::lua_pushboolean(l, true);
    return 1;
}

//-----------------------------------------------------------------------------

#[lua_cfunction]
fn luaopen_net_tcp_core(l: *mut lua::lua_State) -> c_int {
    lua::luaL_newmetatable(l, "Net::TCP");

    lua::lua_pushstring(l, "__gc");
    lua::lua_pushcfunction(l, Some(meth_gc));
    lua::lua_rawset(l, -3);

    lua::lua_pushstring(l, "__index");
    lua::lua_newtable(l);
    
    lua::lua_pushstring(l, "receive");
    lua::lua_pushcfunction(l, Some(meth_recv));
    lua::lua_rawset(l, -3);

    lua::lua_pushstring(l, "send");
    lua::lua_pushcfunction(l, Some(meth_send));
    lua::lua_rawset(l, -3);
    
    lua::lua_pushstring(l, "_getfd");
    lua::lua_pushcfunction(l, Some(meth_get_fd));
    lua::lua_rawset(l, -3);

    lua::lua_rawset(l, -3);

    // Module table
    lua::lua_newtable(l);

    lua::lua_pushstring(l, "connect");
    lua::lua_pushcfunction(l, Some(meth_connect));
    lua::lua_rawset(l, -3);

    lua::lua_pushstring(l, "select");
    lua::lua_pushcfunction(l, Some(meth_select));
    lua::lua_rawset(l, -3);

    return 1;
}