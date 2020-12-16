use crate::structs::{Data, Node};

pub type TNode = Node;

pub const ADD: i32 = 0;
const DELETE: i32 = 1;
const INSERT: i32 = 2;
const REPLACE: i32 = 3;


impl Node {
    pub fn new<T: Into<Option<Data>>>(id: &str, data: T) -> Self {
        Self {
            id: id.to_string(),
            parent_id: "".to_string(),
            children_id: Vec::new(),
            data: data.into(),
        }
    }

    pub fn update_children_id(&mut self, nid: &str, mode: i32, replace: &str) {
        if nid.is_empty() {
            return;
        }
        /* if mode == None {
             mode = ADD;
         }*/
        if mode == ADD || mode == INSERT {
            self.children_id.push(nid.to_string());
        } else if mode == DELETE {
            let mut idx = 0;
            for point in self.children_id.into_iter() {
                if point == nid {
                    self.get_children_id().remove(idx);
                }
                idx += 1;
            }
        } else if mode == REPLACE {
            if replace.len() == 0 {
                // TODO
            }
            let index = self.children_id.iter().position(|key| key == nid);
            self.children_id.insert(index.unwrap(), replace.to_string());
        }
    }

    pub fn update_parent_id(&mut self, parent_id: &str) {
        self.parent_id = parent_id.to_string();
    }

    pub fn get_id(self) -> String {
        self.id
    }

    pub fn get_parent_id(self) -> String {
        self.parent_id
    }

    pub fn get_children_id(self) -> Vec<String> {
        self.children_id
    }
}
