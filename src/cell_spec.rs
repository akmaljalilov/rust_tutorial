use std::cell::{Cell, Ref, RefCell};
use std::rc::Rc;

#[derive(Debug)]
struct MyStruct {
    value: u8
}

#[derive(Debug)]
struct MyStructWithOutCell {
    value: u8
}


#[test]
fn test1() {
    // let my_struct = MyStruct { value: Cell::new(10) };
    // my_struct.value = 50;
    // println!("{:?}", my_struct.value.take());
    // println!("{:?}", my_struct.value);
    //
    // let mut my_struct = MyStructWithOutCell { value: 10 };
    // my_struct.value = 50;
    // println!("{:?}", my_struct.value);
}

#[test]
fn test2() {
    #[derive(Debug)]
    struct MyStructRCell {
        value: RefCell<i32>
    }
    let my_struct = MyStructRCell { value: RefCell::new(10) };
    let mut value = my_struct.value.borrow_mut();
    *value = 10;
    // let value1 = my_struct.value.borrow_mut();
    print!("{:#?}", value);
}