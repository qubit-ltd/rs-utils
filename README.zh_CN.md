# rs-utils

[![Rust CI](https://github.com/qubit-ltd/rs-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-utils/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-utils/coverage-badge.json)](https://qubit-ltd.github.io/rs-utils/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-utils.svg?color=blue)](https://crates.io/crates/qubit-utils)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

为 Rust 项目提供通用、轻依赖的基础工具。该 crate 统一承载 Qubit 项目反复需要的
标准区间运算、可恢复内存分配、运行时临时值和经过审查的底层 slice 访问能力。

## 目标用户

适合需要在多个 Qubit 项目中复用小型基础能力，同时不希望引入大型框架或重复实现
边界敏感逻辑的 Rust 开发者。

## 安装

在 `Cargo.toml` 中添加已发布的 crate：

```toml
[dependencies]
qubit-utils = "0.3"
```

## 快速开始

假设某服务收到一个数值区间请求，但实际处理范围不能越过策略允许的窗口。
区间工具可以直接接收 Rust 标准区间，并返回持有所有权的标准库边界：

```rust
use std::ops::Bound::Excluded;
use std::ops::Bound::Included;

use qubit_utils::range::encloses;
use qubit_utils::range::intersection;

let policy = 1..10;
let requested = 5..=20;
let accepted = intersection(&policy, &requested);

assert_eq!(accepted, Some((Included(5), Excluded(10))));
assert!(encloses(&policy, &accepted.expect("the windows overlap")));
```

返回值是 `(Bound<T>, Bound<T>)`，它已经实现 `RangeBounds<T>`，可以直接用于
标准库有序集合的范围查询。

## 为什么需要这个项目

边界相关代码很容易出现“多数场景正确、极端情况错误”的实现，而且分散在多个
crate 后维护成本会持续上升。`qubit-utils` 集中维护 Qubit 项目反复使用的小型契约，
同时优先保留 Rust 标准表示，并控制默认依赖规模。

## 当前状态

0.3.0 版本提供以下可复用 API：

- 标准区间的关系判断与代数运算：`is_empty`、`encloses`、`overlaps`、
  `is_connected`、`intersection`、`span`、`gap` 和 `compare`；
- 可恢复分配接口：`create_vec`、`try_reserve_vec`、`try_reserve_string`、
  `allocation_error`，以及 `cfg(coverage)` 配置下的测试辅助接口；
- `nonzero`：用于构造 `NonZeroUsize` 的公开函数；
- `Transient<T>`：用于表示“运行时临时状态”的包装类型（不参与 `Eq`/`Hash`）；
- `UncheckedSlice` 与 `SliceRange`：低层 slice 范围与无边界检查操作工具。

## 能力

`range` 模块直接使用 `std::ops::RangeBounds`，既接受普通 Rust 区间语法，也接受
任意 `(Bound<T>, Bound<T>)` 边界组合。它以稳定的 free function 补充当前稳定版
标准库尚未完整提供的区间关系和运算。

其余模块提供内存、数值、slice 和运行时状态工具。常用类型与函数仍保留原有的
crate 根重导出路径。

## 限制

除 `is_empty` 外，区间代数要求端点实现全序 `Ord`。浮点数包含 NaN，需要先通过
明确排序策略的包装类型使用。空区间判断采用端点顺序语义，不会针对离散域做规范化；
因此整数开区间 `(1, 2)` 不会仅因为 1 和 2 之间没有整数而被判为空。

本 crate 不定义自有区间类型、动态比较器、离散域步进，也不内置日期或金额等业务
区间。后续能力只在具备跨项目复用价值时增加。

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
Pull Request 前运行 `./align-ci.sh`格式化代码，运行`./ci-check.sh`对齐CI要求。

## 作者

**Haixing Hu** - *Qubit Co. Ltd.*

仓库地址：[https://github.com/qubit-ltd/rs-utils](https://github.com/qubit-ltd/rs-utils)
