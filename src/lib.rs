use std::ptr::NonNull;
use std::alloc::{self};
pub struct MyVec<T> {
    pointer: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl <T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            capacity: 0,
            pointer: NonNull::dangling(), //when capacity is zero, pointer will be dangling -> if cap == 0 do not use ptr
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(std::mem::size_of::<T>(), 0, "no zero sized types");
        if self.capacity == 0 {
            
            let layout = alloc::Layout::array::<T>(4).expect("could not allocate");
            // the layout is hardcoded to be 4x size of <T> 
            
            let pointer = unsafe { alloc::alloc(layout) } as *mut T;
            let pointer: NonNull<T> = NonNull::new(pointer).expect("could not allocate memory");
            
            unsafe { pointer.as_ptr().write(item) };
            // SAFETY: pointer is nonnull and have allocated enough space this item and 3 more
            // the memory previoiusly at ptr in not read
            
            self.pointer = pointer;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            let offset: usize = self.len.checked_mul(std::mem::size_of::<T>()).expect("cannot reach memory location");
            assert!(offset < isize::MAX as usize, "wrapped isize");
            // offset cannot wrap around and pointer is pointing to valid memory
            // and writing to an offset at self.len is valid

            unsafe { 
                self.pointer.as_ptr().add(self.len).write(item);
                self.len += 1;
            }
        } else {
            debug_assert!(self.len == self.capacity); // len should always eq capacity if this block is reached
            let new_capacity = self.capacity.checked_mul(2).expect("capacity wrapped");
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_mul(size % align).expect("Can't allocate");
            let ptr = unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size: usize = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(self.pointer.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("could not reallocate");
                ptr.as_ptr().add(self.len).write(item);
                ptr
            };
            self.pointer = ptr;
            self.len += 1;
            self.capacity = new_capacity;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { &*self.pointer.as_ptr().add(index) })
    }

}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe { 
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.pointer.as_ptr(), self.len));
        
            let layout = alloc::Layout::from_size_align_unchecked(
                std::mem::size_of::<T>() * self.capacity, 
                std::mem::align_of::<T>(),
            );
            
            alloc::dealloc(self.pointer.as_ptr() as *mut u8, layout)
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    
    fn it_works() {
        let mut vec: MyVec<usize> = MyVec::<usize>::new();
        vec.push(1usize); //first element will imply type for all element in vec
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        for n in 0..vec.len() {
            assert_eq!(vec.get(n), Some(&(n + 1)));
        }

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }
}
