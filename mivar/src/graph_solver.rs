use crate::KnowledgeBase;
use js_sandbox::Script;
use serde_json::Value;
use std::collections::HashMap;
use std::rc::Rc;

struct Graph {
    all_nodes: HashMap<String, Node>,
}

struct Node {
    mark: bool,
    wtg: Vec<WayToGet>,
    value: Option<Value>,
    default: Option<Value>,
    name: String,
    possible_to_calc: bool,
}

impl Node {
    pub(crate) fn new(name: String, default: Option<Value>) -> Node {
        Node {
            mark: false,
            wtg: vec![],
            value: None,
            default,
            name,
            possible_to_calc: false,
        }
    }
}

struct WayToGet {
    script: Script,
    args: Vec<Rc<Node>>,
}

impl Graph {
    pub(crate) fn new(kb: KnowledgeBase) -> Graph {
        let mut all_nodes = HashMap::new();

        for rule in kb.rules.iter() {
            let name = rule.target.get_full_name();
            let node = all_nodes.entry(name.clone()).or_insert(Node::new(
                name,
                rule.target.pointer.borrow().default.clone(), // TODO might be very expensive
            ));

        }
        Graph { all_nodes }
    }
}
