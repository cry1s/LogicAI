use crate::{builder::KnowledgeBaseBuilder, utils::process_function_string, KnowledgeBaseError, Result};
use js_sandbox::{AnyError, Script};
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct KnowledgeBase {
    base: HashMap<String, KBClass>,
    relations: HashMap<String, Relation>,
    rules: Vec<Rule>,
}

impl KnowledgeBase {
    pub fn solve(
        &self,
        known_values: &Vec<(&Parameter, Value)>,
        values_to_find: &Vec<&Parameter>,
    ) -> Result<HashMap<String, Value>> {
        todo!()
    }

    pub(crate) fn new_rule(
        &self,
        name: &str,
        description: &str,
        relation: &Relation,
        args: &Vec<&Parameter>,
        out: &Parameter,
    ) -> Result<Rule> {
        todo!()
    }

    pub fn new() -> KnowledgeBase {
        KnowledgeBase {
            base: Default::default(),
            relations: Default::default(),
            rules: vec![],
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
        let (name, args_count) = process_function_string(js_function).ok_or(
            KnowledgeBaseError::BadCode(AnyError::msg("Failed to parse name and args of function")),
        )?;
        if args_count == 0 {
            return Err(KnowledgeBaseError::BadCode(AnyError::msg(
                "0 arguments function",
            )));
        }
        if self.relations.contains_key(&name) {
            Err(KnowledgeBaseError::NameAlreadyExists)
        } else {
            let relation = Relation::new(&name, args_count, script, description);
            self.relations.insert(name, relation.clone());
            Ok(relation)
        }
    }

    pub(crate) fn builder() -> KnowledgeBaseBuilder {
        KnowledgeBaseBuilder::new()
    }
}

#[derive(Clone)]
pub struct KBClass {
    pointer: Rc<RefCell<KBClassInner>>,
}

impl KBClass {
    fn new(name: &str, description: &str) -> Self {
        KBClass {
            pointer: Rc::new(RefCell::new(KBClassInner::new(name, description))),
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

struct KBClassInner {
    name: String,
    description: String,
    parametres: HashMap<String, Parameter>,
    classes: HashMap<String, KBClass>,
}

impl KBClassInner {
    fn new(name: &str, description: &str) -> Self {
        KBClassInner {
            name: name.to_string(),
            description: description.to_string(),
            parametres: Default::default(),
            classes: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Parameter {
    pointer: Rc<RefCell<ParameterInner>>,
}

impl Parameter {
    fn new(name: &str, description: &str, default: Option<Value>) -> Self {
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
    fn new(name: &str, description: &str, default: Option<Value>) -> Self {
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

pub struct Rule {
    target: Parameter,
    args: Vec<Parameter>,
    relation: Relation,
}
