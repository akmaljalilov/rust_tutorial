/*mod structs;
mod tree;
mod config;
*/
#[cfg(test)]
mod tests {
    // use crate::structs::{Config, Node, TreeLib};

    #[test]
    fn it_works() {
        let x = Box::new("Salom");
        let static_ref: &'static mut &str = Box::leak(x);
        *static_ref = "Hi";
        assert_eq!(*static_ref, "Hi");
    }

    #[test]
    fn spec_tree_lib() {
        // let mut tree = TreeLib::new(None, false, None);
        // tree.create_node("1", "", None, Config {})
    }
}
