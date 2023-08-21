use crate::graph_solver::Graph;
use crate::{builder::KnowledgeBaseBuilder, KnowledgeBaseError, Result};
use js_sandbox::{AnyError, Script};
use serde_json::Value;
use std::collections::hash_map::Entry::Vacant;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default, Debug)]
pub struct KnowledgeBase {
    pub(crate) base: HashMap<String, KBClass>,
    relations: HashMap<String, Relation>,
    pub(crate) rules: Vec<Rule>,
}

impl KnowledgeBase {
    pub fn solve(
        &self,
        known_values: &[(Parameter, Value)],
        values_to_find: &[Parameter],
    ) -> HashMap<String, Value> {
        Graph::new(self, known_values, values_to_find).go()
    }

    pub fn new_rule(
        &mut self,
        name: &str,
        description: &str,
        relation: Relation,
        args: &[Parameter],
        out: Parameter,
    ) -> Result<()> {
        if relation.pointer.borrow().args_count != args.len() {
            return Err(KnowledgeBaseError::ArgCountError);
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
            let class = KBClass::new(name, description, None);
            self.base.insert(name.to_string(), class.clone());
            Ok(class)
        }
    }

    pub fn new_relation(&mut self, js_function: &str, description: &str) -> Result<Relation> {
        let _ = Script::from_string(js_function).map_err(KnowledgeBaseError::BadCode)?;
        let (name, args_count) = process_function_string(js_function).ok_or(
            KnowledgeBaseError::BadCode(AnyError::msg("Failed to parse name and args of function")),
        )?;
        if args_count == 0 {
            return Err(KnowledgeBaseError::BadCode(AnyError::msg(
                "0 arguments function",
            )));
        }
        if let Vacant(e) = self.relations.entry(name) {
            let relation = Relation::new(e.key(), args_count, js_function, description);
            e.insert(relation.clone());
            Ok(relation)
        } else {
            Err(KnowledgeBaseError::NameAlreadyExists)
        }
    }

    pub fn builder() -> KnowledgeBaseBuilder {
        KnowledgeBaseBuilder::new()
    }
}

pub(crate) fn process_function_string(input: &str) -> Option<(String, usize)> {
    // Поиск индекса начала имени функции
    let start_idx = input.find("function")?;

    // Ищем индекс открытой скобки после имени функции
    let open_bracket_idx = input[start_idx..].find('(').unwrap_or(0) + start_idx;

    // Ищем индекс закрывающей скобки после списка аргументов
    let close_bracket_idx = input[open_bracket_idx..].find(')').unwrap_or(0) + open_bracket_idx;

    // Извлекаем имя функции и аргументы
    let function_name = input[start_idx + "function".len()..open_bracket_idx]
        .trim()
        .to_string();
    let arguments = input[open_bracket_idx + 1..close_bracket_idx]
        .split(',')
        .count();

    Some((function_name, arguments))
}

#[derive(Clone, Debug)]
pub struct KBClass {
    pointer: Rc<RefCell<KBClassInner>>,
    master: Option<Box<KBClass>>,
}

impl KBClass {
    pub(crate) fn get_full_name(&self) -> String {
        if let Some(master) = &self.master {
            master.get_full_name() + &self.pointer.borrow().name
        } else {
            self.pointer.borrow().name.clone()
        }
    }

    fn new(name: &str, description: &str, master: Option<Box<KBClass>>) -> Self {
        KBClass {
            pointer: Rc::new(RefCell::new(KBClassInner::new(name, description))),
            master,
        }
    }

    pub fn new_class(&mut self, name: &str, description: &str) -> Result<KBClass> {
        if self.pointer.borrow().classes.contains_key(name) {
            Err(KnowledgeBaseError::NameAlreadyExists)
        } else {
            let class = KBClass::new(name, description, Some(Box::new(self.clone())));
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
            let parameter = Parameter::new(name, description, default, Box::new(self.clone()));
            self.pointer
                .borrow_mut()
                .parametres
                .insert(name.to_string(), parameter.clone());
            Ok(parameter)
        }
    }
}

#[derive(Debug)]
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

#[derive(Clone, Debug)]
pub struct Parameter {
    pub(crate) pointer: Rc<RefCell<ParameterInner>>,
    pub(crate) master: Box<KBClass>,
}

impl Parameter {
    pub(crate) fn get_full_name(&self) -> String {
        self.master.as_ref().get_full_name() + &self.pointer.borrow().name
    }

    fn new(name: &str, description: &str, default: Option<Value>, master: Box<KBClass>) -> Self {
        Parameter {
            pointer: Rc::new(RefCell::new(ParameterInner::new(
                name,
                description,
                default,
            ))),
            master,
        }
    }
}

#[derive(Debug)]
pub(crate) struct ParameterInner {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) default: Option<Value>,
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

#[derive(Clone, Debug)]
pub struct Relation {
    pub(crate) pointer: Rc<RefCell<RelationInner>>,
}

impl Relation {
    fn new(name: &str, args_count: usize, js_code: &str, description: &str) -> Self {
        Relation {
            pointer: Rc::new(RefCell::new(RelationInner::new(
                name,
                args_count,
                js_code,
                description,
            ))),
        }
    }
}

#[derive(Debug)]
pub(crate) struct RelationInner {
    pub(crate) name: String,
    pub(crate) args_count: usize,
    pub(crate) js_code: String,
    pub(crate) description: String,
}

impl RelationInner {
    fn new(name: &str, args_count: usize, js_code: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            args_count,
            js_code: js_code.to_string(),
            description: description.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) target: Parameter,
    pub(crate) args: Vec<Parameter>,
    pub(crate) relation: Relation,
}

impl Rule {
    pub fn new(
        name: &str,
        description: &str,
        target: Parameter,
        args: &[Parameter],
        relation: Relation,
    ) -> Rule {
        Rule {
            name: name.to_string(),
            description: description.to_string(),
            target,
            args: Vec::from(args),
            relation,
        }
    }
}
