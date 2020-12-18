#[test]
fn struct_1() {
    // struct MyStruct<T: ?Sized>(T);
    // struct Struct(MyStruct<[String]>);
}

#[test]
fn str_2() {
    /*trait MyTrait {}
    trait MyTraitTwo: Sized {}

    struct MyStruct();
    impl MyTrait for MyStruct {}
    impl MyTraitTwo for MyStruct {}*/

    // let my_str: &dyn MyTrait = &MyStruct();
    // let my_str: &dyn MyTraitTwo = &MyStruct();
}

