extern crate bindgen;

use std::env;

fn main() {
    let path = env::var("LUA_INCDIR").unwrap();
    let include = format!("-I{}", path);

    let bindings = bindgen::Builder::default()
    .header("src/wrapper.h")
    .clang_arg(include)
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .allowlist_function("lua_createtable")
    .allowlist_function("lua_newuserdatauv")
    .allowlist_function("lua_pushboolean")
    .allowlist_function("lua_pushcclosure")
    .allowlist_function("lua_pushinteger")
    .allowlist_function("lua_pushnil")
    .allowlist_function("lua_pushlstring")
    .allowlist_function("lua_rawget")
    .allowlist_function("lua_rawset")
    .allowlist_function("lua_setmetatable")
    .allowlist_function("lua_settop")
    .allowlist_function("lua_tointegerx")
    .allowlist_function("lua_tolstring")
    .allowlist_function("lua_touserdata")
    .allowlist_function("lua_type")
    .allowlist_function("luaL_newmetatable")
    .allowlist_function("luaL_setmetatable")
    .allowlist_function("luaL_testudata")
    .allowlist_var("LUA_TNIL")
    .allowlist_var("LUA_TTABLE")
    .generate()
    .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/liblua.rs")
        .expect("Couldn't write bindings!");
}