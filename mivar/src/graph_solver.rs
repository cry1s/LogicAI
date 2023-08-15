use crate::KnowledgeBase;
use js_sandbox::Script;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

pub(crate) struct Graph {
    all_nodes: HashMap<String, Rc<RefCell<Node>>>,
}

#[derive(Debug)]
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
    args: Vec<Rc<RefCell<Node>>>,
}

impl Debug for WayToGet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WayToGet")
            .field("args", &self.args)
            .finish()
    }
}

impl Graph {
    pub(crate) fn new(kb: &KnowledgeBase) -> Self {
        let mut all_nodes = HashMap::new();

        for rule in kb.rules.iter() {
            let name = rule.target.get_full_name();
            let node = all_nodes
                .entry(name.clone())
                .or_insert(Rc::new(RefCell::new(Node::new(
                    name,
                    rule.target.pointer.borrow().default.clone(), // TODO might be very expensive
                ))));
            let node = Rc::clone(node);
            node.borrow_mut().wtg.push(WayToGet {
                script: Script::from_string(&rule.relation.pointer.borrow().js_code)
                    .expect("code already checked"), // TODO move to single script
                args: {
                    rule.args
                        .iter()
                        .map(|arg| {
                            let name = arg.get_full_name();
                            let node =
                                all_nodes
                                    .entry(name.clone())
                                    .or_insert(Rc::new(RefCell::new(Node::new(
                                        name,
                                        arg.pointer.borrow().default.clone(), // TODO might be very expensive
                                    ))));
                            Rc::clone(node)
                        })
                        .collect()
                },
            });
        }
        dbg!(&all_nodes);
        Graph { all_nodes }
    }
}
