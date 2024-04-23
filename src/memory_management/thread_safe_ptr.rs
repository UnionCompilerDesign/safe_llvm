use std::sync::{Arc, Mutex};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct ThreadSafePtr<T> {
    ptr: Arc<Mutex<NonNull<T>>>,
    _marker: PhantomData<T>,
}

impl<T> ThreadSafePtr<T> {
    pub fn new(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| ThreadSafePtr {
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

impl<T> Clone for ThreadSafePtr<T> {
    fn clone(&self) -> Self {
        ThreadSafePtr {
            ptr: Arc::clone(&self.ptr),
            _marker: PhantomData,
        }
    }
}

unsafe impl<T> Send for ThreadSafePtr<T> {}
unsafe impl<T> Sync for ThreadSafePtr<T> {}
    