use std::ffi::c_int;

use lua54 as lua;
use lua::{lua_State};
use lua_macro::{lua_cfunction};

mod util;

struct Person {
    x: i32,
    y: i32,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("drop Person: x = {}, y = {}", self.x, self.y);
    }
}

#[lua_cfunction]
fn meth_gc(l: *mut lua_State) -> c_int {
    let u = util::touserdata::<Person>(l, 1);
    println!("gc Person: x = {}, y = {}", u.ptr.x, u.ptr.y);
    unsafe{ std::ptr::read(&u.ptr) };
    return 0;
}

#[lua_cfunction]
fn meth_create(l: *mut lua_State) -> c_int {
    util::newuserdata(l, Person{x: 0, y: 0});

    lua::lua_newtable(l);

    lua::lua_pushstring(l, "__gc");
    lua::lua_pushcfunction(l, Some(meth_gc));
    lua::lua_rawset(l, -3);

    lua::lua_setmetatable(l, -2);

    return 1;
}

#[lua_cfunction]
fn meth_show(l: *mut lua_State) -> c_int {
    let u = util::touserdata::<Person>(l, 1);

    println!("x = {}, y = {}", u.ptr.x, u.ptr.y);

    u.ptr.x = u.ptr.x + 1;
    u.ptr.y = u.ptr.y + 1;

    return 0;
}

#[lua_cfunction]
fn luaopen_tcp(l: *mut lua_State) -> c_int {
    lua::lua_newtable(l);

    lua::lua_pushstring(l, "create");
    lua::lua_pushcfunction(l, Some(meth_create));
    lua::lua_rawset(l, -3);

    lua::lua_pushstring(l, "show");
    lua::lua_pushcfunction(l, Some(meth_show));
    lua::lua_rawset(l, -3);

    return 1;
}