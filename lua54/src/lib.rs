#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{c_int, c_void, CString};
use std::slice;
use std::str;
use std::ptr;

mod liblua;

pub use liblua::{lua_State, lua_CFunction};

pub const LUA_TNIL: i32 = liblua::LUA_TNIL as i32;
pub const LUA_TTABLE: i32 = liblua::LUA_TTABLE as i32;

pub fn lua_isnil(l: *mut lua_State, idx: c_int) -> bool {
    unsafe {
        liblua::lua_type(l, idx) == LUA_TNIL
    }
}

pub fn lua_istable(l: *mut lua_State, idx: c_int) -> bool {
    unsafe {
        liblua::lua_type(l, idx) == LUA_TTABLE
    }
}

pub fn lua_newtable(l: *mut lua_State) {
    unsafe {
        liblua::lua_createtable(l, 0, 0);
    }
}

pub fn lua_newuserdata(l: *mut lua_State, s: usize) -> *mut c_void {
    unsafe {
        liblua::lua_newuserdatauv(l, s, 1)
    }
}

pub fn lua_pop(l: *mut lua_State, n: c_int) {
    unsafe {
        liblua::lua_settop(l, (-n) - 1);
    }
}

pub fn lua_pushboolean(l: *mut lua_State, b: bool) {
    unsafe {
        let v = if b { 1 } else { 0 };
        liblua::lua_pushboolean(l, v);
    }
}

pub fn lua_pushcfunction(l: *mut lua_State, f: lua_CFunction) {
    unsafe {
        liblua::lua_pushcclosure(l, f, 0);
    }
}

pub fn lua_pushinteger(l: *mut lua_State, n: i64) {
    unsafe {
        liblua::lua_pushinteger(l, n);
    }
}

pub fn lua_pushlstring(l: *mut lua_State, s: &str, len: usize) {
    unsafe {
        liblua::lua_pushlstring(l, s.as_ptr() as *const i8, len);
    }
}

pub fn lua_pushbytes(l: *mut lua_State, s: &[u8]) {
    unsafe {
        liblua::lua_pushlstring(l, s.as_ptr() as *const i8, s.len());
    }
}

pub fn lua_pushnil(l: *mut lua_State) {
    unsafe {
        liblua::lua_pushnil(l);
    }
}

pub fn lua_pushstring(l: *mut lua_State, s: &str) {
    unsafe {
        liblua::lua_pushlstring(l, s.as_ptr() as *const i8, s.len());
    }
}

pub fn lua_rawget(l: *mut lua_State, idx: c_int) {
    unsafe {
        liblua::lua_rawget(l, idx);
    }
}

pub fn lua_rawset(l: *mut lua_State, idx: c_int) {
    unsafe {
        liblua::lua_rawset(l, idx);
    }
}

pub fn lua_tointeger(l: *mut lua_State, idx: c_int) -> i64 {
    unsafe {
        liblua::lua_tointegerx(l, idx, ptr::null_mut())
    }
}

pub fn lua_touserdata(l: *mut lua_State, idx: c_int) -> *mut c_void {
    unsafe {
        liblua::lua_touserdata(l, idx)
    }
}

pub fn lua_type(l: *mut lua_State, idx: c_int) -> i32 {
    unsafe {
        liblua::lua_type(l, idx)
    }
}

pub fn lua_setmetatable(l: *mut lua_State, idx: c_int) -> c_int {
    unsafe {
        liblua::lua_setmetatable(l, idx)
    }
}

pub fn lua_settop(l: *mut lua_State, idx: c_int) {
    unsafe {
        liblua::lua_settop(l, idx);
    }
}

pub fn lua_tolstring(l: *mut lua_State, idx: c_int, len: &mut usize) -> *const i8 {
    unsafe {
        liblua::lua_tolstring(l, idx, len)
    }
}

pub fn lua_toslice<'a>(l: *mut lua_State, idx: c_int) -> &'a [u8] {
    unsafe {
        let mut len: usize = 0;
        let ptr = liblua::lua_tolstring(l, idx, &mut len);
        slice::from_raw_parts(ptr as *const u8, len)
    }
}

pub fn lua_tostring<'a>(l: *mut lua_State, idx: c_int) -> &'a str {
    unsafe {
        let mut len: usize = 0;
        let ptr = liblua::lua_tolstring(l, idx, &mut len);
        str::from_utf8_unchecked(slice::from_raw_parts(ptr as *const u8, len))
    }
}

pub fn luaL_newmetatable(l: *mut lua_State, name: &str) -> c_int {
    unsafe {
        let v = name.as_bytes().to_vec();
        let ptr = CString::from_vec_unchecked(v);
        liblua::luaL_newmetatable(l, ptr.as_ptr())
    }
}

pub fn luaL_setmetatable(l: *mut lua_State, name: &str) {
    unsafe {
        let v = name.as_bytes().to_vec();
        let ptr = CString::from_vec_unchecked(v);
        liblua::luaL_setmetatable(l, ptr.as_ptr())
    }
}

pub fn luaL_testudata(l: *mut lua_State, idx: c_int, name: &str) -> *mut c_void {
    unsafe {
        let v = name.as_bytes().to_vec();
        let ptr = CString::from_vec_unchecked(v);
        liblua::luaL_testudata(l, idx, ptr.as_ptr())
    }
}
