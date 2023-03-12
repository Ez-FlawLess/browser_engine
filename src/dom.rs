use std::{sync::{Arc, Mutex}, fmt::Display};

use self::node::Node;
use crate::{utils::GetWhile, dom::element::Element};

pub mod node;
pub mod element;

pub struct Dom {
    dom_tree: Vec<Arc<Mutex<Node>>>,
    node_stack: Vec<Arc<Mutex<Node>>>,
}

impl Dom {
    pub fn new() -> Self {
        Self { 
            dom_tree: Vec::new(),
            node_stack: Vec::new(),
        }
    }

    pub fn parse_html(&mut self, html: String) {

        let mut html_iter = html.chars().peekable();
        'main_loop: while let Some(char) = html_iter.next() {
            match char {
                '<' => {
                    // element definition
                    match html_iter.next() {
                        Some('/') => {
                            // it's closing tag
                            // it should get removed from the node_stack
                            html_iter.get_while(|ch| ch != '>' && !ch.is_whitespace());
                            self.remove_from_tree();
                        },
                        Some(char) => {
                            // it's opening tag
                            // should get added to node_stack and to the child of it's last node
                            let tag_name = {
                                let mut result = char.to_string();
        
                                while let Some(char) = html_iter.next() {
                                    if char.is_whitespace() {
                                        break;
                                    } else if char == '>' {
                                        // end of tag
                                        // has no attribute
                                        self.add_to_tree(Arc::new(Mutex::new(Node::Element(Element::new(result)))));
                                        continue 'main_loop;
                                    } else {
                                        result.push(char);
                                    }
                                }
        
                                result
                            };

                            let mut element = Element::new(tag_name);

                            while let Some(char) = html_iter.next() {
                                if char == '>' {
                                    // end of tag
                                    self.add_to_tree(Arc::new(Mutex::new(Node::Element(element))));
                                    continue 'main_loop;
                                } else if char == ' ' {
                                    continue;
                                } else {
                                    // add attribute to element
                                    let name = char.to_string() + &html_iter.get_while(|ch| ch != '=' && !ch.is_whitespace());
                                    html_iter.get_while(|ch| ch != '"' && !ch.is_whitespace());
                                    let value = html_iter.get_while(|ch| ch != '"');
                                    element.add_attribute(name, value);
                                }
                            }
                        },
                        None => (),
                    }
                },
                _ => {
                    if char.is_whitespace() {
                        continue;
                    }
                    // child is text
                    let mut text = char.to_string();
                    let mut holder = String::new();

                    while html_iter.peek().map_or(false, |&ch| ch != '<' ) {
                        let char = html_iter.next().unwrap();
                        if char.is_whitespace() {
                            holder.push(char);
                        } else {
                            text.push_str(&holder);
                            holder.clear();
                            text.push(char);
                        }
                        
                    }

                    self.add_to_tree(Arc::new(Mutex::new(Node::Text(text))));
                },
            }
        }
    }
    
    fn add_to_tree(&mut self, node: Arc<Mutex<Node>>) {
        if self.node_stack.len() == 0 {
            self.node_stack.push(Arc::clone(&node));
            self.dom_tree.push(node);
        } else {
            {
                let last_node_in_stack = self.node_stack.last_mut().unwrap();
                let mut guard = last_node_in_stack.lock().unwrap();
                match *guard {
                    Node::Element(ref mut e) => {
                        e.add_to_children(Arc::clone(&node));
                    },
                    _ => (),
                }
            }
            let guard = node.lock().unwrap();
            if let Node::Element(_) = *guard {
                self.node_stack.push(Arc::clone(&node));
            }
        }
    }

    fn remove_from_tree(&mut self) {
        self.node_stack.pop();
    }

}

impl Display for Dom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        for node in &self.dom_tree {
            let guard = node.lock().unwrap();
            match *guard {
                Node::Element(ref e) => {
                    write!(f, "{}", e).unwrap();
                },
                _ => (),
            }
        }
        
        Ok(())
    }
}