use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct CPointer<T> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T> CPointer<T> {
    pub fn new(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| CPointer {
            ptr,
            _marker: PhantomData,
        })
    }

    pub fn get_ref(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    pub fn is_null(&self) -> bool {
        false 
    }
}

impl<T> Clone for CPointer<T> {
    fn clone(&self) -> Self {
        CPointer {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for CPointer<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.ptr.as_ptr());
            std::alloc::dealloc(self.ptr.cast().as_ptr(), std::alloc::Layout::new::<T>());
        }
        
    }
}

unsafe impl<T> Send for CPointer<T> {}
unsafe impl<T> Sync for CPointer<T> {}
