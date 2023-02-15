use std::ffi::c_int;
use std::net::TcpStream;
use std::io::Write;

use lua54 as lua;
use lua_macro::lua_cfunction;

mod userdata;

#[lua_cfunction]
fn meth_gc(l: *mut lua::lua_State) -> c_int {
    let u = userdata::touserdata::<TcpStream>(l, 1);
    userdata::dropuserdata(u);
    return 0;
}

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

#[lua_cfunction]
fn luaopen_tcp(l: *mut lua::lua_State) -> c_int {
    lua::luaL_newmetatable(l, "Net::TCP");

    lua::lua_pushstring(l, "__gc");
    lua::lua_pushcfunction(l, Some(meth_gc));
    lua::lua_rawset(l, -3);

    lua::lua_pushstring(l, "__index");
    lua::lua_newtable(l);
    
    lua::lua_pushstring(l, "send");
    lua::lua_pushcfunction(l, Some(meth_send));
    lua::lua_rawset(l, -3);
    
    lua::lua_rawset(l, -3);

    // Module table
    lua::lua_newtable(l);

    lua::lua_pushstring(l, "connect");
    lua::lua_pushcfunction(l, Some(meth_connect));
    lua::lua_rawset(l, -3);

    return 1;
}