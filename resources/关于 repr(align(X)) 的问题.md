### 问题描述

在`ttywrite`目录下执行`test.sh`命令的时候时，报错如下：

``` bash
➜  ttywrite git:(master) ✗ ./test.sh
Compiling project with 'cargo build'...
   Compiling quote v0.3.15
   Compiling xmodem v0.1.0 (file:///media/yu/Windows/Users/Yu/Documents/GitHub/Rust/CS140e/1-shell/xmodem)
   Compiling bitflags v1.0.4
   Compiling ansi_term v0.11.0
   Compiling libc v0.2.51
   Compiling strsim v0.8.0
   Compiling unicode-xid v0.0.4
   Compiling vec_map v0.8.1
   Compiling unicode-width v0.1.5
   Compiling synom v0.11.3
   Compiling textwrap v0.11.0
   Compiling syn v0.11.11
error: non-string literals in attributes, or string literals in top-level positions, are experimental (see issue #34981)
 --> /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/notbsd/linux/other/align.rs:6:16
  |
6 |                repr(align(8)))]
  |                ^^^^^^^^^^^^^^^
  |
  = help: add #![feature(attr_literals)] to the crate attributes to enable

error: non-string literals in attributes, or string literals in top-level positions, are experimental (see issue #34981)
    --> /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/notbsd/linux/align.rs:12:24
     |
12   |                        repr(align(4)))]
     |                        ^^^^^^^^^^^^^^^
     | 
    ::: /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/notbsd/linux/mod.rs
     |
2470 | expand_align!();
     | ---------------- in this macro invocation
     |
     = help: add #![feature(attr_literals)] to the crate attributes to enable

error: non-string literals in attributes, or string literals in top-level positions, are experimental (see issue #34981)
    --> /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/notbsd/linux/align.rs:31:24
     |
31   |                        repr(align(8)))]
     |                        ^^^^^^^^^^^^^^^
     | 
    ::: /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/notbsd/linux/mod.rs
     |
2470 | expand_align!();
     | ---------------- in this macro invocation
     |
     = help: add #![feature(attr_literals)] to the crate attributes to enable

error: non-string literals in attributes, or string literals in top-level positions, are experimental (see issue #34981)
    --> /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/macros.rs:77:15
     |
77   |             $(#[$attr])*
     |               ^^^^^^^^
     | 
    ::: /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/notbsd/linux/mod.rs
     |
2470 | expand_align!();
     | ---------------- in this macro invocation
     |
     = help: add #![feature(attr_literals)] to the crate attributes to enable

error: non-string literals in attributes, or string literals in top-level positions, are experimental (see issue #34981)
    --> /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/notbsd/linux/align.rs:56:24
     |
56   |                        repr(align(8)))]
     |                        ^^^^^^^^^^^^^^^
     | 
    ::: /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/notbsd/linux/mod.rs
     |
2470 | expand_align!();
     | ---------------- in this macro invocation
     |
     = help: add #![feature(attr_literals)] to the crate attributes to enable

error: non-string literals in attributes, or string literals in top-level positions, are experimental (see issue #34981)
    --> /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/notbsd/linux/align.rs:75:24
     |
75   |                        repr(align(8)))]
     |                        ^^^^^^^^^^^^^^^
     | 
    ::: /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/notbsd/linux/mod.rs
     |
2470 | expand_align!();
     | ---------------- in this macro invocation
     |
     = help: add #![feature(attr_literals)] to the crate attributes to enable

error: non-string literals in attributes, or string literals in top-level positions, are experimental (see issue #34981)
    --> /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/notbsd/linux/align.rs:94:24
     |
94   |                        repr(align(8)))]
     |                        ^^^^^^^^^^^^^^^
     | 
    ::: /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/notbsd/linux/mod.rs
     |
2470 | expand_align!();
     | ---------------- in this macro invocation
     |
     = help: add #![feature(attr_literals)] to the crate attributes to enable

error: non-string literals in attributes, or string literals in top-level positions, are experimental (see issue #34981)
  --> /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/macros.rs:77:15
   |
77 |               $(#[$attr])*
   |                 ^^^^^^^^
   | 
  ::: /home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/align.rs
   |
1  | / s! {
2  | |     #[repr(align(4))]
3  | |     pub struct in6_addr {
4  | |         pub s6_addr: [u8; 16],
5  | |     }
6  | | }
   | |_- in this macro invocation
   |
   = help: add #![feature(attr_literals)] to the crate attributes to enable

error: aborting due to 8 previous errors

error: Could not compile `libc`.
warning: build failed, waiting for other jobs to finish...
error: build failed
ERROR: ttywrite compilation failed
Opening PTYs...
Running test 1/10.
./test.sh: line 48: ./target/debug/ttywrite: No such file or directory
ERROR: input and output differ
4JLvbOK8sJ+6hz2PvZDJp1wvdEZy+MzdrosIz5g5UghBBwtO4zYujhjrNmSiHjkf+HHf+tfP4NpY != 
```

### 解决

从报错信息中可以大致猜测，`repr(align(8))`等语句使用了某种实验性功能，却没有在相关文件中添加`#![feature(attr_literals)]`信息，因此报错。

解决这个问题，最关键的地方是找到在哪个文件中添加这么一行，经过搜索和试验，这个文件是报错信息里的`src`目录里面的`lib.rs`文件。

如，我的报错信息里面，出错的文件有`/home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/unix/notbsd/linux/align.rs`, `/home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/macros.rs`等，因此我要添加`#![feature(attr_literals)]`信息的文件在`/home/yu/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.51/src/lib.rs`。