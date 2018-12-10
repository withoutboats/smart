use std::ops::Deref;
use std::mem;
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::Arc;

pub(crate) struct Pointer<T: ?Sized> {
    ptr: NonNull<T>,
    clone: unsafe fn(&T) -> NonNull<T>,
    drop: unsafe fn(NonNull<T>),
    pub(crate) sync: bool,
}

impl<T: ?Sized> Deref for Pointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> From<Rc<T>> for Pointer<T> {
    fn from(ptr: Rc<T>) -> Pointer<T> {
        unsafe fn clone<T: ?Sized>(arg: &T) -> NonNull<T> {
            let arg = Rc::from_raw(arg);
            let rc = arg.clone();
            mem::forget(arg);
            NonNull::new_unchecked(Rc::into_raw(rc) as *mut _)
        }
        unsafe fn drop<T: ?Sized>(ptr: NonNull<T>) {
            mem::drop(Rc::from_raw(ptr.as_ptr()));
        }
        Pointer {
            ptr: unsafe { NonNull::new_unchecked(Rc::into_raw(ptr) as *mut _) },
            clone: clone::<T>,
            drop: drop::<T>,
            sync: false,
        }
    }
}

impl<T: ?Sized> From<Arc<T>> for Pointer<T> {
    fn from(ptr: Arc<T>) -> Pointer<T> {
        unsafe fn clone<T: ?Sized>(arg: &T) -> NonNull<T> {
            let arg = Arc::from_raw(arg);
            let rc = arg.clone();
            mem::forget(arg);
            NonNull::new_unchecked(Arc::into_raw(rc) as *mut _)
        }
        unsafe fn drop<T: ?Sized>(ptr: NonNull<T>) {
            mem::drop(Arc::from_raw(ptr.as_ptr()));
        }
        Pointer {
            ptr: unsafe { NonNull::new_unchecked(Arc::into_raw(ptr) as *mut _) },
            clone: clone::<T>,
            drop: drop::<T>,
            sync: true,
        }
    }
}

impl<T: ?Sized> From<&'static T> for Pointer<T> {
    fn from(ptr: &'static T) -> Pointer<T> {
        unsafe fn clone<T: ?Sized>(arg: &T) -> NonNull<T> {
            NonNull::from(arg)
        }
        unsafe fn drop<T: ?Sized>(_ptr: NonNull<T>) { }
        Pointer {
            ptr: NonNull::from(ptr),
            clone: clone::<T>,
            drop: drop::<T>,
            sync: true,
        }
    }
}

impl<T: ?Sized> Clone for Pointer<T> {
    fn clone(&self) -> Pointer<T> {
        Pointer {
            ptr: unsafe { (self.clone)(self.ptr.as_ref()) },
            clone: self.clone,
            drop: self.drop,
            sync: self.sync,
        }
    }
}

impl<T: ?Sized> Drop for Pointer<T> {
    fn drop(&mut self) {
        unsafe { (self.drop)(self.ptr) }
    }
}
