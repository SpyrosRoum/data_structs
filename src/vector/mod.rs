#[cfg(test)]
mod tests;

use std::{alloc, mem, ops, ptr::NonNull, slice};

pub struct Vector<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, item: T) {
        if mem::size_of::<T>() == 0 {
            panic!("Zero sized T")
        }
        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4).unwrap();
            // layout is 4 * size_of::<T> and size_of::<T> > 0
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            self.ptr = NonNull::new(ptr).unwrap();
            unsafe { self.ptr.as_ptr().write(item) }
            self.capacity = 4;
            self.len = 1;
        } else {
            if self.len >= self.capacity {
                let new_cap = self.capacity.checked_mul(2).unwrap();
                let layout = alloc::Layout::array::<T>(new_cap).unwrap();
                let new_ptr = unsafe { alloc::alloc(layout) } as *mut T;
                let new_ptr = NonNull::new(new_ptr).unwrap();
                unsafe { self.ptr.as_ptr().copy_to(new_ptr.as_ptr(), self.len) };
                self.ptr = new_ptr;
                self.capacity = new_cap;
            }
            unsafe { self.ptr.as_ptr().add(self.len).write(item) }
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                Some(self.ptr.as_ptr().add(self.len).read())
            }
        }
    }
}

impl<T> ops::Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> ops::DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T, I> ops::Index<I> for Vector<T>
where
    I: slice::SliceIndex<[T]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        ops::Index::index(&**self, index)
    }
}

impl<T, I> ops::IndexMut<I> for Vector<T>
where
    I: slice::SliceIndex<[T]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        ops::IndexMut::index_mut(&mut **self, index)
    }
}
