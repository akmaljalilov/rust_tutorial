#![feature(nonnull_slice_from_raw_parts)]
#![feature(option_result_contains)]
#![feature(option_insert)]
#![feature(option_zip)]
#![feature(new_uninit)]
#![feature(maybe_uninit_extra)]
#![feature(allocator_api)]
#![feature(map_first_last)]
/*mod structs;
mod tree;
mod config;
*/
mod test;
mod alloc_spec;
mod struct_spec;
mod rc_spec;
mod cell_spec;
mod weak_spec;
mod general_tree;
mod btree_spec;

#[cfg(test)]
mod tests {
    // use crate::structs::{Config, Node, TreeLib};

    use std::collections::LinkedList;
    use std::ptr::NonNull;

    #[test]
    fn box_leak() {
        let x = Box::new("Salom");
        let static_ref: &'static mut &str = Box::leak(x);
        *static_ref = "Hi";
        assert_eq!(*static_ref, "Hi");
    }

    #[test]
    fn non_null() {
        let mut x = [5, 6, 7];
        let x_mut_ptr = x.as_mut_ptr();
        let nonnull_pointer = NonNull::new(x_mut_ptr).unwrap();
        let slice = NonNull::slice_from_raw_parts(nonnull_pointer, 3);
        assert_eq!(unsafe { slice.as_ref()[2] }, 7);
    }

    #[test]
    fn linked_list() {
        let mut list = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);
        list.push_back(5);
        list.push_back(2);

    }

    #[test]
    fn spec_tree_lib() {
        // let mut tree = TreeLib::new(None, false, None);
        // tree.create_node("1", "", None, Config {})
    }
}
