/*
 * Copyright 2022 taylor.fish <contact@taylor.fish>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "allocator_api", feature(allocator_api))]
#![cfg_attr(feature = "doc_cfg", feature(doc_cfg))]
#![deny(unsafe_op_in_unsafe_fn)]

//! This crate provides a minimal fallback for the standard library’s allocator
//! API, which is currently unstable. It doesn’t provide everything that the
//! standard library offers, but it provides enough for basic use cases.
//!
//! Crate features
//! --------------
//!
//! If the crate feature `allocator_api` is enabled, this crate will simply
//! re-export the real allocator API in the standard library. Of course, this
//! requires Rust nightly.
//!
//! If the crate feature `std` is enabled (the default), the crate will use
//! [`std`]; otherwise, it will be `no_std`. Using [`std`] allows
//! [`AllocError`] to implement [`std::error::Error`].

extern crate alloc;

#[cfg(not(feature = "allocator_api"))]
mod fallback;

#[cfg(not(feature = "allocator_api"))]
pub use fallback::{AllocError, Allocator, Global};

#[cfg(feature = "allocator_api")]
pub use alloc::alloc::{AllocError, Allocator, Global};

#[test]
fn test() {
    use alloc::alloc::Layout;
    let _: &dyn Allocator = &Global;
    let ptr = Global.allocate(Layout::new::<u64>()).unwrap();
    unsafe {
        ptr.cast::<u64>().as_ptr().write(0x123456789abcdef);
        (&&&Global).deallocate(ptr.cast(), Layout::new::<u64>());
    }
}
