use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use crate::SyncPointer;
use crate::pointer::Pointer;

/// A dynamic smart pointer which is not guaranteed to be threadsafe.
///
/// This type can be constructed from any shared ownership pointer, including Rc. It also provides
/// a conversion method to SyncPointer, which must be threadsafe. Converting a SharedPointer with
/// a non-threadsafe implementation would panic.
pub struct SharedPointer<T: ?Sized> {
    pub(crate) inner: Pointer<T>,
}

impl<T> SharedPointer<T> {
    /// Construct a new SharedPointer directly.
    ///
    /// By default, this uses an Arc, so a SharedPointer constructed this way can be cast into a
    /// SyncPointer without panicking.
    pub fn new(data: T) -> SharedPointer<T> {
        SharedPointer::from(Arc::new(data))
    }
}

impl<T: ?Sized> Deref for SharedPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.inner
    }
}

impl<T: ?Sized> From<&'static T> for SharedPointer<T> {
    fn from(ptr: &'static T) -> SharedPointer<T> {
        SharedPointer { inner: Pointer::from(ptr) }
    }
}

impl<T: ?Sized> From<Rc<T>> for SharedPointer<T> {
    fn from(ptr: Rc<T>) -> SharedPointer<T> {
        SharedPointer { inner: Pointer::from(ptr) }
    }
}

impl<T: ?Sized> From<Arc<T>> for SharedPointer<T> {
    fn from(ptr: Arc<T>) -> SharedPointer<T> {
        SharedPointer { inner: Pointer::from(ptr) }
    }
}

impl<T: ?Sized> Clone for SharedPointer<T> {
    fn clone(&self) -> SharedPointer<T> {
        SharedPointer { inner: self.inner.clone() }
    }
}

impl<T: ?Sized> From<SyncPointer<T>> for SharedPointer<T> {
    fn from(ptr: SyncPointer<T>) -> SharedPointer<T> {
        SharedPointer { inner: ptr.inner }
    }
}
