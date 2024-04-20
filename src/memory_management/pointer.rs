use std::sync::{Arc, RwLock, atomic::{AtomicUsize, Ordering}};
use std::{marker::PhantomData, ptr};

#[derive(Debug)]
pub struct CPointer<T> {
    ptr: Arc<RwLock<*mut T>>,
    ownership_count: Arc<AtomicUsize>,
    _ownership_marker: PhantomData<T>,
}

impl<T> CPointer<T> {
    pub fn new(ptr: Option<*mut T>) -> Self {
        let new_ptr = CPointer { 
            ptr: Arc::new(RwLock::new(ptr.unwrap_or(ptr::null_mut()))),
            ownership_count: Arc::new(AtomicUsize::new(1)), 
            _ownership_marker: PhantomData,
        };
        if ptr.is_some() {
            new_ptr.ownership_count.store(1, Ordering::SeqCst);
        }
        new_ptr
    }

    pub fn get_ref(&self) -> *mut T {
        *self.ptr.read().unwrap()
    }

    pub fn set_ref(&self, new_ptr: *mut T) {
        let mut ptr_guard = self.ptr.write().unwrap();
        *ptr_guard = new_ptr;
    }

    pub fn is_null(&self) -> bool {
        *self.ptr.read().unwrap() == ptr::null_mut()
    }
}

impl<T> Clone for CPointer<T> {
    fn clone(&self) -> Self {
        self.ownership_count.fetch_add(1, Ordering::SeqCst); 
        CPointer {
            ptr: Arc::clone(&self.ptr),
            ownership_count: Arc::clone(&self.ownership_count),
            _ownership_marker: PhantomData,
        }
    }
}

impl<T> Drop for CPointer<T> {
    fn drop(&mut self) {
        if self.ownership_count.fetch_sub(1, Ordering::SeqCst) == 1 {
            let mut ptr = self.ptr.write().unwrap();
            if !(*ptr).is_null() {
                unsafe {
                    let _dropped = Box::from_raw(*ptr);
                }
                *ptr = ptr::null_mut();
            }
        }
    }
}

unsafe impl<T> Send for CPointer<T> {}
unsafe impl<T> Sync for CPointer<T> {}