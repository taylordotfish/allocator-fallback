allocator-fallback
==================

This crate provides a minimal fallback for the standard library’s allocator
API, which is currently unstable.

Usage
-----

Because allocator-fallback can be configured to re-export the real
unstable allocator API via the [`allocator_api` crate feature][features],
your crate must conditionally enable `#![feature(allocator_api)]` in
preparation for this occurrence; otherwise, compilation errors may occur.
This is true even if your crate never directly enables the `allocator_api`
feature, because a different crate that also depends on allocator-fallback
could enable it.

[features]: #crate-features

To accomplish this, in `Cargo.toml`, add allocator-fallback as both a
regular dependency and a build dependency:

```toml
[dependencies]
allocator-fallback = "0.1.10"

[build-dependencies]
allocator-fallback = "0.1.10"
```

**Note:** It is very important that the two dependencies are identical. Do
not enable a feature in one without enabling it in the other.

Then add a [build script][build] (`build.rs`) with the following
contents:[^1]

[build]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

```rust
fn main() {
   if allocator_fallback::HAS_ALLOCATOR_API {
       println!("cargo:rustc-cfg=has_allocator_api");
   }
   println!("cargo:rerun-if-changed=build.rs");
}
```

Finally, at the top of your crate root (likely `lib.rs` or `main.rs`), add
the following:

```rust
#![cfg_attr(has_allocator_api, feature(allocator_api))]
```

Rust may show a warning about an “unexpected `cfg` condition name”; you can
silence it by adding the following to Cargo.toml:

```toml
[lints.rust.unexpected_cfgs]
level = "warn"
check-cfg = ["cfg(has_allocator_api)"]
```

### Use as an optional dependency

If you’d like allocator-fallback to be an optional dependency, first add
`optional = true` to both of its declarations as a dependency:

```toml
[dependencies.allocator-fallback]
version = "0.1.10"
optional = true

[build-dependencies.allocator-fallback]
version = "0.1.10"
optional = true
```

Then adjust `build.rs` as follows:[^1]

```rust
fn main() {
   #[cfg(feature = "allocator-fallback")]
   if allocator_fallback::HAS_ALLOCATOR_API {
       println!("cargo:rustc-cfg=has_allocator_api");
   }
   println!("cargo:rerun-if-changed=build.rs");
}
```

Finally, make sure you still have the following in your crate root:

```rust
#![cfg_attr(has_allocator_api, feature(allocator_api))]
```

[^1]: These build script code snippets have been released to the public
      domain using the [CC0 1.0 Universal Public Domain Dedication][CC0].

[CC0]: https://creativecommons.org/publicdomain/zero/1.0/legalcode.txt

### Exposing an `allocator_api` feature in your crate

If you want your crate to directly provide an `allocator_api` feature that
enables the real allocator API, add the following to your Cargo.toml:

```toml
[features]
allocator_api = ["allocator-fallback/allocator_api"]
```

If you declared allocator-fallback as an optional dependency, add the
following instead:

```toml
[features]
allocator_api = ["allocator-fallback?/allocator_api"]
```

Crate features
--------------

If the crate feature `allocator_api` is enabled, this crate will simply
re-export the real allocator API in the standard library. Of course, this
requires Rust nightly.

If the crate feature `std` is enabled (the default), the crate will use
[`std`]; otherwise, it will be `no_std`. Using [`std`] allows
[`AllocError`] to implement [`std::error::Error`].

[`std`]: https://doc.rust-lang.org/std/
[`AllocError`]: https://doc.rust-lang.org/std/alloc/struct.AllocError.html
[`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html

Documentation
-------------

[Documentation is available on docs.rs.](https://docs.rs/allocator-fallback)

License
-------

allocator-fallback is licensed under version 2 of the Apache License. See
[LICENSE](LICENSE).

Contributing
------------

By contributing to allocator-fallback, you agree that your contribution may be
used according to the terms of allocator-fallback’s license.
