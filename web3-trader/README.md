# WEB3 Trader

```
cd web3-trader
```

* Build

```
cargo clean
cargo build
cargo build -r
```

* Run

```
cargo run
cargo run -r
```

* Build Docker

```
docker build -t asa/web3-trader .
docker run -p 8080:8080 asa/web3-trader

curl -kv http://localhost:8080/health_check
```