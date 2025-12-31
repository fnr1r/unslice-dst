# unslice-dst

A more flexible version of [slice-dst](https://crates.io/crates/slice-dst).
([GitHub](https://github.com/CAD97/pointer-utils/tree/master/crates/slice-dst))

Started because I wanted to save a `size_of::<usize>()` of memory. ¯\\\_(ツ)\_/¯

## Features

- `cast_impl_core_cstr` - implements [`DstCast`] for [`core::ffi::CStr`]
  (requires Rust 1.64)  
  WARNING: The repr of `CStr` is not guaranteed! See the official [`core`] docs
  for more info.
