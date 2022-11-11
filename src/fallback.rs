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

use alloc::alloc::Layout;
use core::fmt::{self, Display, Formatter};
use core::mem::MaybeUninit;
use core::ptr::{self, NonNull};

/// A fallback for [`alloc::alloc::AllocError`], which is currently unstable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AllocError;

impl Display for AllocError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "memory allocation failed")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "std")))]
impl std::error::Error for AllocError {}

/// A fallback for [`alloc::alloc::Allocator`], which is currently unstable.
///
/// # Safety
///
/// See [`alloc::alloc::Allocator`].
pub unsafe trait Allocator {
    /// See [`alloc::alloc::Allocator::allocate`].
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>;

    /// See [`alloc::alloc::Allocator::deallocate`].
    ///
    /// # Safety
    ///
    /// See [`alloc::alloc::Allocator::deallocate`].
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);

    /// See [`alloc::alloc::Allocator::allocate_zeroed`].
    fn allocate_zeroed(
        &self,
        layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = self.allocate(layout)?;
        // SAFETY: `Self::allocate` always returns a pointer to valid memory.
        unsafe {
            let len = (*(ptr.as_ptr() as *mut [MaybeUninit<u8>])).len();
            (ptr.as_ptr() as *mut u8).write_bytes(0_u8, len);
        }
        Ok(ptr)
    }

    /// See [`alloc::alloc::Allocator::grow`].
    ///
    /// # Safety
    ///
    /// See [`alloc::alloc::Allocator::grow`].
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let new = self.allocate(new_layout)?;
        // SAFETY: Checked by caller.
        unsafe {
            (new.as_ptr() as *mut u8)
                .copy_from_nonoverlapping(ptr.as_ptr(), old_layout.size());
            self.deallocate(ptr, old_layout);
        }
        Ok(new)
    }

    /// See [`alloc::alloc::Allocator::grow_zeroed`].
    ///
    /// # Safety
    ///
    /// See [`alloc::alloc::Allocator::grow_zeroed`].
    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let new = self.allocate(new_layout)?;
        // SAFETY: `Self::allocate` always returns a pointer to valid memory.
        // Sizes are checked by caller (new size must not be less than old
        // size).
        unsafe {
            let len = (*(new.as_ptr() as *mut [MaybeUninit<u8>])).len();
            (new.as_ptr() as *mut u8)
                .copy_from_nonoverlapping(ptr.as_ptr(), old_layout.size());
            (new.as_ptr() as *mut u8)
                .add(old_layout.size())
                .write_bytes(0_u8, len - old_layout.size());
            self.deallocate(ptr, old_layout);
        }
        Ok(new)
    }

    /// See [`alloc::alloc::Allocator::shrink`].
    ///
    /// # Safety
    ///
    /// See [`alloc::alloc::Allocator::shrink`].
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let new = self.allocate(new_layout)?;
        // SAFETY: `Self::allocate` always returns a pointer to valid memory.
        // Sizes are checked by caller (new size must not be greater than old
        // size).
        unsafe {
            let len = (*(new.as_ptr() as *mut [MaybeUninit<u8>])).len();
            (new.as_ptr() as *mut u8)
                .copy_from_nonoverlapping(ptr.as_ptr(), len);
            self.deallocate(ptr, old_layout);
        }
        Ok(new)
    }

    /// See [`alloc::alloc::Allocator::by_ref`].
    fn by_ref(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
}

// SAFETY: This impl simply forwards to `A`.
unsafe impl<A> Allocator for &A
where
    A: Allocator + ?Sized,
{
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        A::allocate(*self, layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        // SAFETY: Checked by caller.
        unsafe {
            A::deallocate(*self, ptr, layout);
        }
    }
}

/// A fallback for [`alloc::alloc::Global`], which is currently unstable.
#[derive(Clone, Copy, Debug, Default)]
pub struct Global;

// SAFETY: The `alloc` and `dealloc` functions in the standard library behave
// as required. Clones of this allocator will necessarily behave the same, as
// they forward to the global allocator.
unsafe impl Allocator for Global {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        assert!(layout.size() != 0);
        NonNull::new(ptr::slice_from_raw_parts_mut(
            // SAFETY: We ensured that the size of the layout is not 0.
            unsafe { alloc::alloc::alloc(layout) },
            layout.size(),
        ))
        .ok_or(AllocError)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        // SAFETY: Ensured by caller.
        unsafe { alloc::alloc::dealloc(ptr.as_ptr(), layout) };
    }
}
