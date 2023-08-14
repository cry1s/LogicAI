use crate::{
    builder::KnowledgeBaseBuilder, utils::process_function_string, KnowledgeBaseError, Result,
};
use js_sandbox::{AnyError, Script};
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use std::collections::hash_map::Entry::Vacant;
use KnowledgeBaseError::ArgCountError;

#[derive(Default)]
pub struct KnowledgeBase {
    base: HashMap<String, KBClass>,
    relations: HashMap<String, Relation>,
    rules: Vec<Rule>,
}

impl KnowledgeBase {
    pub fn solve(
        &self,
        known_values: &[(Parameter, Value)],
        values_to_find: &[Parameter],
    ) -> Result<HashMap<String, Value>> {
        todo!()
    }

    pub(crate) fn new_rule(
        &mut self,
        name: &str,
        description: &str,
        relation: Relation,
        args: &[Parameter],
        out: Parameter,
    ) -> Result<()> {
        if relation.pointer.borrow().args_count != args.len() {
            return Err(ArgCountError)
        }
        let rule = Rule::new(name, description, out, args, relation);
        self.rules.push(rule);
        Ok(())
    }

    pub fn new() -> KnowledgeBase {
        KnowledgeBase::default()
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
            Script::from_string(js_function).map_err(KnowledgeBaseError::BadCode)?;
        let (name, args_count) = process_function_string(js_function).ok_or(
            KnowledgeBaseError::BadCode(AnyError::msg("Failed to parse name and args of function")),
        )?;
        if args_count == 0 {
            return Err(KnowledgeBaseError::BadCode(AnyError::msg(
                "0 arguments function",
            )));
        }
        if let Vacant(e) = self.relations.entry(name) {
            let relation = Relation::new(e.key(), args_count, script, description);
            e.insert(relation.clone());
            Ok(relation)
        } else {
            Err(KnowledgeBaseError::NameAlreadyExists)
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
    name: String,
    description: String,
    target: Parameter,
    args: Vec<Parameter>,
    relation: Relation,
}

impl Rule {
    pub fn new(name: &str, description: &str, target: Parameter, args: &[Parameter], relation: Relation) -> Rule {
        Rule {
            name: name.to_string(),
            description: description.to_string(),
            target,
            args: Vec::from(args),
            relation,
        }
    }
}
