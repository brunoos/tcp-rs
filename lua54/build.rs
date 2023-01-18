extern crate bindgen;

fn main() {
    let bindings = bindgen::Builder::default()
    .header("src/wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .clang_arg("-I/home/local/lua/5.4/include")
    .allowlist_function("lua_createtable")
    .allowlist_function("lua_newuserdatauv")
    .allowlist_function("lua_pushcclosure")
    .allowlist_function("lua_pushinteger")
    .allowlist_function("lua_pushnumber")
    .allowlist_function("lua_rawset")
    .allowlist_function("lua_touserdata")
    .generate()
    .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/liblua.rs")
        .expect("Couldn't write bindings!");
}