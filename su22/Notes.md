# 第一讲 基本语法

## 1 Rust 语言介绍

## 2 第一次接触

## 3 基本语法

## 4 小结

# 第二讲

# 第三讲

# 第四讲

# 第五讲 项目管理与常用库

## 1 项目管理

### Rust的模块系统

- 包(crates)
- 箱(crates)
- 模块(modules)
- 路径(paths)

### 模块

- Rust中所有项目的作用域都是基于模块的。
  	- 如果不是pub， 只能在同一模块中访问
  	- 如果是pub，可以在其他模块中访问
- 可以在同一个源代码文件中定义多个模块，可以在模块中定义模块。

#### 通过use使用模块

#### Cargo

- 一个包里可以同时包含库箱 (lib crate) 和二进制（可执行文件）箱 (bin crate)。
- 一般来说，库箱会包含导出项目，而二进制箱不会。
- （以下路径均相对于包所在的目录）
- Cargo 允许一个包里同时出现 src/lib.rs 和 src/main.rs。
  - Cargo 还会构建 src/bin/*.rs，作为可执行文件。*
- *示例代码放在 examples/*.rs。
  - 会在 cargo test 的时候构建，保证示例代码可以通过编译。
  - 可以用 cargo run --example foo 来运行。
- 集成测试（非单元测试）放在 tests/*.rs，用来测试正确性。

- 基准测试程序 (benchmarks) 放在 benches/*.rs，用来测试性能。

## 2 语法补充

## 3 智能指针

# 第六讲

## 1 闭包

移动闭包

移动闭包的应用场景

- 在闭包f需要比创建它的作用域活得更久时，需要用到移动闭包。

   - 

  ``` rust
  fn make_closure(x: i32) -> Box<dyn Fn(i32) -> i32> {
    
  ```

返回闭包

- 有时需要将闭包作为函数的返回值
- 但闭包是特型对象（trait），大小是不确定的

用`Box<dyn Fn(A) -> B`

``` Rust
fn box_up_your_closure_and_move_out() -> Box<dyn Fn(i32) -> i32> {
  let local = 2;
  Box::new(move |x| x * local)
}
```

## 2 并发的概念

- 什么是并发？

  ​	并发（concurrency）是指程序同时有多个正在运行的线程(threads)

- 线程之间可以共享数据，而不引入通信的开销

- 线程比单独的进程要轻量级

- 并发(concurrency)VS并行(parallelism)

### 死锁：

死锁发生的四个条件：

## 3 线程

## 4 共享线程状态

## 5 通道

## 6 小结

# 第七讲

