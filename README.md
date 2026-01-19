# unslice-dst

A more flexible version of [slice-dst](https://crates.io/crates/slice-dst).
([GitHub](https://github.com/CAD97/pointer-utils/tree/master/crates/slice-dst))

Started because I wanted to save a `size_of::<usize>()` of memory. ¯\\\_(ツ)\_/¯

## Features

### without std or alloc / safe

- `cast_macro_tt` - uses a more flexible TT muncher macro

### without std or alloc / unsafe

- `cast_unseal` - reveals the sealed type for manual implementation  
  WARNING: Please don't. If the macro doesn't work, your type likely isn't
  compatible.
- `core_ffi_cstr_impl` - implements [`DstCast`] and [`DstLayout`] for
  [`core::ffi::CStr`] (requires Rust 1.64)  
  WARNING: The repr of `CStr` is not guaranteed! See the official [`core`] docs
  for more info.

### std

- `std` - Rust `std` crate support, featuring:
  - impls of `DstCast` for `std` slice-like types
  - impls of `DstLayout` for `std` for slice-like types
  - depends on `cast_macro_tt` because I'm lazy
- `std_ffi_cstr_impl` - same as `core_ffi_cstr_impl`, but for
  [`std::ffi::CStr`]  
  WARNING: Same issues as `core_ffi_cstr_impl` apply.
