#[derive(Debug)]
pub struct Component {
    pub elements: Vec<Element>,
}

#[derive(Debug, Clone)]
pub enum Element {
    Html(HtmlTag),
    Block(CodeBlock),
    Text(String)
}

#[derive(Debug, Clone)]
pub struct HtmlTag {
    pub tag: String,
    pub attributes: Vec<Attribute>,
    pub content: Vec<Element>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: Element,
    pub value: Option<Element>,
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    // pub is_async: bool,
    pub content: Vec<Element>,
}
