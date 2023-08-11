use serde_json::Value;
use super::Result;

pub struct KnowledgeBaseBuilder {

}

impl KnowledgeBaseBuilder {
    pub(crate) fn build(&self) {
        todo!()
    }

    pub(crate) fn new_rule(&self, p0: &str, p1: &Vec<&str>, p2: &Vec<&str>, p3: &str, p4: &str) -> Result<KnowledgeBaseBuilder> {
        todo!()
    }

    pub(crate) fn new_relation(&self, p0: &str) -> Result<KnowledgeBaseBuilder> {
        todo!()
    }

    pub(crate) fn go_base(&self) -> KnowledgeBaseBuilder {
        todo!()
    }

    pub(crate) fn leave_class(&self) -> KnowledgeBaseBuilder {
        todo!()
    }

    pub(crate) fn add_parameter(&self, p0: &str, p1: &str, p2: Option<Value>) -> Result<KnowledgeBaseBuilder> {
        todo!()
    }

    pub(crate) fn new_class(&self, p0: &str, p1: &str) -> Result<KnowledgeBaseBuilder> {
        todo!()
    }

    pub(crate) fn new() -> Self {
        todo!()
    }
}