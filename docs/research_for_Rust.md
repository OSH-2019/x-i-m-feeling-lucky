# 调研报告

<!-- TOC -->

- [调研报告](#调研报告)
    - [1. 项目简介](#1-项目简介)
    - [2. 项目背景](#2-项目背景)
        - [2.1 树莓派上的操作系统](#21-树莓派上的操作系统)
        - [2.2 Rust 编程语言与操作系统](#22-rust-编程语言与操作系统)
            - [2.2.1 Rust 语言简介](#221-rust-语言简介)
            - [2.2.2 Why is it Safe?](#222-why-is-it-safe)
                - [Memory safety](#memory-safety)
                - [Memory management](#memory-management)
                - [Ownership](#ownership)
            - [2.2.3 当前基于 Rust 的 OS 的对比分析](#223-当前基于-rust-的-os-的对比分析)
            - [2.2.4 Redox OS](#224-redox-os)
    - [3. 立项依据](#3-立项依据)
        - [3.1 项目名称](#31-项目名称)
        - [3.2 项目介绍](#32-项目介绍)
        - [3.3 项目依据与预期](#33-项目依据与预期)
    - [4. 前瞻性分析](#4-前瞻性分析)
        - [4.1 Rust is good for developing an OS](#41-rust-is-good-for-developing-an-os)
        - [4.2 Why this project?](#42-why-this-project)
    - [5. 参考资料](#5-参考资料)

<!-- /TOC -->

## 1. 项目简介

**Implementing an OS for Raspberry Pi 3 in Rust**

使用 Rust 编程语言写一个能在树莓派上运行的操作系统。

**硬件设备**

- 树莓派 3B+（OSH 课程统一发放）；

- Micro SD 卡 （OSH课程统一发放）；

- USB to TTL 转换器；

- 读卡器。

**学习资源**

- OSH 课程和教材；
- [CS140e 课程](<https://cs140e.sergio.bz/>)；
- [Writing an OS in Rust](https://os.phil-opp.com/second-edition/) By Philipp Oppermann；
- [Rust 官方文档](https://doc.rust-lang.org/stable/)；
- [rust-raspi3-OS-tutorials](<https://github.com/rust-embedded/rust-raspi3-OS-tutorials>) on Github.com；
- [CS140e](<https://www.reddit.com/r/cs140e/>) on Reddit.com。

**目标**

在树莓派上完成一个操作系统的基本组成（引导、文件系统、内存系统、进程管理等），先达成能上机跑起来的目标（连接显示器，开机后能够显示一个Shell，提供对常用命令的支持）。若仍有余力，将在某些部分（如进程调度、文件系统）上做进一步优化。**本项目重点不在做出什么新东西，做出什么比 [Redox](<https://www.redox-os.org/>) 更好的东西，而是试图通过“造轮子”的过程，将 OSH 课上学的东西用起来。**


## 2. 项目背景

### 2.1 树莓派上的操作系统

树莓派是一种小型、轻量级而功能齐全的嵌入式设备，由于其相对低廉的价格和优质的生态成为了良好的教学用设备和极客们大显身手的舞台。

树莓派的CPU基于ARM架构，运行在其上的操作系统需要针对这种架构和硬件配置做出一定的修改和优化。而许多常见的操作系统都有为树莓派ARM架构专门开发的版本。

其中的一些通用系统简介如下：

- **Raspbian**([homepage](http://www.raspbian.org/))

  树莓派的“官方“通用操作系统。它基于Debian(Linux的一个版本)而特别为树莓派开发。事实上它不仅仅是一个OS，其内部配置了相当多的应用软件，涵盖办公、开发、教育等各个方面。其兼容性和性能非常优秀，是当前树莓派上最实用最广泛的操作系统。

- **Ubuntu Mate**([homepage](https://ubuntu-mate.org/))

  Ubuntu Mate针对树莓派1和2的发行版本，界面个性美观。

- **Ubuntu Snappy Core**([homepage](https://www.ubuntu.com/core))

  Ubuntu针对物联网(IoT)的一个发行版本，兼容树莓派。主要面向开发者。

- **CentOS**([homepage](https://www.centos.org/))

  CentOS针对ARM的发行版，兼容树莓派。

- **Windows 10 IoT**([homepage](https://developer.microsoft.com/en-us/windows/iot))

  Windows针对物联网(IoT)的一个发行版本，兼容Windows 10的桌面风格，兼容树莓派。它不是完整的Windows版本，主要面向开发者。

- **FreeBSD**([homepage](https://www.freebsd.org/)) 

  FreeBSD针对树莓派的发行版。

- **Kali**([homepage](https://www.kali.org/))

  基于Debian的Kali操作系统针对树莓派的发行版。它内置了一系列安全性测试工具，可执行渗透测试，非常适合注重安全性测试的程序员和开发者。

- **Pidora**([homepage](http://www.pidora.ca/))

  在Fedora Remix 基础上针对树莓派优化过的操作系统。

- **Arch Linux**([homepage](https://archlinuxarm.org/))

  Arch Linux针对ARM的发行版，兼容树莓派。其特点为轻量级、使用简单、软件更新速度快。

- **RISC OS**([homepage](https://www.riscosopen.org))

  它基于RISC精简指令集，是相当简洁的操作系统。它比当今的绝大多数操作系统要简单，又具有良好的实时性，但仅支持单用户模式，且在安全性上没有太多保障，还有很大的提升空间。 

可以看出，绝大多数兼容树莓派的操作系统都是原版本针对ARM架构或树莓派本身的发行版，其本身的特性并没有太大的变化，也就是说原来操作系统所具有的弊端依旧会存在。而对于树莓派这类嵌入式设备来说，其安全性问题更甚。

现在的嵌入式操作系统总是能够推陈出新，更新换代速度很快，其功能也逐渐丰富起来。不过在很多情况下，其安全性受到了一定程度上的忽视。一方面是由于目前的嵌入式设备还处在发展中，没有成为相关攻击的主要目标；另一方面，当前的嵌入式系统采用的加密+身份验证模式对于维持其安全性已经足够。

不过随着嵌入式系统的发展，其在我们生活中与计算机的地位将会越来越高。彼时就一定得更加认真地去看待它相关的安全性问题。我们希望从现在就做一些相关的尝试，针对树莓派这样一种嵌入式设备本身使用Rust语言开发一个操作系统，从实现的层面上规避许多安全性的问题并对Rust的硬件进行一定程度上的优化工作。

### 2.2 Rust 编程语言与操作系统

#### 2.2.1 Rust 语言简介

Rust 是一个着重于安全性（特别是并发安全）的多重范型编程语言。Rust 在语法上和 C++ 类似，但是能够在保持高性能的同时提供更好的内存安全性。


<p align="center">
<img alt="Redox" width="180" src="research_for_Rust.assets/rust_logo.png">
</p>

Rust 由 Mozila Research 的 Graydon Hoare 设计，Dave Herman、Brendan Eich 亦有贡献。

Rust 在 Stack Overflow 的 [2016](https://stackoverflow.com/insights/survey/2016#technology-most-loved-dreaded-and-wanted)、[2017](https://stackoverflow.com/insights/survey/2017#most-loved-dreaded-and-wanted)、[2018](https://insights.stackoverflow.com/survey/2018/#most-loved-dreaded-and-wanted) 年开发者调查中，是“最被喜爱的编程语言”。

![1554027955391](research_for_Rust.assets/stat.png)

#### 2.2.2 Why is it Safe?

Rust 编程语言最大的亮点是安全性，正如其官网所说。

> Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — and enable you to eliminate many classes of bugs at compile-time.

Rust 的许多特性保证了它的安全性，现列举部分如下。

##### Memory safety

内存安全是 Rust 的一个设计目标，它不允许在 Safe Rust（和它相对的是 Unsafe Rust，Rust 默认运行方式是前者）中出现空指针、野指针和数据竞争。数据只能通过几种固定的方式初始化。Rust 没有空值`Null`，而是提供了`Option`枚举类型，它可以是`Some`或`None`两种状态之一，前者表示存在，后者表示不存在。Rust 放弃`Null`、使用`Option`的`None`表达不可用，这一选择确保了程序员必须对一个可能不可用的值做处理，否则 Rust 将会无法通过编译，这种“强制”措施保证了程序的安全性。

##### Memory management

Rust 既没有像 C、C++ 一样使用手工管理内存的方式，也没有像 Java、Go 、Python 等语言一样使用自动垃圾回收系统，而是通过加入`lifetime`和`ownship`特性，实现了内存的自动回收。

##### Ownership

Rust 中，所有的值都有一个 owner，值可以通过不可修改的引用（`&T`）、可修改的引用（`&mut T`）或所有权传递（`T`）来传递。这种`ownership`机制使得程序能够在编译的时候被 complier 检查，从而实现更安全的内存管理。



#### 2.2.3 当前基于 Rust 的 OS 的对比分析

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

#### 2.2.4 Redox OS

<p align="center">
<img alt="Redox" width="346" src="research_for_Rust.assets/redox_logo.png">
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

![img](research_for_Rust.assets/large.png)



Redox 不仅仅是个内核，而是个全功能的操作系统，它提供了内存分配器、文件系统、显示管理器等扩展，和内核本身共同构成了一个实用、便捷的操作系统生态。某种意义上可以把它理解成基于内存安全的编程语言的、加上一些现代技术的 GNU 或 BSD 生态。（译自<https://github.com/redox-os/redox>）

就在几天前（2019年3月24日），[Redox OS 0.5.0](https://www.redox-os.org/news/release-0.5.0/) 发布。

![1554029449257](research_for_Rust.assets/redox_new_version.png)

新的 Redox OS 将 Relibc 用做默认 C 语言库（Relibc 是一个用 Rust 编程语言编写的C语言库的实现）。Redox OS 0.5 还包括对其事件系统的改进、完成对 Pthreads 的支持、对 LLVM 和使用 LLVM 的项目（如Mesa 和 LLVMpipe）的更好支持、对 EFI 的改进等等。（引自 [Redox OS 0.5发布](https://www.linuxidc.com/Linux/2019-03/157707.htm)）




## 3. 立项依据

### 3.1 项目名称

**基于 Rust 语言开发可在树莓派机器上运行的操作系统**。


### 3.2 项目介绍


如名称所示，我们希望利用 Rust 在语法上内禀的安全性来尝试开发一个较为完整的操作系统，并使之可以在树莓派 3B+ 机器上正常运行。

总体上，我们的项目涉及到两个方面：

- 使用 Rust 语言进行操作系统的开发。操作系统有几大基本组成部分：进程调度、内存管理、文件系统、设备驱动等。这些方面都应当纳入我们的考虑当中。特别地，操作系统的方方面面都涉及到安全性的问题。而 Rust 不仅具有像 C++ 那样的执行时高效率，且其在语法上的严苛性能够保证在开发层面上避免许多的操作系统安全性漏洞，使用它作为开发的语言是一个比较好的选择；
- 在树莓派机器上运行操作系统，这涉及到对树莓派硬件的理解。为了使我们设计的操作系统能够在树莓派机器上运行，其还要针对机器的 ARM 架构和硬件驱动程序接口进行设计，这也是该项目的重难点之一。

我们会将目标操作系统的安全性与其在树莓派上的运行作为首要的关注点。


### 3.3 项目依据与预期


如背景中所介绍，有许多基于 ARM 架构的操作系统可以在树莓派上运行，也有一些着眼于 Rust 的安全性而实现的操作系统问世，不过目前能综合两者，即在树莓派这类嵌入式设备上运行的基于 Rust 的操作系统还很少。我们认为可以汲取这些开发者的经验和课本上学习的操作系统知识，将其用于我们的 Rust 的系统实现。

通过完成该项目，我们能够：

- 实现一个小的操作系统内核，针对其在树莓派机器上的运行做相关方面的性能测试，并与其它同类型系统进行比较；
- 在该操作系统内核上实现一些简单的系统调用，使其具有一般通用操作系统的基本功能；
- 活用所学习的知识，深入理解操作系统的概念与原理；
- 熟练掌握 Rust 语言的开发技术。


## 4. 前瞻性分析

### 4.1 Rust is good for developing an OS

某种意义上，Rust 是当前最适合开发操作系统的编程语言：C 和 C++ 有些古老，在内存安全性等方面欠佳，大多数现代编程语言又不能胜任编写底层代码，而 Rust 语言既能像 C 和 C++ 一样深入底层，其自身的特点又使得基于其的 OS 更安全。

当前在操作系统开发领域，C 和 C++ 历史悠久，其潜力几乎已经被开发完，但是 Rust 作为一种全新的更安全的编程语言，大大提高了操作系统开发的天花板。当前较为成熟的 Redox OS 已经证明了 Rust 开发操作系统的可行性。

### 4.2 Why this project?

这个项目看起来不那么新奇，而且还有很大的“造轮子”的嫌疑，但是它作为和课程密切相关的一个项目，一方面能够巩固运用我们在 OSH 课程上学习的知识，让我们对操作系统有一个更全面、深入的理解；另一方面，这个项目起到一个引子的作用：我们也许时间精力不足，做出来的东西简单、粗糙，用户交互只有一个 Shell ......但是有了这个项目的基础，日后继续开发出功能更完善、界面更好看的系统也许只是个时间问题。

## 5. 参考资料

- [Stanford CS140e - Operating Systems](https://cs140e.sergio.bz/)

- [Rust Documentation](https://doc.rust-lang.org/stable/)

- [Writing an OS in Rust](https://os.phil-opp.com/)

- [rust-embedded/rust-raspi3-OS-tutorials: Rust bare-metal and OS tutorials on the Raspberry Pi 3](https://github.com/rust-embedded/rust-raspi3-OS-tutorials)

- [Stanford: An experimental course on operating systems](https://www.reddit.com/r/cs140e/)

