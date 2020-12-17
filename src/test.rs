use core::fmt;
use std::marker::PhantomData;

struct MyStruct<'a, T> {
    value: T,
    phantom: PhantomData<&'a T>,
}

impl<T: fmt::Debug> fmt::Debug for MyStruct<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("my_struct").field("value ", &self.value).finish()
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::test::MyStruct;

    #[test]
    fn debug_spec() {
        let my_struct = MyStruct { value: 10, phantom: PhantomData };
        let str = format!("{:#?}", my_struct);
        print!("{}", str);
    }

    #[test]
    /*fn option_spec() {
        ///  Take
        let mut some = Some(4);
        let new_some = some.take();
        assert_eq!(new_some, Some(4));
        assert_eq!(some, None);

        /// Contains
        let op = Some(5);
        assert_eq!(op.contains(&5), true);

        /// as_ref
        let op = Some("Hello Rust".to_string());
        let op_new = op.as_ref().map(|s| s.len());
        println!("{:?}", op);
        println!("{:?}", op_new);

        /// as_mut
        let mut op = Some("Hello Rust".to_string());
        match op.as_mut() {
            Some(value) => *value = "Hi, Akmal".to_string(),
            None => {}
        }
        println!("{:?}", op);

        /// expect
        let op: Option<&str> = None;
        // let res = op.expect("not value");

        /// unwrap_or_else
        let k = 4;
        assert_eq!(None.unwrap_or_else(|| 2 * k), 8);

        /// insert
        let mut op = None;
        let elm = op.insert(10);
        assert_eq!(*elm, 10);
        assert_eq!(op, Some(10));

        /// and or
        let op_true = Some("Salom");
        let op_false = Some("hi");
        let res = op_true.and(op_false);
        assert_eq!("hi", res.unwrap());
        let res = op_true.or(op_false);
        assert_eq!("Salom", res.unwrap());

        /// filter
        fn is_four(x: &i32) -> bool { *x == 4 }
        let op = Some(4);
        assert_eq!(op.filter(is_four), Some(4));
        assert_eq!(Some(5).filter(is_four), None);
        assert_eq!(None.filter(is_four), None);

        /// get_or_insert
        let mut op = None;
        let value = op.get_or_insert("Salom");
        assert_eq!(*value, "Salom");
        assert_eq!(op, Some("Salom"));

        /// replace
        let mut op = Some(10);
        let op_new = op.replace(5);
        assert_eq!(op, Some(5));
        assert_eq!(op_new, Some(10));

        /// zip
        let op = Some(10);
        let op_hello = Some("hello");
        let op_new = op.zip(op_hello);
        print!("{:?}", op_new);

        /// zip_with
        #[derive(Debug, PartialEq)]
        struct Point {
            x: f64,
            y: f64,
        }
        impl Point {
            fn new(x: f64, y: f64) -> Self {
                Self { x, y }
            }
        }

        let x = Some(10.5);
        let y = Some(10.6);
        let new_op = x.zip_with(y, Point::new);
        assert_eq!(Some(Point { x: 10.5, y: 10.6 }), new_op);
    }*/
    #[test]
    fn box_test() {
        let x = Box::new(10);
        // println!("{}", std::mem::size_of_val(&x));
        // println!("{}", std::mem::size_of_val(&10));

        let mut five = Box::<u32>::new_uninit();
        let value = five.write(10);
        let five_value = unsafe {
            five.assume_init() //Todo
        };
        println!("{}", five_value);

        // new_uninit_slice
        let mut box_un_init_slice = Box::<[i32]>::new_uninit_slice(3);
        let box_un_init_slice = unsafe {
            box_un_init_slice[0].as_mut_ptr().write(10);

            box_un_init_slice.assume_init()
        };
        assert_eq!(*box_un_init_slice, [10, 0, 0]);

        //raw_pointer
        let mut box_new = Box::new(10);
        let raw = unsafe { Box::into_raw(box_new) };
        let box_new = unsafe { Box::from_raw(raw) };
        print!("0");
    }

    #[test]
    fn new_zeroed_box() {
        let b = Box::<[i32]>::new_zeroed_slice(3);
        let b = unsafe{b.assume_init()};
        let s = std::mem::size_of_val(&b);
        assert_eq!(*b, [0, 0, 0]);
        print!("s");
    }

    #[test]
    fn bin_spec() {
        assert_eq!(5, 0b101);
    }
}