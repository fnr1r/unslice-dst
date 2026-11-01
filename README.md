# unslice-dst

A more flexible version of [slice-dst](https://crates.io/crates/slice-dst).
([GitHub](https://github.com/CAD97/pointer-utils/tree/master/crates/slice-dst))

Started because I wanted to save a `size_of::<usize>()` of memory. ¯\\\_(ツ)\_/¯

## Goals

- Have a simple-to-use macro for creating DSTs
- Inline everything. And at least try to keep everything
  [`const`](https://doc.rust-lang.org/std/keyword.const.html#compile-time-evaluable-functions).
- Avoid required dependencies at all costs (extra, optional trait impls are fine)
- Stay on lowest comfortable MSRV
- (eventually) publish everything i still have in my local repo

## Layers

- Layer 1 - Missing stable Rust features
  - [`cast`] - (limited) [`?Sized`] pointer casts
  - [`uninit`] - uninitialized pointers (depends on [`cast`])
  - [`layout`] - (limited) [`?Sized`] layouts (may depend on [`cast`])
- Layer 2 - Adapters (depends on Layer 1)
  - [`fat_ptr`] - fat pointer struct
  - `initializers` - pre-written... initializers (private)
    (depends on manual [`layout`] implementation)
  - `container` - smart pointer traits (may be private)
  - [`allocation`] - combines `container` with [`alloc`] (depends on `container`)
- Layer 3 - Final types - [`structs`] (depends on everything else)

## Features

### no deps / safe

- `cast_macro_tt` - uses a more flexible TT muncher macro

### no deps / unsafe

- `cast_unseal` - reveals the sealed type for manual implementation  
  WARNING: Please don't. If the macro doesn't work, your type likely isn't
  compatible.
- `container_unseal` - allows implementation of `DstContainer` for foreign
  smart pointers
- `container_rc_optimize` - optimize `(A)Rc` container allocations  
  WARNING: prone to ABI changes
- `layout_automatic` - use the automatic layout implementation  
  WARNING: relies on Undefined Behavior, namely creating a temporary null
  reference, which shouldn't be read.  
  Avoidable on nightly and MSRV `1.99` thanks to `layout_for_ptr` feature
  ([`core::alloc::Layout::for_value_raw`]).

### rustc

- `layout_feature_layout_for_ptr` - use `layout_for_ptr`

### core

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

### Extras

- `plain` - implement [`Plain`] for own structs
  (why not [`bytemuck`]? because [`bytemuck`] requires [`Sized`])

[`bytemuck`]: https://crates.io/crates/bytemuck
[`?Sized`]: core::marker::Sized
[`Plain`]: https://docs.rs/plain/0.2.3/plain/trait.Plain.html
