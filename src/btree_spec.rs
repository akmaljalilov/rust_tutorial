use std::any::Any;
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::fmt::Debug;
use std::rc::Rc;


type Parent<D> = Option<Rc<Node<D>>>;

struct Node<D> {
    parent: Parent<D>,
    children: RefCell<BTreeMap<String, Rc<Node<D>>>>,
    data: D,
    id: RefCell<String>,
}

impl<D> Node<D> where D: 'static + Debug {
    fn new<T: Into<Option<Rc<Node<D>>>>>(parent: T, children: RefCell<BTreeMap<String, Rc<Node<D>>>>, data: D) -> Rc<Node<D>> {
        // let mut b_tree: BTreeMap<String, Rc<Node<String>>> = BTreeMap::new();
        Rc::new(Node {
            parent: parent.into(),
            children,
            data,
            id: RefCell::new("".to_string()),
        })
    }

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


    fn level(&self, leaf: &Rc<Node<D>>) -> i32 {
        let mut level = 0;
        let mut parent = leaf.parent.clone();
        while parent.is_some() {
            let current_parent = parent.unwrap();
            if current_parent.id == self.id {
                break;
            }
            parent = current_parent.parent.clone();
            level += 1;
        }
        return level;
    }

    fn depth(&self) -> i32 {
        let leaves = self.get_leaves();
        let mut max_level = 0;
        for (_, leaf) in leaves.iter() {
            max_level = max_level.max(self.level(&leaf))
        }

        return max_level;
    }

    fn is_leaf(&self) -> bool {
        self.children.borrow().is_empty()
    }

    fn get_leaves(&self) -> BTreeMap<String, Rc<Node<D>>> {
        let mut leaves: BTreeMap<String, Rc<Node<D>>> = Default::default();
        self.leaves(&mut leaves);
        return leaves;
    }

    fn leaves(&self, leaves: &mut BTreeMap<String, Rc<Node<D>>>) {
        for (key, child) in self.children.borrow().iter() {
            if child.is_leaf() {
                leaves.insert(key.clone(), Rc::clone(child));
            }
            child.leaves(leaves);
        }
    }
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
}


#[test]
fn test1() {
    let harry = Node::new(None, RefCell::new(BTreeMap::new()), "Harry".to_string());
    let _bill = harry.insert_child("bill".to_string());
    let jane = harry.insert_child("jane".to_string());
    jane.insert_child("mark".to_string());
    jane.insert_child("diane".to_string());
    harry.insert_child("mary".to_string());
    // let mut level = 0;
    // print_tree(&mut harry, "", &mut level);
    // let level = harry.depth();
    // print!("{}", level);
    harry.level_order_travels();
}

/*fn print_tree(node: &Rc<Node<String>>, tab: &str, level: &mut i32) {
    println!("BEFORE");
    *level += 1;
    println!("level {}{}{}", level, tab, node.data);
    for (_, tree) in node.children.borrow_mut().iter() {
        let mut data = "No parent";
        let parent = node.parent.as_ref();
        if parent.is_some() {
            data = &parent.unwrap().data;
        }
        println!("INSIDE {}", data);
        let tabs = format!("{}\t", tab);

        print_tree(tree, tabs.as_str(), level);
    }
    print!("");
}*/
