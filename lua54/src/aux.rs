use std::ffi::c_int;
use std::mem;
use std::ptr;

use super::lua_State;

pub struct UserData<T> {
    pub ptr: Box<T>
}

pub fn newuserdata<'a, T>(l: *mut lua_State, data: T) -> &'a UserData<T> {
    unsafe {
        let ptr = super::lua_newuserdata(l, mem::size_of::<UserData<T>>()) as *mut UserData<T>;
        let u: &mut UserData<T> = &mut *ptr;
        ptr::write(&mut u.ptr, Box::new(data));
        return u;
    }
}

pub fn touserdata<'a, T>(l: *mut lua_State, idx: c_int) -> &'a UserData<T> {
    unsafe {
        let ptr = super::lua_touserdata(l, idx) as *const UserData<T>;
        return &*ptr;
    }
}
