use super::{sys, CFHashCode, CFIndex};
use crate::core::{Arc, ObjectType};
use std::{cell::UnsafeCell, fmt, hash, ptr::NonNull};

/// Unique constant integer value that identifies particular Core Foundation
/// opaque types.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/corefoundation/cftypeid?language=swift) |
/// [Objective-C](https://developer.apple.com/documentation/corefoundation/cftypeid?language=objc)
pub type CFTypeID = usize;

/// An instance of a Core Foundation type.
///
/// This is designed to be used behind a reference. In the future, this will be
/// defined as an
/// [`extern type`](https://github.com/rust-lang/rfcs/blob/master/text/1861-extern-types.md).
///
/// All Core Foundation types within this crate ultimately
/// [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html) to this type.
#[repr(C)]
pub struct CFType {
    // Stores data that may be mutated behind a shared reference. Internal
    // mutability triggers undefined behavior without `UnsafeCell`.
    _data: UnsafeCell<[u8; 0]>,
}

impl ObjectType for CFType {
    #[inline]
    #[doc(alias = "CFRetain")]
    fn retain(obj: &Self) -> Arc<Self> {
        unsafe { Arc::from_raw(sys::CFRetain(obj)) }
    }

    #[inline]
    #[doc(alias = "CFRelease")]
    unsafe fn release(obj: NonNull<Self>) {
        sys::CFRelease(obj.as_ptr());
    }
}

// This type is used globally, so we must be able to share it across threads.
unsafe impl Sync for CFType {}
unsafe impl Send for CFType {}

impl AsRef<CFType> for CFType {
    #[inline]
    fn as_ref(&self) -> &CFType {
        self
    }
}

impl PartialEq for CFType {
    #[inline]
    #[doc(alias = "CFEqual")]
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::CFEqual(self, other) != 0 }
    }
}

impl hash::Hash for CFType {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.hash().hash(state)
    }
}

impl fmt::Debug for CFType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as *const Self).fmt(f)
    }
}

impl CFType {
    /// Returns this object's reference count.
    ///
    /// This method is only useful for debugging certain objects.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1521288-cfgetretaincount).
    #[inline]
    #[doc(alias = "CFGetRetainCount")]
    pub fn retain_count(&self) -> CFIndex {
        unsafe { sys::CFGetRetainCount(self) }
    }

    /// Returns a code that can be used to identify `self` in a hashing
    /// structure.
    #[inline]
    #[doc(alias = "CFHash")]
    pub fn hash(&self) -> CFHashCode {
        unsafe { sys::CFHash(self) }
    }

    /// Returns the unique identifier of an opaque type to which `self` belongs.
    #[inline]
    #[doc(alias = "CFGetTypeID")]
    pub fn get_type_id(&self) -> CFTypeID {
        unsafe { sys::CFGetTypeID(self) }
    }

    // TODO: `CFGetAllocator`

    // TODO: `CFCopyDescription`
}

/// An automatically-reference-counted pointer to a type-erased Core Foundation
/// object.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/corefoundation/cftyperef?language=swift) |
/// [Objective-C](https://developer.apple.com/documentation/corefoundation/cftyperef?language=objc)
pub type CFTypeRef = Arc<CFType>;
