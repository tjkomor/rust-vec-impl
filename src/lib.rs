use std::ptr::NonNull;
use std::alloc;
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
        } else {
            todo!()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    
    fn it_works() {
        let mut vec: MyVec<usize> = MyVec::<usize>::new();
        vec.push()
        // vec.push(1usize); //first element will imply type for all element in vec
        // vec.push(2);
        // vec.push(3);
        // vec.push(4);
        // vec.push(5);

        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
    }
}
