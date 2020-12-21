#![allow(dead_code)]
use std::any::Any;
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::fmt::Debug;
use std::rc::Rc;

/// Type Parent
type Parent<D> = Option<Rc<Node<D>>>;

/// Struct Node
struct Node<D> {
    parent: Parent<D>,
    children: RefCell<BTreeMap<String, Rc<Node<D>>>>,
    data: D,
    id: RefCell<String>,
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl<D> Node<D> where D: 'static + Debug {
    /// Makes a new, empty `Node`.
   ///
   ///
   /// # Examples
   ///
   /// Basic usage:
   ///
   /// ```
   ///
   /// let mut node = Node::new("root".to_string());
   ///
   /// // entries can now be inserted into the empty node
   /// node.insert_child("mark".to_string());
   /// ```
    fn new(data: D) -> Rc<Node<D>> {
        // let mut b_tree: BTreeMap<String, Rc<Node<String>>> = BTreeMap::new();
        Rc::new(Node {
            parent: None,
            data,
            id: RefCell::new("".to_string()),
            children: RefCell::new(Default::default()),
        })
    }

    /// Insert child for node.
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    ///
    /// let mut node = Node::new("root".to_string());
    ///
    /// // entries can now be inserted into the empty node
    /// node.insert_child("mark".to_string());
    /// assert_eq!(mark.data, "mark".to_string());
    /// assert_eq!(mark.parent.as_ref().unwrap().data, "root".to_string());
    /// ```

    fn insert_child(self: &Rc<Node<D>>, data: D) -> Rc<Node<D>> {
        let node = Rc::new(Node {
            parent: Some(Rc::clone(self)),
            children: RefCell::new(Default::default()),
            data,
            id: RefCell::new("".to_string()),
        });
        let str = format!("{:?}{:?}", node.type_id(), node.data); //todo
        self.children.borrow_mut().insert(str.to_string(), Rc::clone(&node));
        *node.id.borrow_mut() = self.children.borrow_mut().last_entry().unwrap().key().to_string();
        // *node.parent.borrow_mut() = ;
        return Rc::clone(&node);
    }

    /// Get level node.
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    ///
    /// let mut node = Node::new("root".to_string());
    ///
    /// let mark = node.insert_child("mark".to_string());
    /// let level = node.level(&mark);
    /// assert_eq!(level, 1);
    /// ```

    fn level(&self, leaf: &Rc<Node<D>>) -> i32 {
        let mut level = 0;
        let mut parent = leaf.parent.clone();
        while parent.is_some() {
            let current_parent = parent.unwrap();
            level += 1;
            if current_parent.id == self.id {
                break;
            }
            parent = current_parent.parent.clone();
        }
        return level;
    }
    /// Get depth node.
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    ///
    /// let mut node = Node::new("root".to_string());
    ///
    /// let b = node.insert_child("a".to_string())
    ///             .insert_child("b".to_string());
    /// let depth = node.depth(None);
    /// assert_eq!(depth, 2);
    /// ```

    fn depth<T: Into<Option<Rc<Node<D>>>>>(self: &Rc<Node<D>>, node: T) -> i32 {
        if let Some(node) = node.into() {
            return self.level(&Rc::clone(&node));
        }
        let leaves = self.get_leaves();
        let mut max_level = 0;
        for (_, leaf) in leaves.iter() {
            max_level = max_level.max(self.level(&leaf))
        }

        return max_level;
    }
     ///
     ///
     /// if there are no children in the node, then it returns true, otherwise it returns false.
     ///
     /// # Examples
     ///
     /// ```
     /// let mut node = Node::new("root".to_string());
     ///
     /// let a = node.insert_child("a".to_string());
     /// let b = a.insert_child("b".to_string());
     /// assert_eq!(a.is_leaf(), false);
     /// assert_eq!(b.is_leaf(), true);
     /// ```
    fn is_leaf(&self) -> bool {
        self.children.borrow().is_empty()
    }

    /// Get a node of leaves
    ///
    /// Returns the leaves of the node.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut node = Node::new("root".to_string());
    ///
    /// let a = node.insert_child("a".to_string());
    /// let b = a.insert_child("b".to_string());
    /// assert_eq!(a.is_leaf(), false);
    /// assert_eq!(b.is_leaf(), true);
    /// ```
    fn get_leaves(&self) -> BTreeMap<String, Rc<Node<D>>> {
        let mut leaves: BTreeMap<String, Rc<Node<D>>> = Default::default();
        self.set_leaves(&mut leaves);
        return leaves;
    }

    ///
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// let mut node = Node::new("root".to_string());
    ///
    /// let a = node.insert_child("a".to_string());
    /// let b = a.insert_child("b".to_string());
    /// let mut leaves = Default::default();
    /// node.set_leaves(&mut leaves);
    /// assert_eq!(leaves.len(), 1);
    /// ```
    fn set_leaves(&self, leaves: &mut BTreeMap<String, Rc<Node<D>>>) {
        for (key, child) in self.children.borrow().iter() {
            if child.is_leaf() {
                leaves.insert(key.clone(), Rc::clone(child));
            }
            child.set_leaves(leaves);
        }
    }
    /// Level order travels
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// let mut node = Node::new("root".to_string());
    ///
    /// let harry = Node::new("Harry".to_string());
    /// let _bill = harry.insert_child("bill".to_string());
    /// let jane = harry.insert_child("jane".to_string());
    /// jane.insert_child("mark".to_string());
    /// jane.insert_child("diane".to_string());
    /// harry.level_order_travels();
    ///
    /// // "Harry"
    /// // "bill" "jane"
    /// // "diane" "mark"
    ///
    /// ```
    fn level_order_travels(self: &Rc<Node<D>>) {
        let mut vec_deq = VecDeque::<Rc<Node<D>>>::new();
        vec_deq.push_back(Rc::clone(self));
        let mut cur_parent: Option<Rc<Node<D>>> = None;
        while !vec_deq.is_empty() {
            let mut n = vec_deq.len();
            if cur_parent.is_some() {
                // print!("Parent: {:?} Children: ", cur_parent.borrow_mut().as_ref().unwrap().data);
            }
            while n > 0 {
                let p = vec_deq.pop_front().unwrap();
                print!("{:?} ", p.data);
                for (_, child) in p.children.borrow_mut().iter() {
                    cur_parent = Some(Rc::clone(&p));
                    vec_deq.push_back(Rc::clone(child));
                }
                n -= 1;
            }
            println!()
        }
    }
    /// Shows the whole node
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// let mut node = Node::new("root".to_string());
    ///
    /// let harry = Node::new("Harry".to_string());
    /// let _bill = harry.insert_child("bill".to_string());
    /// let jane = harry.insert_child("jane".to_string());
    /// jane.insert_child("mark".to_string());
    /// jane.insert_child("diane".to_string());
    /// harry.level_order_travels();
    ///
    /// // "Harry"
    /// 	// "bill"
    /// 	//	"jane"
    /// 	//		"diane"
    /// 	//		"mark"
    ///
    /// ```
    fn show(&self, tab: &str) {
        println!("{}{:?}", tab, self.data);
        for (_, tree) in self.children.borrow_mut().iter() {
            /*let mut data = "No parent";
            let parent = node.parent.as_ref();
            if parent.is_some() {
                data = &parent.unwrap().data;
            }*/
            let tabs = format!("{}\t", tab);
            tree.show(tabs.as_str());
        }
        print!("");
    }
}


#[test]
fn test() {
    let harry = Node::new("Harry".to_string());
    let _bill = harry.insert_child("bill".to_string());
    let jane = harry.insert_child("jane".to_string());
    jane.insert_child("mark".to_string());
    jane.insert_child("diane".to_string());
    // harry.show("\t");
    harry.level_order_travels();
}

