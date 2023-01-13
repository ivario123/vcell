//! Just like [`Cell`] but with [volatile] read / write operations
//!
//! [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
//! [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
#![no_std]
#![allow(dead_code, unused_variables, unused_unsafe, invalid_value)]
#![deny(missing_docs)]
#![deny(warnings)]

extern crate symex_lib;
use symex_lib::assume;

/// Just like [`Cell`] but with [volatile] read / write operations
///
/// [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
/// [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
#[repr(transparent)]
pub struct VolatileCell<T: symex_lib::Any + PartialEq> {
    value: T, // There is no point pretending we have a pointer, lets just store the value
}
/// Wrapper type to make return types prettier
pub type Ptr<T> = *mut T;

impl<T: symex_lib::Any + PartialEq> VolatileCell<T> {
    /// Creates a new `VolatileCell` containing the given value
    pub fn new(value: T) -> Self {
        let ret = VolatileCell { value: value };
        //symbolic(&mut ret.value);
        ret
    }

    /// Returns a copy of the contained value
    #[inline(always)]
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        self.value
    }

    /// Borrows the value for a bit
    pub fn borrow(&self) -> &T {
        &self.value
    }

    /// Borrows the value as mutable for a bit
    pub fn borrow_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Sets the contained value
    #[inline(always)]
    pub fn set(&self, value: T)
    where
        T: Copy + PartialEq,
    {
        // We need to discard the old value otherwise we get unsolvable conditions.
        symex_lib::symbolic(&self.value);
        assume(self.value == value); // Assume that the value now is the value, this is a replacement for writing to the pointer location
    }

    /// Returns a raw pointer to the underlying data in the cell
    #[inline(always)]
    pub fn as_ptr(&self) -> Ptr<T>
    where
        T: symex_lib::Any,
    {
        let symbolic_value = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        symex_lib::symbolic(&symbolic_value); // We have no idea what the value actually is. In the ideal case this should pass a ptr to the value held by the struct
        symbolic_value
    }
}

// NOTE implicit because of `UnsafeCell`
// unsafe impl<T> !Sync for VolatileCell<T> {}
