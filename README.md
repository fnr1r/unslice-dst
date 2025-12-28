# unslice-dst

A more flexible version of [slice-dst](https://crates.io/crates/slice-dst).
([GitHub](https://github.com/CAD97/pointer-utils/tree/master/crates/slice-dst))

Started because I wanted to save a `size_of::<usize>()` of memory. ¯\\\_(ツ)\_/¯

## Features

- `cast_impl_core_cstr` - implements [`DstCast`] for [`core::ffi::CStr`]
  (requires Rust 1.64)  
  WARNING: The repr of `CStr` is not guaranteed! See the official [`core`] docs
  for more info.
- `cast_impl_std_cstr` - same as `cast_impl_core_cstr`, but for
  [`std::ffi::CStr`]
- `cast_macro_tt` - uses a more flexible TT muncher macro
- `casting_unseal` - reveals the sealed type for manual implementation  
  WARNING: Please don't. If the macro doesn't work, your type likely isn't
  compatible.
- `layout_automatic` - use the automatic layout implementation
  WARNING: relies on Undefined Behavior
- `std` - Rust `std` crate support, featuring:
  - impls of `DstCast` for `std` slice-like types
  - impls of `DstLayout` for `std` (only on Unix tho)
  - depends on `cast_macro_tt` because I'm lazy
