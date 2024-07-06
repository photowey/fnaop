/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// ----------------------------------------------------------------

use fnaop::Aspect;

use crate::common::models::{HelloContext, LifetimeHelloContext};

#[cfg(test)]
mod common;

// ----------------------------------------------------------------

// ----------------------------------------------------------------

#[test]
fn test_aspect_say_hello() {
    say_hello(42);
}

#[test]
fn test_aspect_mod_say_hello() {
    mod_say_hello(9527, 8848.0);
    mod_say_hello_empty();
}

// ----------------------------------------------------------------

#[test]
fn test_aspect_say_hello_with_struct_parameter() {
    let ctx = HelloContext {
        x: 9527,
        y: 8848.01,
    };
    say_hello_with_struct_parameter(ctx.clone());
    say_hello_with_struct_parameter_x(ctx.clone());
}

#[test]
fn test_aspect_say_hello_struct_lifetime() {
    let x = 9527;
    let y = 8848.01;

    let ctx = LifetimeHelloContext { x: &x, y: &y };

    say_hello_struct_lifetime(ctx);
}

#[test]
fn test_aspect_say_hello_struct_lifetime_with_return() {
    let x = 9527;
    let y = 8848.01;

    let ctx = LifetimeHelloContext { x: &x, y: &y };
    let rvt = say_hello_struct_lifetime_with_return(ctx);

    assert_eq!(9527, rvt)
}

// ---------------------------------------------------------------- simple

#[Aspect(before = "before_fn", after = "after_fn")]
pub fn say_hello(x: i64) {
    println!("Hello:say_hello, {}", x);
}

#[Aspect(before = "before_fn_empty", after = "before_fn_empty")]
pub fn say_hello_empty() {
    println!("Hello:say_hello_empty");
}

// ---------------------------------------------------------------- other.mod

#[Aspect(before = "before::before_fn", after = "after::after_fn")]
pub fn mod_say_hello(x: i64, y: f64) {
    println!("mod:Hello, {}", x);
}

#[Aspect(before = "before::empty_before_fn", after = "after::empty_after_fn")]
pub fn mod_say_hello_empty() {
    println!("empty:Hello");
}

// ---------------------------------------------------------------- struct.parameter

#[Aspect(before = "before::struct_before_fn", after = "after::struct_after_fn")]
pub fn say_hello_with_struct_parameter(ctx: HelloContext) {
    println!("struct::Hello, {} {}", ctx.x, ctx.y);
}

#[Aspect(before = "before::struct_before_fn", after = "after::struct_after_fn")]
pub fn say_hello_with_struct_parameter_x(ctx: HelloContext) {
    println!("struct::Hello, {} {}", ctx.x, ctx.y);
}

// ---------------------------------------------------------------- struct.parameter & lifetime

#[Aspect(
    before = "before::struct_before_fn_lifetime",
    after = "after::struct_after_fn_lifetime"
)]
pub fn say_hello_struct_lifetime<'a>(ctx: LifetimeHelloContext<'a>) {
    println!("struct::Hello, {} {}", ctx.x, ctx.y);
}

#[Aspect(
    before = "before::struct_before_fn_lifetime",
    after = "after::struct_after_fn_lifetime"
)]
pub fn say_hello_struct_lifetime_with_return<'a>(ctx: LifetimeHelloContext<'a>) -> i64 {
    println!("struct::Hello, {} {}", ctx.x, ctx.y);

    *ctx.x
}

// ---------------------------------------------------------------- aop.functions

fn before_fn(x: &i64) {
    println!("before_fn::Before: {}", x);
}

fn after_fn(x: &i64) {
    println!("before_fn::After: {}", x);
}

#[allow(dead_code)]
fn before_fn_empty() {
    println!("before_fn_empty::Before");
}

#[allow(dead_code)]
fn after_fn_empty() {
    println!("before_fn_empty::After");
}

// ----------------------------------------------------------------

mod before {
    use crate::{HelloContext, LifetimeHelloContext};

    pub fn before_fn(x: &i64, y: &f64) {
        println!("before::before_fn::Before: {} {}", x, y);
    }

    pub fn empty_before_fn() {
        println!("before::empty_before_fn::Before");
    }

    pub fn struct_before_fn(ctx: &HelloContext) {
        println!("before::struct_before_fn::Before: {} {}", ctx.x, ctx.y);
    }

    pub fn struct_before_fn_lifetime<'a>(ctx: &'a LifetimeHelloContext<'a>) {
        println!(
            "before::struct_before_fn_lifetime::Before: {} {}",
            ctx.x, ctx.y
        );
    }
}

mod after {
    use crate::{HelloContext, LifetimeHelloContext};

    pub fn after_fn(x: &i64, y: &f64) {
        println!("after::after_fn::After: {} {}", x, y);
    }

    pub fn empty_after_fn() {
        println!("after::empty_after_fn::After");
    }

    pub fn struct_after_fn(ctx: &HelloContext) {
        println!("after::struct_after_fn::After: {} {}", ctx.x, ctx.y);
    }

    pub fn struct_after_fn_lifetime<'a>(ctx: &'a LifetimeHelloContext<'a>) {
        println!(
            "after::struct_after_fn_lifetime::After: {} {}",
            ctx.x, ctx.y
        );
    }

    // $ cargo test --test integration_tests -- --show-output
}
