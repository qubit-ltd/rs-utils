# rs-utils

[![Rust CI](https://github.com/qubit-ltd/rs-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-utils/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-utils/coverage-badge.json)](https://qubit-ltd.github.io/rs-utils/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-utils.svg?color=blue)](https://crates.io/crates/qubit-utils)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

为 Rust 项目提供通用工具。本仓库用于沉淀 Qubit 软件中可复用的小型辅助能力。

## 目标用户

需要在多个 Qubit 项目之间共享通用工具的 Rust 开发者。

## 安装

在 `Cargo.toml` 中添加已发布的 crate：

```toml
[dependencies]
qubit-utils = "0.2"
```

## 快速开始

在执行有意绕过边界检查的 slice 访问前，先使用带检查的范围计算：

```rust
use qubit_utils::{nonzero, SliceRange, UncheckedSlice};

let required = nonzero(2);
let input = [0x10_u8, 0x20, 0x30];
let end = SliceRange::checked_range_end(input.len(), 1, required.get(), "range exceeds input")
    .expect("validated range should fit");

// SAFETY: `end` 证明请求的范围位于 `input` 内。
let value = unsafe { UncheckedSlice::read(&input, end - 1) };
assert_eq!(value, 0x20);
```

## 当前状态

0.2.0 版本已提供多个可复用的通用 API：

- 可恢复分配接口：`create_vec`、`try_reserve_vec`、`try_reserve_string`、
  `allocation_error`，以及 `coverage` 特性下的测试辅助接口；
- `nonzero`：用于构造 `NonZeroUsize` 的公开函数；
- `Transient<T>`：用于表示“运行时临时状态”的包装类型（不参与 `Eq`/`Hash`）；
- `UncheckedSlice` 与 `SliceRange`：低层 slice 范围与无边界检查操作工具。

## 能力

提供依赖非常轻量、可直接复用的通用基础能力，持续服务于 Qubit 的 Rust
生态项目。

## 限制

当前公开 API 刻意保持精简，侧重底层公共能力，后续功能会按使用场景逐步补充。

## 测试

```bash
# 使用默认 feature 集运行测试
cargo test

# 使用项目声明的全部 feature 运行测试
cargo test --all-features

# 运行项目 CI 检查
./ci-check.sh

# 检查代码覆盖率
./coverage.sh
```

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

本项目基于 Apache License 2.0 授权。完整许可证文本请参阅
[LICENSE](LICENSE)。

## 贡献

欢迎贡献。请遵循 Rust API 指南，及时更新公共 API 文档与测试，并在提交
Pull Request 前运行 `./align-ci.sh` 格式化代码，运行 `./ci-check.sh` 对齐 CI 要求。

## 作者

**Haixing Hu** - *Qubit Co. Ltd.*

仓库地址：[https://github.com/qubit-ltd/rs-utils](https://github.com/qubit-ltd/rs-utils)
