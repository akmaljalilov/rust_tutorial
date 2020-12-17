use std::collections::VecDeque;

#[test]
fn test() {
    struct Node {
        key: u32,
        child: VecDeque<Node>,
    }
    fn new_node(key: u32) -> Node {
        Node {
            key,
            child: VecDeque::new(),
        }
    }

    fn level_order_travels(root: Node) {
        let mut vec_deq = VecDeque::<Node>::new();
        vec_deq.push_back(root);

        while !vec_deq.is_empty() {
            let mut n = vec_deq.len();
            while n > 0 {
                let p = vec_deq.pop_front().unwrap();
                print!("{} ", p.key);
                for child in p.child {
                    vec_deq.push_back(child)
                }
                n -= 1;
            }
            println!();
        }
    }

    let mut node = new_node(10);
    node.child.push_back(new_node(2));
    node.child.push_back(new_node(34));
    node.child.push_back(new_node(56));
    node.child.push_back(new_node(100));
    node.child[0].child.push_back(new_node(77));
    node.child[0].child.push_back(new_node(88));
    node.child[2].child.push_back(new_node(1));
    node.child[3].child.push_back(new_node(7));
    node.child[3].child.push_back(new_node(8));
    node.child[3].child.push_back(new_node(9));
    level_order_travels(node);
}