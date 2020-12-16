use std::collections::HashMap;
use antlr_rust::parser_rule_context::ParserRuleContext;

#[derive(Clone)]
pub struct Node {
    pub id: String,
    pub parent_id: String,
    pub children_id: Vec<String>,
    pub data: Option<Data>,
}

#[derive(Clone, Debug)]
pub struct Config {

}

#[derive(Clone, Debug)]
pub struct Data {
    pub node_antlr: ParserRuleContext,// TODO
    pub node_state: Config,
}


#[derive(Clone)]
pub struct TreeLib {
    pub node: Node,
    pub nodes: HashMap<String, Node>,
    pub root: String,
    pub identifier: String,
}
