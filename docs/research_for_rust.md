# OS on Raspberry Pi with Rust

## 小组成员

- 刘云飞 PB17051044
- 李维晟 PB17000100
- 汪若辰 PB17000098
- 余磊 PB17051053

## 项目简介

使用 Rust 编程语言写一个能在树莓派上运行的操作系统。

## 项目背景

### 树莓派上的操作系统







### Rust 编程语言与操作系统

#### Rust 语言简介

Rust 是一个着重于安全性（特别是并发安全）的多重范型编程语言。Rust 在语法上和 C++ 类似，但是能够在保持高性能的同时提供更好的内存安全性。

![Rust programming language black logo.svg](research_for_rust.assets/144px-Rust_programming_language_black_logo.svg.png)

Rust 由 Mozila Research 的 Graydon Hoare 设计，Dave Herman、Brendan Eich 亦有贡献。

Rust 在 Stack Overflow 的 [2016](https://stackoverflow.com/insights/survey/2016#technology-most-loved-dreaded-and-wanted)、[2017](https://stackoverflow.com/insights/survey/2017#most-loved-dreaded-and-wanted)、[2018](https://insights.stackoverflow.com/survey/2018/#most-loved-dreaded-and-wanted) 年开发者调查中，是“最被喜爱的编程语言”。

![1554027955391](research_for_rust.assets/1554027955391.png)

#### Why is it Safe?

Rust 编程语言最大的亮点是安全性，正如其官网所说。

> Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — and enable you to eliminate many classes of bugs at compile-time.

Rust 的以下特性保证了它的安全。

##### Memory safety

内存安全是 Rust 的一个设计目标，它不允许在 Safe Rust（和它相对的是 Unsafe Rust，Rust 默认运行方式是前者）中出现空指针、野指针和数据竞争。

#### 当前基于 Rust 的 OS 的对比分析

当前基于 Rust 的操作系统主要有以下这些。（数据来源：[flosse/rust-os-comparison: A comparison of operating systems written in Rust](https://github.com/flosse/rust-os-comparison)）

- **redox**             ([repository](https://github.com/redox-os/redox) / [homepage](http://www.redox-os.org/))

- **Tock**              ([repository](https://github.com/helena-project/tock) / [homepage](http://www.tockos.org/))

- **intermezzOS**       ([repository](https://github.com/intermezzos/kernel) / [homepage](http://intermezzos.github.io/))

- **reenix**            ([repository](https://github.com/scialex/reenix))

- **rustboot**          ([repository](https://github.com/charliesome/rustboot))

- **RustOS**            ([repository](https://github.com/ryanra/RustOS))

- **QuiltOS**           ([repository](https://github.com/QuiltOS/QuiltOS))

- **Tifflin (rust_os)** ([repository](https://github.com/thepowersgang/rust_os))

- **bkernel**           ([repository](https://github.com/rasendubi/bkernel))

- **Quasar**            ([repository](https://github.com/LeoTestard/Quasar))

- **SOS**               ([repository](https://github.com/hawkw/sos-kernel))

这些操作系统的对比如下表。


| Name            | Architectures   | Pure Rust                | Active? | Kernel architecture        | Target           | Userpace? | Optional GUI? | Contributors | Filesystem  | License                    |
|-----------------|-----------------|--------------------------|---------|----------------------------|------------------|-----------|---------------|--------------|-------------|----------------------------|
| **redox**       | x86 and x86_64  | yes                      | yes     | Microkernel                | General purpose  | yes       | yes           | 50           | [ZFS]/[RedoxFS] | MIT                        |
| **Tock**        | Cortex M        |                          | yes     |                            |                  |           | no            | 40           |             | APL 2 / MIT                |
| **intermezzOS** | x86_64          | no                       | yes     | ?                          | PoC              | no        | no            | 18           | no          | APL 2 / MIT                |
| **RustOS**      | i386            | ?                        | yes     | None                       | PoC              | no        | no            | 10           | no          | APL 2 / MIT                |
| **rustboot**    | i386            | ?                        | no      | None                       | PoC              | no        | no            | 8            | no          | MIT                        |
| **bkernel**     | ARM             | yes                      | yes     | ?                          | Embedded devices | no        | no            | 4            | ?           | GPL with linking exception |
| **SOS**         | x86_64          | yes                      | yes     | Microkernel                | PoC              | no        | no            | 3            | ?           | MIT                        |
| **reenix**      | [Brown's CS167/9] | no                       | no      | Monolithic (current state) | PoC              | no        | no            | 3            | ?           | [unknown]                  |
| **Quasar**      | x86_64          | ?                        | no      | ?                          | ?                | no        | no            | 2            | ?           | ?                          |
| **Tifflin**     | x86_64/amd64    | almost                   | yes     | Monolithic                 | ?                | ?         | yes           | 1            | ISO9660     | 2-Clause-BSD               |

#### Redox OS

<p align="center">
<img alt="Redox" width="346" src="research_for_rust.assets/68747470733a2f2f6769746c61622e7265646f782d6f732e6f72672f7265646f782d6f732f6173736574732f7261772f6d61737465722f6c6f676f732f7265646f782f6c6f676f2e706e67.png">
</p>

在众多基于 Rust 的操作系统中，Redox OS 当之无愧是目前最成熟的操作系统之一，基于此，我们选择它做简要介绍。

在 [Redox OS 官网](https://www.redox-os.org/)上，开发者对其的介绍如下。

> **Redox** is a Unix-like Operating System written in [**Rust**](https://www.rust-lang.org/), aiming to bring the innovations of Rust to a modern microkernel and full set of applications.

Redox 有以下特点。

- Rust 语言实现
- 微内核设计
- 包括可选的 GUI 程序 - Orbital
- 支持 Rust 标准库
- MIT 授权
- 驱动运行在用户空间
- 包括常见的 Unix 命令
- C 程序的新移植库

Redox 的桌面环境 Orbital 也有着成熟、现代的 UI 设计，截图如下。

![img](research_for_rust.assets/large.png)



Redox 不仅仅是个内核，而是个全功能的操作系统，它提供了内存分配器、文件系统、显示管理器等扩展，和内核本身共同构成了一个实用、便捷的操作系统生态。某种意义上可以把它理解成基于内存安全的编程语言的、加上一些现代技术的 GNU 或 BSD 生态。（译自<https://github.com/redox-os/redox>）

就在几天前（2019年3月24日），[Redox OS 0.5.0](https://www.redox-os.org/news/release-0.5.0/) 发布。

![1554029449257](research_for_rust.assets/1554029449257.png)

新的 Redox OS 将 Relibc 用做默认 C 语言库（Relibc 是一个用 Rust 编程语言编写的C语言库的实现）。Redox OS 0.5 还包括对其事件系统的改进、完成对 Pthreads 的支持、对 LLVM 和使用 LLVM 的项目（如Mesa 和 LLVMpipe）的更好支持、对 EFI 的改进等等。（引自 [Redox OS 0.5发布](https://www.linuxidc.com/Linux/2019-03/157707.htm)）




## 立项依据





## 前瞻性分析

### 安全性

Rust 语言自身的特点使得基于其的 OS 更安全。





## 参考资料

- [Stanford CS140e - Operating Systems](https://cs140e.sergio.bz/)

- [Rust Documentation](https://doc.rust-lang.org/stable/)

- [Writing an OS in Rust](https://os.phil-opp.com/)

- [rust-embedded/rust-raspi3-OS-tutorials: Rust bare-metal and OS tutorials on the Raspberry Pi 3](https://github.com/rust-embedded/rust-raspi3-OS-tutorials)

- [Stanford: An experimental course on operating systems](https://www.reddit.com/r/cs140e/)