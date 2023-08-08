use serde_json::Value;

pub struct Node {
    parameter: Value
}

pub struct Edge {
    rule: String,
    start_nodes: Vec<Node>,
    end_nodes: Vec<Node>
}

#[cfg(test)]
mod tests;
