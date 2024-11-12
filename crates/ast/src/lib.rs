#[derive(Debug)]
pub struct Component {
    pub elements: Vec<Element>,
}

#[derive(Debug)]
pub enum Element {
    Html(HtmlTag),
    Block(CodeBlock),
}

#[derive(Debug)]
pub struct HtmlTag {
    pub tag: String,
    pub attributes: Vec<Attribute>,
    pub content: Vec<Element>,
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct CodeBlock {
    pub is_async: bool,
    pub content: String,
}
