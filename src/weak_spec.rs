use std::rc::{Weak, Rc};

#[test]
fn test1() {
    let five  = Rc::new(5);
    let five_weak  = Rc::downgrade(&five);
    let strong_five = five_weak.upgrade();

}