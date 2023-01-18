#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod liblua;

use std::ffi::{c_int, c_void};

pub use liblua::{lua_State, lua_Number, lua_Integer, lua_CFunction};

pub fn lua_newtable(l: *mut lua_State) {
    unsafe {
        liblua::lua_createtable(l, 0, 0);
    }
}

pub fn lua_pushnumber(l: *mut lua_State, n: lua_Number) {
    unsafe {
        liblua::lua_pushnumber(l, n);
    }
}

pub fn lua_pushinteger(l: *mut lua_State, n: lua_Integer) {
    unsafe {
        liblua::lua_pushinteger(l, n);
    }
}

pub fn lua_rawset(l: *mut lua_State, idx: c_int) {
    unsafe {
        liblua::lua_rawset(l, idx);
    }
}

pub fn lua_pushcfunction(l: *mut lua_State, f: lua_CFunction) {
    unsafe {
        liblua::lua_pushcclosure(l, f, 0);
    }
}

pub fn lua_newuserdata(l: *mut lua_State, s: usize) -> *mut c_void {
    unsafe {
        liblua::lua_newuserdatauv(l, s, 1)
    }
}

pub fn lua_touserdata(l: *mut lua_State, idx: c_int) -> *mut c_void {
    unsafe {
        liblua::lua_touserdata(l, idx)
    }
}