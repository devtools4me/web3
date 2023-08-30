# DYDX Bot

* Build

```
cargo clean
cargo build
cargo build -r
```

* Test

```
cargo test -- --nocapture
```

* Run

```
cargo run
cargo run -r
```

* Run with DEBUG

```
RUST_LOG=debug cargo run
```

* Run with details

```
RUST_BACKTRACE=1 cargo run
```

* Build Docker

```
docker build -t asa/dydx-bot .
docker run -p 8080:8080 asa/dydx-bot

curl -kv http://localhost:8080/health_check
```

* Deploy to DigitalOcean

```
git checkout release
git merge develop
git push

doctl apps create --spec spec.yaml
curl -v https://dydx-bot-hl5vk.ondigitalocean.app/health_check
```

## DYDX - Metamask

* Metamask > Goerli
* Open Chrome
* https://dydx.exchange/
* Trade > Testnet > Connect Wallet > Metamask > Remember me
* Chrome > View > Devtools > Application > Local Storage

```
https://trade.stage.dydx.exchange/
 - STARK_KEYS_PAIRS
 - API_KEYS_PAIRS
```

## DYDX Trade

* https://dydx.exchange/trade/ETH-USD

_Open Position_

* Buy
* Amount > 0.1
* Market > Place Market Order
* Portfolio > Positions

_Close Position_

* Sell
* Amount > 0.1
* Market > Place Market Order
* Portfolio > Positions

## Errors

* 'No module named web3'

```
PyErr { type: <class 'ModuleNotFoundError'>, value: ModuleNotFoundError("No module named 'web3'")
```

* Solution

```
pip install -r requirements.txt
pip install web3
```

* 'Library not loaded: @rpath/libpython3.11.dylib'

```
(base) sergei@MacBook-Pro web3 % ./target/debug/examples-dydx
dyld[83961]: Library not loaded: @rpath/libpython3.11.dylib
  Referenced from: <5E177495-D5E4-3233-9A93-12EFC16E767B> /Users/sergei/asa/src/devtools4me/web3/target/debug/examples-dydx
  Reason: tried: '/usr/local/lib/libpython3.11.dylib' (no such file), '/usr/lib/libpython3.11.dylib' (no such file, not in dyld cache)
zsh: abort      ./target/debug/examples-dydx
```

* Solution

```
sudo ln -s \
 /Users/sergei/anaconda3/lib/libpython3.11.dylib \
 /usr/local/lib/libpython3.11.dylib
```

## Python VENV

```
python3 -m venv venv
cp <some folder>/requirements.txt .
source venv/bin/activate
(venv) (base) sergei@MacBook-Pro dydx %
pip3 install -r requirements.txt
```