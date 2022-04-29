allocator-fallback
==================

This crate provides a minimal fallback for the standard library’s allocator
API, which is currently unstable. It doesn’t provide everything that the
standard library offers, but it provides enough for basic use cases.

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
