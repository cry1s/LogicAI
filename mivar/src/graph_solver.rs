use super::Result;
use crate::model::Parameter;
use crate::{KnowledgeBase, KnowledgeBaseError};
use js_sandbox::Script;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::DerefMut;
use std::rc::Rc;

pub(crate) struct Graph<'a> {
    all_nodes: HashMap<String, Rc<RefCell<Node<'a>>>>,
    find_nodes: HashMap<String, Rc<RefCell<Node<'a>>>>,
    script: Script,
}

#[derive(Debug)]
struct Node<'a> {
    mark: bool,
    wtg: Vec<WayToGet<'a>>,
    parameter: Parameter,
    value: Option<Value>,
    possible_to_calc: bool,
}

impl<'a> Node<'a> {
    pub(crate) fn new(parameter: Parameter) -> Self {
        Node {
            mark: false,
            wtg: vec![],
            parameter,
            value: None,
            possible_to_calc: false,
        }
    }

    fn calc_node(&mut self, script: &mut Script) -> Result<Value> {
        if self.value.is_some() {
            return Ok(self.value.clone().unwrap());
        }
        if self.wtg.is_empty() {
            if self.parameter.pointer.borrow().default.is_some() {
                self.value = self.parameter.pointer.borrow().default.clone();
            }
            return Err(KnowledgeBaseError::SolveError);
        }
        println!("Calculating {}", self.parameter.pointer.borrow().name);
        self.wtg
            .iter()
            .filter_map(|w| {
                let args = w
                    .args
                    .iter()
                    .map(|arg| {
                        arg.borrow_mut().calc_node(script).ok() // cause panic in cycle graphs
                    })
                    .fuse()
                    .flatten()
                    .collect::<Vec<Value>>();
                (if args.len() != w.args.len() {
                    Err(KnowledgeBaseError::SolveError)
                } else {
                    let v = dbg!(script.call("multiple_args", &(&w.fun_name, dbg!(&args)))).ok()?;
                    self.value = Some(v);
                    Ok(self.value.clone())
                })
                .ok()?
            })
            .next()
            .ok_or(KnowledgeBaseError::SolveError)
    }
}

struct WayToGet<'a> {
    fun_name: String,
    args: Vec<Rc<RefCell<Node<'a>>>>,
}

impl<'a> Debug for WayToGet<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WayToGet")
            .field("args", &self.args)
            .finish()
    }
}

impl<'a> Graph<'a> {
    pub(crate) fn new(
        kb: &'a KnowledgeBase,
        known_values: &[(Parameter, Value)],
        values_to_find: &[Parameter],
    ) -> Self {
        let mut all_nodes = HashMap::new();

        known_values.iter().for_each(|(par, val)| {
            let name = par.get_full_name();
            let node = all_nodes
                .entry(name)
                .or_insert(Rc::new(RefCell::new(Node::new(par.clone()))));
            node.borrow_mut().deref_mut().value = Some(val.clone())
        });

        let mut find_nodes = HashMap::new();
        values_to_find.iter().for_each(|par| {
            let name = par.get_full_name();

            let node = find_nodes
                .entry(name.clone())
                .or_insert(Rc::new(RefCell::new(Node::new(par.clone()))));
            all_nodes.insert(name, node.clone());
        });

        let mut used_relations = HashSet::new();

        kb.rules.iter().for_each(|rule| {
            let name = rule.target.get_full_name();
            let node = all_nodes
                .entry(name)
                .or_insert(Rc::new(RefCell::new(Node::new(rule.target.clone()))));
            let relation = rule.relation.pointer.borrow();
            used_relations.insert(format!("'{}': {},", relation.name, relation.js_code));
            let node = Rc::clone(node);
            node.borrow_mut().wtg.push(WayToGet {
                fun_name: rule.relation.pointer.borrow().name.to_string(),
                args: {
                    rule.args
                        .iter()
                        .map(|arg| {
                            let name = arg.get_full_name();
                            let node = all_nodes
                                .entry(name)
                                .or_insert(Rc::new(RefCell::new(Node::new(arg.clone()))));
                            Rc::clone(node)
                        })
                        .collect()
                },
            });
        });
        let all_functions = used_relations
            .into_iter()
            .fold("const functions = {".to_string(), |acc, x| acc + &x)
            + "};"
            + "function multiple_args(a) {
    const fn_name = a[0];
    const args = a[1];
    return functions[fn_name](...args)
}";
        dbg!(&all_functions);
        let script =
            Script::from_string(all_functions.as_str()).expect("functions should be checked");

        Graph {
            all_nodes,
            find_nodes,
            script,
        }
    }

    pub(crate) fn go(mut self) -> HashMap<String, Value> {
        self.find_nodes
            .into_iter()
            .filter_map(|(key, node)| {
                let v = node.borrow_mut().calc_node(&mut self.script);
                if let Ok(value) = v {
                    Some((key, value))
                } else {
                    None
                }
            })
            .collect()
    }
}
