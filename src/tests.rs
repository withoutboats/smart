#![cfg(test)]

use super::*;

use std::convert::TryFrom;
use std::panic;
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
    assert_eq!(*SyncPointer::from(SharedPointer::from(&X)), 0);
}

#[test]
#[should_panic]
fn convert_from_rc_panics() {
    SyncPointer::from(SharedPointer::from(Rc::new(0)));
}

#[test]
fn convert_from_arc() {
    assert_eq!(*SyncPointer::from(SharedPointer::from(Arc::new(0))), 0);
}

#[test]
fn try_convert_from_static() {
    assert_eq!(
        SyncPointer::try_from(SharedPointer::from(&X)).map(|x| *x),
        Ok(0)
    );
}

#[test]
fn try_convert_from_rc_panics() {
    let ptr = SharedPointer::from(Rc::new(0));
    assert!(panic::catch_unwind(|| SyncPointer::try_from(ptr)).is_err())
}

#[test]
fn try_convert_from_arc() {
    assert_eq!(
        SyncPointer::try_from(SharedPointer::from(Arc::new(0))).map(|x| *x),
        Ok(0)
    );
}

#[test]
fn custom_try_convert_from_static() {
    assert_eq!(
        SyncPointer::try_from_shared(SharedPointer::from(&X)).map(|x| *x),
        Ok(0)
    );
}

#[test]
fn custom_try_convert_from_rc_errors() {
    assert_eq!(
        SyncPointer::try_from_shared(SharedPointer::from(Rc::new(0))).map(|x| *x),
        Err("Cannot upgrade non-threadsafe SharedPointer to SyncPointer")
    );
}

#[test]
fn custom_try_convert_from_arc() {
    assert_eq!(
        SyncPointer::try_from_shared(SharedPointer::from(Arc::new(0))).map(|x| *x),
        Ok(0)
    );
}

#[test]
fn custom_try_convert_static() {
    assert_eq!(SharedPointer::from(&X).try_into_sync().map(|x| *x), Ok(0));
}

#[test]
fn custom_try_convert_rc_errors() {
    assert_eq!(
        SharedPointer::from(Rc::new(0)).try_into_sync().map(|x| *x),
        Err("Cannot upgrade non-threadsafe SharedPointer to SyncPointer")
    );
}

#[test]
fn custom_try_convert_arc() {
    assert_eq!(
        SharedPointer::from(Arc::new(0)).try_into_sync().map(|x| *x),
        Ok(0)
    );
}
