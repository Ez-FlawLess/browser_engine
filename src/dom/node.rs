use super::element::Element;

#[derive(Debug)]
pub enum Node {
    Element(Element),
    Text(String)
}

