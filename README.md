# BUILD

```sh
$ cargo build --release
```

If Lua 5.4 headers are not in your default include path, you can set `LUA_INCDIR` variable.
```sh
$ export LUA_INCDIR=/opt/local/lua/5.4/include
$ cargo build --release
```

# INSTALL

```sh
$ mkdir -p LUA_PATH/net
$ cp src/tcp.lua LUA_PATH/net

$ mkdir -p LUA_CPATH/net/tcp
$ cp target/release/libtcp.so LUA_CPATH/net/tcp/core.so
```

# Example

```lua
local tcp = require("net.tcp")
local conn = tcp.connect("localhost:5000")
conn:send("Hello world!")
```