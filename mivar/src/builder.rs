use super::Result;
use serde_json::Value;
use crate::KnowledgeBase;

pub struct KnowledgeBaseBuilder {}

impl KnowledgeBaseBuilder {
    pub(crate) fn build(&self) -> Result<KnowledgeBase> {
        todo!()
    }

    pub(crate) fn new_rule(
        &self,
        p0: &str,
        p1: &Vec<&str>,
        p2: &Vec<&str>,
        p3: &str,
        p4: &str,
    ) -> KnowledgeBaseBuilder {
        todo!()
    }

    pub(crate) fn new_relation(&self, p0: &str) -> KnowledgeBaseBuilder {
        todo!()
    }

    pub(crate) fn go_base(&self) -> KnowledgeBaseBuilder {
        todo!()
    }

    pub(crate) fn leave_class(&self) -> KnowledgeBaseBuilder {
        todo!()
    }

    pub(crate) fn add_parameter(
        &self,
        p0: &str,
        p1: &str,
        p2: Option<Value>,
    ) -> KnowledgeBaseBuilder {
        todo!()
    }

    pub(crate) fn new_class(&self, p0: &str, p1: &str) -> KnowledgeBaseBuilder {
        todo!()
    }

    pub(crate) fn new() -> Self {
        todo!()
    }
}
