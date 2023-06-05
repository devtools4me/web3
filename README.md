# WEB3

[WEB3 Trader](web3-trader/README.md)

## Build

* cargo-expand

```
cargo install cargo-expand
cargo expand
```

* rustup

```
> rustup toolchain list
stable-aarch64-apple-darwin (default)

> rustup toolchain install nightly-aarch64-apple-darwin
...
nightly-aarch64-apple-darwin installed - rustc 1.71.0-nightly (ce5919fce 2023-05-15)

> rustup toolchain list
stable-aarch64-apple-darwin (default)
nightly-aarch64-apple-darwin
```

## Deployment

* doctl

```
brew install doctl
doctl auth init
doctl apps create --spec spec.yaml
```

## Hardhat

* initial

```
mkdir hardhat
cd hardhat
npm install --save-dev hardhat
npx hardhat
npx hardhat test
npx hardhat compile
```

* fork ETH main net

```
cd hardhat
npx hardhat node --fork https://eth-mainnet.g.alchemy.com/v2/TtK-PVc3lbV2nb7V_qUwTUALYEEBAySG
```

* fork BNB Smart Chain

```
npx hardhat node --fork https://bsc-dataseed.binance.org/
```