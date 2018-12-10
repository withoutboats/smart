use std::ops::Deref;
use std::sync::Arc;

use crate::SharedPointer;
use crate::pointer::Pointer;

/// A dynamic threadsafe smart pointer.
///
/// A SyncPointer implements Send and Sync, which means the smart pointer it wraps must be Send and
/// Sync (if the data it owns is, that is). For this reason, an Rc cannot be made into a
/// SyncPointer.
pub struct SyncPointer<T: ?Sized> {
    pub(crate) inner: Pointer<T>,
}

impl<T> SyncPointer<T> {
    /// Construct a new SyncPointer directly.
    pub fn new(data: T) -> SyncPointer<T> {
        SyncPointer::from(Arc::new(data))
    }
}

impl<T: ?Sized> Deref for SyncPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.inner
    }
}
impl<T: ?Sized> From<&'static T> for SyncPointer<T> {
    fn from(ptr: &'static T) -> SyncPointer<T> {
        SyncPointer { inner: Pointer::from(ptr) }
    }
}

impl<T: ?Sized> From<Arc<T>> for SyncPointer<T> {
    fn from(ptr: Arc<T>) -> SyncPointer<T> {
        SyncPointer { inner: Pointer::from(ptr) }
    }
}

impl<T: ?Sized> From<SharedPointer<T>> for SyncPointer<T> {
    fn from(ptr: SharedPointer<T>) -> SyncPointer<T> {
        if !ptr.inner.sync {
            panic!("Cannot upgrade non-threadsafe SharedPointer to SyncPointer");
        }
        SyncPointer { inner: ptr.inner }
    }
}

unsafe impl<T: Send + Sync + ?Sized> Send for SyncPointer<T> { }
unsafe impl<T: Send + Sync + ?Sized> Sync for SyncPointer<T> { }

impl<T: ?Sized> Clone for SyncPointer<T> {
    fn clone(&self) -> SyncPointer<T> {
        SyncPointer { inner: self.inner.clone() }
    }
}
