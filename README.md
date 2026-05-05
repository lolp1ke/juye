<!-- SPDX-License-Identifier: Apache-2.0 -->

<div align="center">
  <h1>juye</h1>
  <p>
    64-bit kernel assembled in rust for fun.
  </p>


  ![rust toolchain](https://img.shields.io/badge/rust-nightly-black.svg)
  ![architecture](https://img.shields.io/badge/arch-arm64-purple.svg)
  ![loc](https://img.shields.io/endpoint?url=https://ghloc.vercel.app/api/lolp1ke/juye/badge)
</div>

## getting started

> [!CAUTION]
> tested only on macos, no guarantee that kernel code will compile with little tweaks for linux let alone windows machines

### prerequisites

- nightly rust
```sh
 # development phase started by using this specific version, any later toolchain must be fine to use
rustup toolchain install nightly-2026-02-03
```

- qemu
```sh
# macos
brew install qemu
```

- make
```sh
# macos
brew install make
```

### running

```sh
git clone https://github.com/lolp1ke/juye
make run 
```

more available commands are in [makefile](Makefile).

## license
this codebase availble under [Apache-2.0](LICENSE-APACHE)
