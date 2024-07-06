# `fnaop`

`fnaop` is a lightweight and flexible Rust library designed to bring Aspect-Oriented Programming (`AOP`) to your Rust
functions. By using `fnaop`, you can easily add `pre` and `post` function logic without modifying the core functionality
of your functions, enabling cleaner and more maintainable code.

[APIs Documents](https://docs.rs/fnaop)

[changelog](./CHANGELOG.md)

## 1.`Usage`

Add this to your `Cargo.toml`:

```toml
[dependencies]
fnaop = "0.1"
```

## 2.`APIs`

`Aspect`:
An attribute macro for applying Aspect-Oriented Programming (`AOP`) to functions. The `Aspect` macro allows you to
specify `before` and `after` functions that will be called before and after the target function respectively. This is
useful for encapsulating cross-cutting concerns such as logging, metrics, or other side effects.

### 2.1.`Noarmal`

Ordinary function, adapted to 0 or more ordinary parameters.

```rust
#[Aspect(before = "before_fn")]
pub fn say_hello(x: i64) {
    println!("Hello:say_hello, {}", x);
}

#[Aspect(after = "after_fn")]
pub fn say_hello(x: i64) {
    println!("Hello:say_hello, {}", x);
}

#[Aspect(before = "before_fn", after = "after_fn")]
pub fn say_hello(x: i64) {
    println!("Hello:say_hello, {}", x);
}

// ----------------------------------------------------------------

#[Aspect(before = "before_fn_empty", after = "before_fn_empty")]
pub fn say_hello_empty() {
    println!("Hello:say_hello_empty");
}
```

### 2.2.`Lifetime`

Functions with lifetime parameters.

```rust
#[Aspect(before = "before::struct_before_fn_lifetime", after = "after::struct_after_fn_lifetime")]
pub fn say_hello_struct_lifetime<'a>(ctx: LifetimeHelloContext<'a>) {
    println!("struct::Hello, {} {}", ctx.x, ctx.y);
}
```

### 2.3.`Return value`

Functions with return values.

```rust
#[Aspect(before = "before::struct_before_fn_lifetime", after = "after::struct_after_fn_lifetime")]
pub fn say_hello_struct_lifetime_with_return<'a>(ctx: LifetimeHelloContext<'a>) -> i64 {
    println!("struct::Hello, {} {}", ctx.x, ctx.y);

    *ctx.x
}
```

### 2.4.`Others`

…

```rust
// @see integration_tests.rs
```

