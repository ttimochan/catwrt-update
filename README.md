# catwrt-update

CatWrt 的更新检测支持，即 /usr/bin/catwrt-update 的功能。

## 介绍

Catwrt-Update is a tool to check the update of CatWrt, coding with rust.

## 编译

安装 Rust 环境

```bash

curl --proto '=https' --tlsv1.2  https://sh.rustup.rs  -sSf | sh
```

然后设置 target 为你想要的平台，例如：

```bash
rustup target add mipsel-unknown-linux-musl
```

然后编译：

```bash
cargo build --release --target mipsel-unknown-linux-musl
```

目前支持的平台有：

- mipsel-unknown-linux-musl
- mips-unknown-linux-musl
- arm-unknown-linux-musl
- aarch64-unknown-linux-musl
- x86_64-unknown-linux-musl
- i686-unknown-linux-musl
- x86_64-unknown-linux-gnu
- i686-unknown-linux-gnu

然后，你就在 target 目录下找到你的可执行文件了。
