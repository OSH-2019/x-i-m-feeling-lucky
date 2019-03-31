# OS on Raspberry Pi with Rust

## 小组成员

- 刘云飞 PB17051044
- 李维晟 PB17000100
- 汪若辰 PB17000098
- 余磊 PB17051053

## 项目简介

使用 Rust 编程语言写一个能在树莓派上跑的操作系统。

## 项目背景

### 树莓派上的操作系统



### Rust 编程语言与操作系统

#### Rust 语言简介

Rust 编程语言最大的亮点是安全性，正如其官网所说。

> Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — and enable you to eliminate many classes of bugs at compile-time.

#### Rust 在操作系统上的应用



#### 当前基于 Rust 的 OS 的对比分析

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