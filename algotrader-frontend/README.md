# Algo Trader Frontend

* Install Trunk

```
brew install trunk
```

* Build

```
trunk build
```

* Build Errors

*error[E0463]: can't find crate for `core`*

```
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
trunk build
```

* Build Output

```
../dydx/dydx-bot/dist
```

* Run

```
trunk serve
```
