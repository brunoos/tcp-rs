use std::ffi::c_int;

use lua54 as lua;
use lua::lua_State;

use lua_macro::{lua_cfunction};

struct Person {
    x: i32,
    y: i32,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("drop Person: x = {}, y = {}", self.x, self.y);
    }
}

struct UserData<T> {
    ptr: Box<T>
}

impl<T> Drop for UserData<T> {
    fn drop(&mut self) {
        println!("drop UserData");
    }
}

fn newuserdata<'a, T>(l: *mut lua_State, data: T) -> &'a UserData<T> {
    unsafe {
        let ptr = lua::lua_newuserdata(l, std::mem::size_of::<UserData<T>>()) as *mut UserData<T>;
        let u: &mut UserData<T> = &mut *ptr;
        println!("ok");
        std::ptr::write(&mut u.ptr, Box::new(data));
        return u;
    }
}

fn touserdata<'a, T>(l: *mut lua_State, idx: c_int) -> &'a mut UserData<T> {
    unsafe {
        let ptr = lua::lua_touserdata(l, idx) as *mut UserData<T>;
        return & mut *ptr;
    }
}

#[lua_cfunction]
fn meth_create(l: *mut lua_State) -> c_int {
    let p = Person{x: 0, y: 0};
    newuserdata(l, p);
    return 1;
}

#[lua_cfunction]
fn meth_show(l: *mut lua_State) -> c_int {
    let u = touserdata::<Person>(l, 1);
    
    println!("x = {}, y = {}", u.ptr.x, u.ptr.y);
    
    u.ptr.x = u.ptr.x + 1;
    u.ptr.y = u.ptr.y + 1;

    return 0;
}

#[lua_cfunction]
fn luaopen_tcp(l: *mut lua_State) -> c_int {
    lua::lua_newtable(l);

    lua::lua_pushinteger(l, 1);
    lua::lua_pushcfunction(l, Some(meth_create));
    lua::lua_rawset(l, -3);

    lua::lua_pushinteger(l, 2);
    lua::lua_pushcfunction(l, Some(meth_show));
    lua::lua_rawset(l, -3);

    return 1;
}