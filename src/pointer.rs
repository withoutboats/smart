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
            rc_into_raw_non_null(rc)
        }
        unsafe fn drop<T: ?Sized>(ptr: NonNull<T>) {
            mem::drop(Rc::from_raw(ptr.as_ptr()));
        }
        Pointer {
            ptr: rc_into_raw_non_null(ptr),
            clone,
            drop,
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
            arc_into_raw_non_null(rc)
        }
        unsafe fn drop<T: ?Sized>(ptr: NonNull<T>) {
            mem::drop(Arc::from_raw(ptr.as_ptr()));
        }
        Pointer {
            ptr: arc_into_raw_non_null(ptr),
            clone,
            drop,
            sync: true,
        }
    }
}

impl<T: ?Sized> From<&'static T> for Pointer<T> {
    fn from(ptr: &'static T) -> Pointer<T> {
        fn clone<T: ?Sized>(arg: &T) -> NonNull<T> {
            NonNull::from(arg)
        }
        fn drop<T: ?Sized>(_ptr: NonNull<T>) { }
        Pointer {
            ptr: NonNull::from(ptr),
            clone,
            drop,
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

#[cfg(feature = "nightly")]
fn rc_into_raw_non_null<T: ?Sized>(rc: Rc<T>) -> NonNull<T> {
    Rc::into_raw_non_null(rc)
}
#[cfg(feature = "nightly")]
fn arc_into_raw_non_null<T: ?Sized>(arc: Arc<T>) -> NonNull<T> {
    Arc::into_raw_non_null(arc)
}
#[cfg(not(feature = "nightly"))]
fn rc_into_raw_non_null<T: ?Sized>(rc: Rc<T>) -> NonNull<T> {
    unsafe { NonNull::new_unchecked(Rc::into_raw(rc) as *mut _) }
}
#[cfg(not(feature = "nightly"))]
fn arc_into_raw_non_null<T: ?Sized>(arc: Arc<T>) -> NonNull<T> {
    unsafe { NonNull::new_unchecked(Arc::into_raw(arc) as *mut _) }
}
