use std::ffi::c_int;

use lua54 as lua;
use lua::lua_State;

pub struct UserData<T> {
    pub ptr: Box<T>
}

pub fn newuserdata<'a, T>(l: *mut lua_State, data: T) -> &'a UserData<T> {
    unsafe {
        let ptr = lua::lua_newuserdata(l, std::mem::size_of::<UserData<T>>()) as *mut UserData<T>;
        let u: &mut UserData<T> = &mut *ptr;
        println!("ok");
        std::ptr::write(&mut u.ptr, Box::new(data));
        return u;
    }
}

pub fn touserdata<'a, T>(l: *mut lua_State, idx: c_int) -> &'a mut UserData<T> {
    unsafe {
        let ptr = lua::lua_touserdata(l, idx) as *mut UserData<T>;
        return & mut *ptr;
    }
}
