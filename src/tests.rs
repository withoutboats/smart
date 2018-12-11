#![cfg(test)]

use super::*;

use std::rc::Rc;
use std::sync::Arc;

static X: i32 = 0;

// All of these clone to make sure we are testing for faults in both their clone and drop impls

#[test]
fn construct_shared_from_static() {
    assert_eq!(*SharedPointer::from(&X).clone(), 0);
}

#[test]
fn construct_shared_from_rc() {
    assert_eq!(*SharedPointer::from(Rc::new(0)).clone(), 0);
}

#[test]
fn construct_shared_from_arc() {
    assert_eq!(*SharedPointer::from(Arc::new(0)).clone(), 0);
}

#[test]
fn construct_sync_from_static() {
    assert_eq!(*SyncPointer::from(&X).clone(), 0);
}

#[test]
fn construct_sync_from_arc() {
    assert_eq!(*SyncPointer::from(Arc::new(0)).clone(), 0);
}

#[test]
fn convert_from_static() {
    let shared_ptr = SharedPointer::from(&X);
    let sync_ptr = unsafe { SyncPointer::from_shared(shared_ptr) };
    assert_eq!(*sync_ptr, 0);
}

#[test]
#[should_panic]
fn convert_from_rc_panics() {
    let shared_ptr = SharedPointer::from(Rc::new(0));
    unsafe { SyncPointer::from_shared(shared_ptr) };
}

#[test]
fn convert_from_arc() {
    let shared_ptr = SharedPointer::from(Arc::new(0));
    let sync_ptr = unsafe { SyncPointer::from_shared(shared_ptr) };
    assert_eq!(*sync_ptr, 0);
}
