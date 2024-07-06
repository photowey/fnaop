/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http:///www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(non_snake_case)]

// ----------------------------------------------------------------

use proc_macro::TokenStream;

use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta, Pat, PatType, ReturnType,
    Type, Visibility,
};

// ----------------------------------------------------------------

/// An attribute macro for applying Aspect-Oriented Programming (AOP) to functions.
///
/// The `Aspect` macro allows you to specify `before` and `after` functions that
/// will be called before and after the target function respectively. This is useful
/// for encapsulating cross-cutting concerns such as logging, metrics, or other
/// side effects.
///
/// # Arguments
///
/// - `before` - The path to a function to be called before the target function.
/// - `after` - The path to a function to be called after the target function.
///
/// # Examples
///
/// # #1
/// ```rust
/// use fnaop::Aspect;
///
/// fn before_fn(x: &i64) {
///     println!("before_fn::Before: {}", x);
/// }
///
/// fn after_fn(x: &i64) {
///     println!("before_fn::After: {}", x);
/// }
///
/// fn before_fn_empty() {
///     println!("before_fn_empty::Before");
/// }
///
/// fn after_fn_empty() {
///     println!("before_fn_empty::After");
/// }
///
/// #[Aspect(before = "before_fn", after = "after_fn")]
/// pub fn say_hello(x: i64) {
///     println!("Hello:say_hello, {}", x);
/// }
///
/// #[Aspect(before = "before_fn_empty", after = "before_fn_empty")]
/// pub fn say_hello_empty() {
///     println!("Hello:say_hello_empty");
/// }
///
/// fn main() {
///     say_hello(42);
/// }
/// ```
///
/// # #2
/// ```rust
/// use fnaop::Aspect;
///
/// #[derive(Clone, Debug, PartialEq)]
/// pub struct HelloContext {
///     x: i64,
///     y: f64,
/// }
///
/// #[derive(Clone, Debug, PartialEq)]
/// pub struct LifetimeHelloContext<'a> {
///     x: &'a i64,
///     y: &'a f64,
/// }
///
/// mod before {
///     use super::*;
///
///     pub fn struct_before_fn(ctx: &HelloContext) {
///         println!("before::struct_before_fn::Before: {} {}", ctx.x, ctx.y);
///     }
///
///     pub fn struct_before_fn_lifetime<'a>(ctx: &'a LifetimeHelloContext<'a>) {
///         println!("before::struct_before_fn_lifetime::Before: {} {}", ctx.x, ctx.y);
///     }
/// }
///
/// mod after {
///     use super::*;
///
///     pub fn struct_after_fn(ctx: &HelloContext) {
///         println!("after::struct_after_fn::After: {} {}", ctx.x, ctx.y);
///     }
///
///     pub fn struct_after_fn_lifetime<'a>(ctx: &'a LifetimeHelloContext<'a>) {
///         println!("after::struct_after_fn_lifetime::After: {} {}", ctx.x, ctx.y);
///     }
/// }
///
/// #[Aspect(before = "before::struct_before_fn_lifetime", after = "after::struct_after_fn_lifetime")]
/// pub fn say_hello_struct_lifetime_with_return<'a>(ctx: LifetimeHelloContext<'a>) -> i64 {
///     println!("struct::Hello, {} {}", ctx.x, ctx.y);
///
///     *ctx.x
/// }
///
/// fn main() {
///     let x = 9527;
///     let y = 8848.01;
///     let ctx = LifetimeHelloContext {
///         x: &x,
///         y: &y,
///     };
///
///     let rvt = say_hello_struct_lifetime_with_return(ctx);
///
///     assert_eq!(9527, rvt)
/// }
/// ```
///
/// @author photowey
///
/// @version 0.1.0
///
/// @since 2024/07/07
///
#[proc_macro_attribute]
pub fn Aspect(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_inputs = &input.sig.inputs;
    let fn_generics = &input.sig.generics;
    let fn_output = &input.sig.output;

    let fn_args: Vec<_> = fn_inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(PatType { pat, ty, .. }) => {
                if let Pat::Ident(pat_ident) = &**pat {
                    let ident = &pat_ident.ident;
                    match **ty {
                        Type::Reference(_) => quote! { #ident },
                        _ => quote! { &#ident },
                    }
                } else {
                    panic!("Expected an identifier pattern")
                }
            }
            syn::FnArg::Receiver(_) => panic!("Expected a typed pattern, not self"),
        })
        .collect();

    let mut before_fn = None;
    let mut after_fn = None;

    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(meta)) = arg {
            if let (Some(ident), Lit::Str(lit_str)) = (meta.path.get_ident(), meta.lit) {
                match ident.to_string().as_str() {
                    "before" => {
                        before_fn = Some(syn::parse_str::<syn::Path>(&lit_str.value()).unwrap())
                    }
                    "after" => {
                        after_fn = Some(syn::parse_str::<syn::Path>(&lit_str.value()).unwrap())
                    }
                    _ => {}
                }
            }
        }
    }

    let before_call = if let Some(before) = before_fn {
        quote! {
            #before(#(#fn_args),*);
        }
    } else {
        quote! {}
    };

    let after_call = if let Some(after) = after_fn {
        quote! {
            #after(#(#fn_args),*);
        }
    } else {
        quote! {}
    };

    let vis = match input.vis {
        Visibility::Public(_) => quote! { pub },
        _ => quote! { pub(crate) },
    };

    let expanded = match fn_output {
        ReturnType::Default => {
            quote! {
                #vis fn #fn_name #fn_generics (#fn_inputs) {
                    #before_call
                    #fn_block
                    #after_call
                }
            }
        }

        ReturnType::Type(_, ty) => {
            quote! {
                #vis fn #fn_name #fn_generics (#fn_inputs) -> #ty {
                    #before_call
                    let result = (|| { #fn_block })();
                    #after_call
                    result
                }
            }
        }
    };

    TokenStream::from(expanded)
}
