extern crate llvm_sys as llvm;

use std::{marker::PhantomData, ptr, sync::RwLock};

#[derive(Debug)]
pub struct IRPointer<T> {
    ptr: RwLock<*mut T>, 
    _marker: PhantomData<T>, 
}

impl<T> IRPointer<T> {
    pub fn new(ptr: Option<*mut T>) -> Self {
        IRPointer { 
            ptr: RwLock::new(ptr.unwrap_or(ptr::null_mut())),
            _marker: PhantomData,
        }
    }

    pub fn get_ref(&self) -> *mut T {
        *self.ptr.read().unwrap()
    }

    pub fn set_ref(&self, new_ptr: *mut T) {
        let mut ptr_guard = self.ptr.write().unwrap(); // Safely write to the pointer with write lock.
        *ptr_guard = new_ptr;
    }

    pub fn is_null(&self) -> bool {
        *self.ptr.read().unwrap() == ptr::null_mut()
    }
}

impl<T> Drop for IRPointer<T> {
    fn drop(&mut self) {
        let mut ptr = self.ptr.write().unwrap();
        if !(*ptr).is_null() {
            unsafe {
                let _dropped = Box::from_raw(*ptr);
            }
            *ptr = ptr::null_mut(); 
        }
    }
}


unsafe impl<T> Send for IRPointer<T> {}