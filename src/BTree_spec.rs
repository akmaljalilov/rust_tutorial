use std::collections::BTreeMap;
use std::rc::Rc;

use antlr_rust::parser_rule_context::ParserRuleContext;

#[test]
fn test() {
    let mut b_tree:BTreeMap<&str, u32> = BTreeMap::new();
    b_tree.insert("one", 1);
    let b = *b_tree.get("one").unwrap();
    print!("");
}
