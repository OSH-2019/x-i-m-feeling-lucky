# 关于 Rust 版本的问题

使用最新 nightly 的 rust 在 os/kernel 下运行`make`命令时，会由于 rust 编译器的版本问题报错，包括但不限于找不到 std_unicode 这个 crate。

问题在于，rust 经过一年多的更新迭代，其中 crate 等内容已经有很多变化。

两种解决版本。

## 回退到 nightly-1.28.0 并使用 patch


### 回退 rust 版本

``` bash
rustup default nightly-2018-06-27
rustup component add rust-src
rustup toolchain list
rustc --version
```
### 使用 patch

``` bash
git clone https://cs140e.sergio.bz/os.git
mv patch_for_os_git.patch os
cd os
git apply patch_for_os_git.patch
```

## 回退到 nightly-1.25.0

和 CS140e 中所用版本号完全一样。

有个玄学问题，要先在最新版的 nightly 里面安装旧版本的 xargo :

```bash
cargo install --version 0.3.10 xargo
```

之后回到课程中的版本。

``` bash
rustup default nightly-2018-01-09
rustup component add rust-src
```

如果直接安装 night-2018-01-09，再安装 xargo，会有玄学的无法安装的问题。

在 os/kernel 里面执行：

```
make
```

编译成功，结果如下。

``` bash
➜  kernel git:(master) ✗ make
+ Building build/init.o [as ext/init.S]
+ Building target/aarch64-none-elf/release/libkernel.a [xargo --release]
   Compiling core v0.0.0 (file:///home/yu/.rustup/toolchains/nightly-2018-01-09-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore)
    Finished release [optimized] target(s) in 12.78 secs
   Compiling std_unicode v0.0.0 (file:///home/yu/.rustup/toolchains/nightly-2018-01-09-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd_unicode)
    Finished release [optimized] target(s) in 0.70 secs
   Compiling compiler_builtins v0.1.0 (file:///home/yu/.rustup/toolchains/nightly-2018-01-09-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcompiler_builtins)
    Finished release [optimized] target(s) in 1.75 secs
   Compiling std v0.1.0 (file:///home/yu/Windows/Users/Yu/Documents/GitHub/Rust/CS140e/os/std)
    Finished release [optimized] target(s) in 0.59 secs
   Compiling kernel v0.1.0 (file:///home/yu/Windows/Users/Yu/Documents/GitHub/Rust/CS140e/os/kernel)
   Compiling volatile v0.1.0 (file:///home/yu/Windows/Users/Yu/Documents/GitHub/Rust/CS140e/os/volatile)
   Compiling stack-vec v0.1.0 (file:///home/yu/Windows/Users/Yu/Documents/GitHub/Rust/CS140e/1-shell/stack-vec)
   Compiling pi v0.1.0 (file:///home/yu/Windows/Users/Yu/Documents/GitHub/Rust/CS140e/os/pi)


[A lot of warning...]

    Finished release [optimized] target(s) in 0.72 secs
+ Building build/kernel.elf [ld build/init.o build/kernel.a]
+ Building build/kernel.hex [objcopy build/kernel.elf]
+ Building build/kernel.bin [objcopy build/kernel.elf]
```

