## 👋 rust-gomoku
[![Crates.io](https://img.shields.io/crates/v/rust-gomoku.svg)](https://crates.io/crates/rust-gomoku)
<a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/license-MIT-purple.svg" alt="License: MIT"></a><br>
A console and web-based Gomoku written in Rust and WebAssembly

<p align="center">
  <img src="https://user-images.githubusercontent.com/53862866/133866229-ca90be8f-4fb0-4be2-a252-1d27656bc634.gif" height="450px">
</p>

## Getting started with cargo & npm

Install required program, run
```bash
# install cargo & wasm-pack
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# install npm
curl -sL https://deb.nodesource.com/setup_12.x | sudo -E bash -
sudo apt-get install nodejs -y
```
Go project root, run
```bash
cd rust-gomoku
```

For a system with Webassembly, run
```bash
wasm-pack build
```

Go web dir, run
```bash
cd ./web
```

Install package.json file's dependency, run
```bash
npm install
```

And enjoy 😀, run
```bash
npm run start
```

## Features and bugs
Please send feature requests and bugs at the [issue tracker](https://github.com/namkyu1999/rust-gomoku/issues)
