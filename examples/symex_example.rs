//! Test for [`vcell`] using [`symex_lib`]
extern crate symex_lib;
use symex_lib::{any, assume};
extern crate vcell;
/// Symbolic execution test
pub fn test_vcell() -> u8 {
    use vcell::VolatileCell;
    let value = any::<u8>();
    assume(value <= 15);
    let cell = VolatileCell::new(value);
    let mut sum: u8 = any::<u8>();
    for _i in 0..*cell.borrow() {
        //let a: u8 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        sum += any::<u8>();
    }
    // Use the vcell api to set the value, similar to what embedded code does
    cell.set(11);
    assume(sum < u8::MAX / 2);
    for _i in 0..*cell.borrow() {
        sum -= 1;
    }
    sum
}

fn main() {
    test_vcell();
}
