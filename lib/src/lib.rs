use crate::utils::process_function_string;
use js_sandbox::{AnyError, Script};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use thiserror::Error;

pub mod builder;
mod utils;

#[derive(Error, Debug)]
pub enum KnowledgeBaseError {
    #[error("This name already exists")]
    NameAlreadyExists,
    #[error("{0}")]
    BadCode(AnyError),
    #[error("Way not found")]
    SolveError,
}

pub type Result<T> = std::result::Result<T, KnowledgeBaseError>;

pub struct KnowledgeBase {
    base: HashMap<String, KBClass>,
    relations: HashMap<String, Relation>,
}

impl KnowledgeBase {
    pub(crate) fn solve(
        &self,
        p0: &[(&Parameter, i32); 3],
        p1: &[&Parameter; 2],
    ) -> Result<HashMap<String, Value>> {
        todo!()
    }

    pub(crate) fn new_rule(
        &self,
        p0: &str,
        p1: &str,
        p2: &Relation,
        p3: &Vec<&Parameter>,
        p4: &Parameter,
    ) {
        todo!()
    }

    pub fn new() -> KnowledgeBase {
        KnowledgeBase {
            base: Default::default(),
            relations: Default::default(),
        }
    }

    pub fn new_class(&mut self, name: &str, description: &str) -> Result<KBClass> {
        if self.base.contains_key(name) {
            Err(KnowledgeBaseError::NameAlreadyExists)
        } else {
            let class = KBClass::new(name, description);
            self.base.insert(name.to_string(), class.clone());
            Ok(class)
        }
    }

    pub fn new_relation(&mut self, js_function: &str, description: &str) -> Result<Relation> {
        let script =
            Script::from_string(js_function).map_err(|e| KnowledgeBaseError::BadCode(e))?;
        let (name, args) = process_function_string(js_function).ok_or(
            KnowledgeBaseError::BadCode(AnyError::msg("Failed to parse name and args of function")),
        )?;
        if args == 0 {
            return Err(KnowledgeBaseError::BadCode(AnyError::msg(
                "0 arguments function",
            )));
        }
        if self.relations.contains_key(&name) {
            Err(KnowledgeBaseError::NameAlreadyExists)
        } else {
            let relation = Relation::new(&name, args, script, description);
            self.relations.insert(name, relation.clone());
            Ok(relation)
        }
    }
}

#[derive(Default, Clone)]
pub struct KBClass {
    pointer: Rc<RefCell<KBClassInner>>,
}

impl KBClass {
    fn new(name: &str, description: &str) -> KBClass {
        KBClass {
            pointer: Rc::new(RefCell::new(KBClassInner {
                name: name.to_string(),
                description: description.to_string(),
                parametres: Default::default(),
                classes: Default::default(),
            })),
        }
    }

    pub fn new_class(&mut self, name: &str, description: &str) -> Result<KBClass> {
        if self.pointer.borrow().classes.contains_key(name) {
            Err(KnowledgeBaseError::NameAlreadyExists)
        } else {
            let class = KBClass::new(name, description);
            self.pointer
                .borrow_mut()
                .classes
                .insert(name.to_string(), class.clone());
            Ok(class)
        }
    }

    pub fn new_parameter(
        &self,
        name: &str,
        description: &str,
        default: Option<Value>,
    ) -> Result<Parameter> {
        if self.pointer.borrow().parametres.contains_key(name) {
            Err(KnowledgeBaseError::NameAlreadyExists)
        } else {
            let parameter = Parameter::new(name, description, default);
            self.pointer
                .borrow_mut()
                .parametres
                .insert(name.to_string(), parameter.clone());
            Ok(parameter)
        }
    }
}

#[derive(Default)]
struct KBClassInner {
    name: String,
    description: String,
    parametres: HashMap<String, Parameter>,
    classes: HashMap<String, KBClass>,
}

#[derive(Clone)]
pub struct Parameter {
    pointer: Rc<RefCell<ParameterInner>>,
}

impl Parameter {
    fn new(name: &str, description: &str, default: Option<Value>) -> Parameter {
        Parameter {
            pointer: Rc::new(RefCell::new(ParameterInner::new(
                name,
                description,
                default,
            ))),
        }
    }
}

struct ParameterInner {
    name: String,
    description: String,
    default: Option<Value>,
}

impl ParameterInner {
    fn new(name: &str, description: &str, default: Option<Value>) -> ParameterInner {
        ParameterInner {
            name: name.to_string(),
            description: description.to_string(),
            default,
        }
    }
}

#[derive(Clone)]
pub struct Relation {
    pointer: Rc<RefCell<RelationInner>>,
}

impl Relation {
    fn new(name: &str, args_count: usize, script: Script, description: &str) -> Self {
        Relation {
            pointer: Rc::new(RefCell::new(RelationInner::new(
                name,
                args_count,
                script,
                description,
            ))),
        }
    }
}

struct RelationInner {
    name: String,
    args_count: usize,
    script: Script,
    description: String,
}

impl RelationInner {
    fn new(name: &str, args_count: usize, script: Script, description: &str) -> Self {
        Self {
            name: name.to_string(),
            args_count,
            script,
            description: description.to_string(),
        }
    }
}

#[cfg(test)]
mod tests;
