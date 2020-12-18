use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

struct Owner {
    name: RefCell<String>,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

/*#[test]
fn test() {
    let owner = Rc::new((Owner { name: "A".to_string(), gadgets: RefCell::new(vec![]) }));
    let gadget_1 = Gadget { owner: Rc::clone(&owner), id: 1 };
    let gadget_2 = Gadget { owner: Rc::clone(&owner), id: 2 };
    println!("owner {}", owner.name);
    drop(owner);
    println!("Gadget {} owned by {}", gadget_1.id, gadget_1.owner.name);
    println!("Gadget {} owned by {}", gadget_2.id, gadget_2.owner.name);
}*/

#[test]
fn test() {
    let gadget_owner = Rc::new(Owner { name: RefCell::new("A".to_string()), gadgets: RefCell::new(vec![]) });
    let gadget1 = Rc::new(Gadget { owner: Rc::clone(&gadget_owner), id: 1 });
    let gadget2 = Rc::new(Gadget { owner: Rc::clone(&gadget_owner), id: 2 });
    {
        let mut gadgets = gadget_owner.gadgets.borrow_mut();
        gadgets.push(Rc::downgrade(&gadget1));
        gadgets.push(Rc::downgrade(&gadget2));

        // `RefCell` dynamic borrow ends here.
    }

    // Iterate over our `Gadget`s, printing their details out.
    *gadget_owner.name.borrow_mut() = "B".to_string();
    for gadget_weak in gadget_owner.gadgets.borrow().iter() {

        // `gadget_weak` is a `Weak<Gadget>`. Since `Weak` pointers can't
        // guarantee the allocation still exists, we need to call
        // `upgrade`, which returns an `Option<Rc<Gadget>>`.
        //
        // In this case we know the allocation still exists, so we simply
        // `unwrap` the `Option`. In a more complicated program, you might
        // need graceful error handling for a `None` result.

        let gadget = gadget_weak.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name.to_owned().into_inner());
    }
}


#[test]
fn test2() {
    #[derive(Debug)]
    struct MyStruct {
        value: RefCell<i32>
    }
    let my_struct = MyStruct { value: RefCell::new(5) };
    let rc = Rc::new(&my_struct);
    let owner1_rc = rc.clone();
    let owner2_rc = rc.clone();
    *owner1_rc.value.borrow_mut() = 10;
    assert_eq!(10, *my_struct.value.borrow());
    *owner2_rc.value.borrow_mut() = 25;
    assert_eq!(25, *my_struct.value.borrow());
}

#[test]
fn test3() {
    #[derive(Debug)]
    struct MyStruct {
        value: RefCell<i32>
    }
    let my_struct = MyStruct { value: RefCell::new(5) };
    let owner1 = &my_struct;
    let owner2 = &my_struct;
    *owner1.value.borrow_mut() = 10;
    assert_eq!(*my_struct.value.borrow(), 10);
    *owner2.value.borrow_mut() = 25;
    assert_eq!(*my_struct.value.borrow(), 25);
}

#[test]
fn test4() {
    let x = Cell::new(1);
    let y = &x;
    let z = y;
    x.set(2);
    y.set(3);
    z.set(4);
    print!("{}", x.get());
}

#[test]
fn test5() {

}
