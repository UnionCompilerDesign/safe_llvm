use std::sync::{Arc, Mutex};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct CPointer<T> {
    ptr: Arc<Mutex<NonNull<T>>>,
    _marker: PhantomData<T>,
}

impl<T> CPointer<T> {
    pub fn new(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| CPointer {
            ptr: Arc::new(Mutex::new(ptr)),
            _marker: PhantomData,
        })
    }

    pub fn lock(&self) -> std::sync::MutexGuard<'_, NonNull<T>> {
        self.ptr.lock().expect("Mutex has been poisoned")
    }

    pub fn use_ref<R, F: FnOnce(*mut T) -> R>(&self, func: F) -> R {
        let guard = self.ptr.lock().expect("Mutex has been poisoned");
        let raw_ptr = guard.as_ptr(); 
        func(raw_ptr) 
    }
}

impl<T> Clone for CPointer<T> {
    fn clone(&self) -> Self {
        CPointer {
            ptr: Arc::clone(&self.ptr),
            _marker: PhantomData,
        }
    }
}

unsafe impl<T> Send for CPointer<T> {}
unsafe impl<T> Sync for CPointer<T> {}
    