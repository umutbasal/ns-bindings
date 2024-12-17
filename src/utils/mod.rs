pub mod nsarray;
pub mod nsstring;

use std::ops::Deref;

use cocoa::base::nil;
use objc::rc::StrongPtr;
use objc::runtime::Object;

#[repr(transparent)]
#[derive(Clone)]
pub struct Id(objc::rc::StrongPtr);

impl Id {
    pub unsafe fn new(ptr: *mut Object) -> Self {
        Self(StrongPtr::new(ptr))
    }

    pub fn autorelease(self) -> *mut Object {
        self.0.autorelease()
    }
}

impl std::ops::Deref for Id {
    type Target = StrongPtr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A non-owning reference to an Objective-C object.
#[repr(transparent)]
pub struct IdRef(*const Object);

impl IdRef {
    /// # Safety
    ///
    /// This is unsafe because the caller must ensure that the pointer is valid
    /// for the lifetime of the returned object.
    pub unsafe fn new(inner: *const Object) -> IdRef {
        IdRef(inner)
    }

    pub fn is_nil(&self) -> bool {
        self.0 == nil
    }

    /// # Safety
    ///
    /// This is unsafe because the caller must ensure that the pointer has exclusive
    /// access to the object.
    pub unsafe fn as_mut_ptr(&self) -> *mut Object {
        self.0 as *mut _
    }
}

impl Deref for IdRef {
    type Target = *const Object;

    fn deref(&self) -> &*const Object {
        &self.0
    }
}
