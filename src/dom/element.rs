use std::{collections::HashMap, sync::{Arc, Mutex}, fmt::Display};

use super::node::Node;

#[derive(Debug)]
pub struct Element {
    tag_name: String,
    attributes: HashMap<String, String>,
    children: Vec<Arc<Mutex<Node>>>,
}

impl Element {
    pub fn new(tag_name: String) -> Self {
        Self { tag_name, attributes: HashMap::new(), children: Vec::new() }
    }

    pub fn get_attribute(&self, attribute_name: &str) -> Option<&String> {
        self.attributes.get(attribute_name)
    }

    pub fn add_to_children(&mut self, child: Arc<Mutex<Node>>) {
        self.children.push(child);
    }

    pub fn add_attribute(&mut self, name: String, value: String) {
        self.attributes.insert(name, value);
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ", self.tag_name).unwrap();
        for (k, v) in &self.attributes {
            write!(f, "{}=\"{}\" ", k, v).unwrap();
        }
        write!(f, ">\n").unwrap();
        for child in &self.children {
            let guard = child.lock().unwrap();
            match *guard {
                Node::Element(ref e) => {
                    write!(f, "{}", e).unwrap();
                },
                _ => (),
            }
        }
        writeln!(f, "<{} />", self.tag_name)
    }
}