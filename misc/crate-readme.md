allocator-fallback
==================

This crate provides a minimal fallback for the standard library’s allocator
API, which is currently unstable.

Usage
-----

Because allocator-fallback can be configured to re-export the real
unstable allocator API (see [Crate features](#crate-features)), users
of this crate must make sure they conditionally enable
`#![feature(allocator_api)]` in preparation for this occurrence; otherwise,
compilation errors may occur. This is the case even for crates that never
directly enable allocator-fallback’s `allocator_api` feature, because a
different crate that also depends on allocator-fallback could enable it.

To accomplish this, in `Cargo.toml`, duplicate your dependency on
`allocator-fallback` in the `[build-dependencies]` section. For example:

```toml
[dependencies]
allocator-fallback = "0.1.7"

[build-dependencies]
allocator-fallback = "0.1.7"
```

Then, add a [build script][build] (`build.rs`) with the following
contents:[^1]

[build]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

```rust
fn main() {
   if allocator_fallback::HAS_ALLOCATOR_API {
       println!("cargo:rustc-cfg=has_allocator_api");
   }
}
```

Finally, at the top of your crate root (likely `lib.rs` or `main.rs`), add
the following:

```rust
#![cfg_attr(has_allocator_api, feature(allocator_api))]
```

### Use as an optional dependency

The instructions above will not work if `allocator-fallback` is declared
as an optional dependency. In this case, adjust the instructions as
follows:

Duplicate the dependency on `allocator-fallback` in `[build-dependencies]`
as before, keeping `optional = true` in both occurrences. For example:

```toml
[dependencies.allocator-fallback]
version = "0.1.7"
optional = true

[build-dependencies.allocator-fallback]
version = "0.1.7"
optional = true
```

Then, use the following as the contents of your build script (`build.rs`)
instead:[^1]

```rust
fn main() {
   #[cfg(feature = "allocator-fallback")]
   if allocator_fallback::HAS_ALLOCATOR_API {
       println!("cargo:rustc-cfg=has_allocator_api");
   }
}
```

Finally, as before, add the following to the top of your crate root:

```rust
#![cfg_attr(has_allocator_api, feature(allocator_api))]
```

[^1]: These build script code snippets have been released to the public
      domain using the [CC0 1.0 Universal Public Domain Dedication][CC0].

[CC0]: https://creativecommons.org/publicdomain/zero/1.0/legalcode.txt

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
