extern crate llvm_sys as llvm;

use std::{marker::PhantomData, ptr};

#[derive(Debug, Clone)]
pub struct IRPointer<T> {
    ptr: *mut T, 
    _marker: PhantomData<T>, 
}

impl<T> IRPointer<T> {
    pub fn new(ptr: *mut T) -> Self {
        IRPointer { ptr, _marker: PhantomData }
    }

    pub fn as_ref(&self) -> *mut T {
        self.ptr 
    }
}

impl<T> Drop for IRPointer<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _dropped = Box::from_raw(self.ptr);
            }
            self.ptr = ptr::null_mut(); 
        }
    }
}