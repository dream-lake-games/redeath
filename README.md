# Redeath

A game about dying.

# WASM stuff

Add

```toml
[target.wasm32-unknown-unknown]
rustflags = ["--cfg=web_sys_unstable_apis"]
runner = "wasm-server-runner"
```

to `.cargo/config.toml`.

Building with trunk:

```sh
# From project root
trunk build
# The -o just opens index
npx http-server dist -o
```

