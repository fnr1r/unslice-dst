# unslice-dst

A more flexible version of [slice-dst](https://crates.io/crates/slice-dst).
([GitHub](https://github.com/CAD97/pointer-utils/tree/master/crates/slice-dst))

Started because I wanted to save a `size_of::<usize>()` of memory. ¯\\\_(ツ)\_/¯

## Features

### without std / safe

- `cast_macro_tt` - uses a more flexible TT muncher macro

### without std / unsafe

- `cast_unseal` - reveals the sealed type for manual implementation  
  WARNING: Please don't. If the macro doesn't work, your type likely isn't
  compatible.
- `container_unseal` - allows implementation of `DstContainer` for foreign
  smart pointers
- `container_rc_optimize` - optimize `(A)Rc` container allocations  
  WARNING: prone to ABI changes
- `core_ffi_cstr_impl` - implements [`DstCast`] and [`DstLayout`] for
  [`core::ffi::CStr`] (requires Rust 1.64)  
  WARNING: The repr of `CStr` is not guaranteed! See the official [`core`] docs
  for more info.
- `layout_automatic` - use the automatic layout implementation  
  WARNING: relies on Undefined Behavior, namely creating a temporary null
  reference, which shouldn't be read.  
  Avoidable on nightly thanks to [`core::alloc::Layout::for_value_raw`].

### std

- `std` - Rust `std` crate support, featuring:
  - impls of `DstCast` for `std` slice-like types
  - impls of `DstLayout` for `std` for slice-like types
  - depends on `cast_macro_tt` because I'm lazy
- `std_ffi_cstr_impl` - same as `core_ffi_cstr_impl`, but for
  [`std::ffi::CStr`]  
  WARNING: Same issues as `core_ffi_cstr_impl` apply.
