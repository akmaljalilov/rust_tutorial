use std::collections::HashMap;

use crate::structs::{Config, Data, Node, TreeLib};
use crate::tree::node::ADD;

pub type TypeTreeLib = TreeLib;

const DEPTH: i32 = 2;
const WIDTH: i32 = 3;
const ZIGZAG: i32 = 4;

impl TreeLib {
    /// New tree or copy another tree with a shallow or deep copy
    pub fn new<T: Into<Option<TypeTreeLib>>, B: Into<Option<Node>>>(mut tree: T, node_class_option: B, deep: bool) -> Self {
        let Some(mut node_class) = node_class_option.into();
        let mut root = String::new();
        let mut nodes = HashMap::new();
        if let Some(tree) = tree.into() {
            root = tree.root;
            if deep {
                for &key in tree.nodes.keys() {
                    let opt_value = tree.nodes.get(&key);
                    if opt_value.is_some() {
                        let &value = opt_value.unwrap();
                        nodes.insert(key.clone(), value);
                    }
                }
            } else {
                nodes = tree.nodes;
            }
        }
        if node_class_option.into().is_none() {
            node_class = Node::new("", None);
        }
        Self {
            node: node_class,
            nodes,
            root,
            identifier: "".to_string(),
        }
    }

    pub fn contains(&self, nid: &str) -> bool {
        self.nodes.get(nid).is_some()
    }

    pub fn add_nod(&mut self, parent_id: &str) {
        let ident_string = self.node.get_id();
        let ident = ident_string.as_str();
        if parent_id.len() == 0 {
            if self.root.len() == 0 {
                // TODO
            } else {
                self.root = ident.to_string();
            }
        } else if !self.contains(parent_id) {
            // TODO
        }
        self.nodes.insert(ident.to_string(), self.node.clone());
        self.update_children_id(parent_id, ident, ADD);
        self.update_parent_id(ident, parent_id);
    }

    pub fn children(&self, nid: &str) -> Vec<Node> {
        let mut tmp = Vec::new();
        let op_node = self.nodes.get(nid);
        if op_node.is_none() {
            return vec![];
        };
        let &node = op_node.unwrap();
        for key in node.children_id {
            match self.nodes.get(&key) {
                Some(v) => {
                    tmp.push(v.clone());
                }
                _ => {}
            }
        }
        tmp
    }

    pub fn create_node<T: Into<Option<>>>(
        &mut self,
        identifier: &str,
        parent_id: &str,
        node_antlr: T, // ParserRuleContext,// TODO
        node_state: Config,
    ) {
        let data = Data {
            node_antlr,
            node_state,
        };
        self.node = Node::new(identifier, data);
        self.add_nod(parent_id);
    }

    pub fn all_nodes(&self, calc_nodes_id: bool) -> (Vec<Node>, HashMap<usize, Node>, HashMap<usize, Vec<Node>>) {
        let mut nodes = Vec::new();
        let mut nodes_ids = HashMap::new();
        let mut nodes_id = HashMap::new();
        let mut keys_sort = vec![];
        for (key, _) in self.nodes {
            keys_sort.push(key);
        }
        keys_sort.sort();
        for key in keys_sort {
            let op_node = self.nodes.get(&key);
            if op_node.is_some() {
                let node = op_node.unwrap();
                if calc_nodes_id {
                    let id = self.level(&node.get_id());
                    let mut nod_ids = nodes_ids.get_mut(&id);
                    if nod_ids.is_none() {
                        nodes_ids.insert(id, vec![]);
                        nod_ids = nodes_ids.get_mut(&id);
                    }
                    let node_ids = nod_ids.unwrap();
                    node_ids.push(node.clone());
                    nodes_id.insert(id, node.clone());
                }
                nodes.push(node.clone());
            }
        }
        (nodes, nodes_id, nodes_ids)
    }

    pub fn level(&self, nid: &str) -> usize {
        self.rsearch(nid.to_string()).len()
    }

    pub fn depth(&self) -> usize {
        let mut ret = 0;
        for (_, node) in self.nodes {
            if node.get_children_id().len() > 0 {
                let level = self.level(&node.get_id());
                ret = if level >= ret { level } else { ret }
            }
        }
        ret
    }

    pub fn parent(&self, nid: &str) -> Result<Node, String> {
        if !self.contains(nid) {
            exceptions(format!("{}{}{}", "Node '", nid, "' is not in the tree").as_str());
        }
        let node = self.nodes.get(nid);
        let pid;
        let result = {
            if node.is_none() {
                return Err("empty".to_string());
            }
            pid = node.unwrap().get_parent_id();
            if pid.is_empty() || !self.contains(&pid) {
                Err("empty".to_string())
            } else {
                Ok("".to_string())
            }
        };
        match result {
            Ok(_) => {}
            Err(_r) => {}
        } // Todo
        let node = self.nodes.get(&pid);
        match node {
            Some(&n) => Ok(n),
            _ => Err("empty".to_string()),
        }
    }

    pub fn subtree(&self, nid: &str) -> TreeLib {
        let mut st = TreeLib::new(None, false, None);
        if nid.is_empty() {
            return st;
        }
        if !self.contains(nid) {
            exceptions(&format!(
                "Node '{}' is not in the tree",
                nid
            ));
        }
        st.root = nid.to_string();
        let r = self.expand_tree(nid, 2);
        for id in r {
            let node = self.nodes.get(&id);
            if node.is_some() {
                let nodes = node.unwrap();
                st.nodes.insert(nodes.get_id(), nodes.clone());
            }
        }
        st
    }

    fn rsearch<T: Into<Option<String>>>(&self, mut nid: T) -> Vec<String> {
        let Some(mut nid) = nid.into();
        if nid.is_empty() || !self.contains(&nid) {
            return vec![];
        }
        let mut ret = Vec::new();
        loop {
            ret.push(nid.to_string());
            if self.root != nid {
                let node = self.nodes.get(&nid);
                match node {
                    Some(n) => {
                        nid = n.get_id();
                    }
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        ret
    }

    fn expand_tree<T: Into<Option<i32>> + std::cmp::PartialEq>(
        &self,
        nid: &str,
        mode_option: T,
    ) -> Vec<String> {
        let mut mode;
        if let Some(mode_option) = mode_option.into() {
            mode = mode_option
        } else {
            mode = DEPTH;
        }

        let mut ret = Vec::new();
        let nid = if nid.is_empty() {
            self.root.clone()
        } else {
            nid.to_string()
        };
        if !self.contains(&nid) {
            exceptions(format!("{}{}{}", "Node '", nid, "' is not in the tree").as_str());
        }
        ret.push(nid.clone());
        let mut tmp = Vec::new();
        let op_value = self.nodes.get(&nid);
        if op_value.is_none() {
            return ret;
        }
        let value = op_value.unwrap();
        for v in value.children_id {
            let r = self.nodes.get(&v);
            if r.is_some() {
                tmp.push(r.unwrap());
            }
        }
        let mut queue = tmp;
        if DEPTH == mode || WIDTH == mode {
            while !queue.is_empty() {
                ret.push(queue[0].get_id());
                let mut expansion = Vec::new();
                for v in queue[0].children_id {
                    expansion.push(self.nodes.get(&v).unwrap())
                }
                if mode == DEPTH {
                    let mut tmp_expansion = Vec::new();
                    for node in expansion {
                        tmp_expansion.push(node)
                    }
                    for i in 1..queue.iter().len() {
                        match queue.get(i) {
                            Some(&n) => tmp_expansion.push(n),
                            _ => {}
                        }
                    }
                    queue = tmp_expansion;
                } else if mode == WIDTH {
                    let mut tmp_expansion = Vec::new();
                    for exp in expansion {
                        tmp_expansion.push(exp);
                    }

                    for node in queue {
                        tmp_expansion.push(node);
                    }
                    queue = tmp_expansion;
                }
            }
        } else if mode == ZIGZAG {
            let mut stack_fw = Vec::new();
            // queue = self.reverse(queue);
            let mut stack_bw = queue.clone();
            let mut stack = queue.clone();
            let mut direction = false;
            while stack.len() > 0 {
                let mut expansion = Vec::new();
                for i in stack[0].children_id {
                    expansion.push(self.nodes.get(&i));
                }
                ret.push(stack[0].get_id());
                stack.remove(0);
                if direction {
                    for i in expansion {
                        if i.is_some() {
                            stack_bw.push(i.unwrap());
                        }
                    }
                } else {
                    for i in expansion {
                        if i.is_some() {
                            stack_fw.push(i.unwrap());
                        }
                    }
                }
                if stack.len() == 0 {
                    direction = !direction;
                    stack = if direction {
                        stack_fw.clone()
                    } else {
                        stack_bw.clone()
                    }
                }
            }
        } else {
            exceptions(&format!(
                "{}{}{}",
                "Traversal mode '", mode, "' is not supported"
            ));
        };
        ret.clone()
    }

    fn update_parent_id(&mut self, nid: &str, parent_id: &str) {
        let result = self.nodes.get_mut(nid);
        if result.is_some() {
            let node = result.unwrap();
            node.update_parent_id(parent_id);
        }
    }

    fn update_children_id(&mut self, parent_id: &str, child_id: &str, mode: i32) {
        if parent_id.len() == 0 {
            return;
        } else {
            let result = self.nodes.get_mut(parent_id);
            if result.is_some() {
                let node = result.unwrap();
                node.update_children_id(child_id, mode, "");
            }
        }
    }
}

fn exceptions(str: &str) {
    print!("exp {}", str);
}
