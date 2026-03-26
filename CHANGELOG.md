Changelog
=========

0.1.9
-----

* `HAS_ALLOCATOR_API` is no longer unconditionally true on Rust nightly. Note
  that this makes it more important to duplicate the crate in both
  `dependencies` and `build-dependencies` as instructed.
* Enabling `allocator_api` on stable Rust is no longer an error; it will simply
  be ignored. However, doing this is not recommended, because if Rust
  stabilizes the allocator API with breaking changes, compilation errors could
  occur on stable Rust.
* Updated usage instructions to be clearer and more explicit.

0.1.8
-----

* Added `rerun-if-changed` to build script examples.

0.1.7
-----

* Added `HAS_ALLOCATOR_API` and instructions for how to use the crate in a way
  that avoids compilation errors if someone enables the `allocator_api`
  feature.

0.1.6
-----

* Added missing `?Sized` bound on implementation of `Allocator` for references.

0.1.5
-----

* Fixed safety comments in `Allocator::allocate_zeroed`.
* Added missing safety comment in `Allocator::shrink`.

0.1.4
-----

* Implemented the remaining methods of `Allocator`.

0.1.3
-----

* Added the unstable `doc_cfg` feature.

0.1.2
-----

* Added missing `Self: Sized` bound to `Allocator::by_ref`.

0.1.1
-----

* Implemented `Allocator` for references to an allocator.

0.1.0
-----

Initial release.
