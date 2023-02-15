use std::ffi::c_int;
use std::mem;
use std::ptr;

use lua54 as lua;
use lua::lua_State;

pub struct UserData<T> {
    pub data: Box<T>
}

pub fn dropuserdata<T>(u: &UserData<T>) {
    unsafe {
        let ptr = &u.data as *const Box<T>;
        ptr::read(ptr);
    }
}

pub fn newuserdata<'a, T>(l: *mut lua_State, data: T) -> &'a mut UserData<T> {
    unsafe {
        let ptr = lua::lua_newuserdata(l, mem::size_of::<UserData<T>>()) as *mut UserData<T>;
        let u: &mut UserData<T> = &mut *ptr;
        ptr::write(&mut u.data, Box::new(data));
        return u;
    }
}

pub fn touserdata<'a, T>(l: *mut lua_State, idx: c_int) -> &'a mut UserData<T> {
    unsafe {
        let ptr = lua::lua_touserdata(l, idx) as *mut UserData<T>;
        return &mut *ptr;
    }
}

pub fn testudata<'a, T>(l: *mut lua_State, idx: c_int, name: &str) -> Option<&'a mut UserData<T>> {
    unsafe {
        let ptr = lua::luaL_testudata(l, idx, name);
        if ptr.is_null() {
            return None;
        }
        return Some(&mut *(ptr as *mut UserData<T>));
    }
}