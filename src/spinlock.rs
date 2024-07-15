use core::cell::UnsafeCell;
use core::sync::atomic::{Ordering, AtomicBool};
use core::ops::{Deref, DerefMut};

pub struct LockGuard<'a, T> {
    lock: &'a SpinLock<T>
}

impl<T> Deref for LockGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for LockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for LockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
    cpu: UnsafeCell<u32>
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl <T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
                locked: AtomicBool::new(false),
                value: UnsafeCell::new(value),
                cpu: UnsafeCell::new(0)
            }
    }

    pub fn aquire(&self) -> LockGuard<T> {
        while self.locked.compare_exchange(
                false,
                true,
                Ordering::Acquire,
                Ordering::SeqCst
            ).is_err() {}
            LockGuard { lock: self }
    }

    pub fn release(&self) {
        unsafe {if *self.cpu.get() == 0 {
            panic!("Trying to release lock aquired by another thread!");
        }}
        self.locked.store(false, Ordering::Release);
    }
}
