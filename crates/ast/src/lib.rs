#[derive(Debug)]
pub struct Component {
    pub elements: Vec<Element>,
}

#[derive(Debug, Clone)]
pub enum Element {
    Html(HtmlTag),
    Block(CodeBlock),
}

#[derive(Debug, Clone)]
pub enum HtmlContent {
    Element(Element),
    Text(String)
}

#[derive(Debug, Clone)]
pub struct HtmlTag {
    pub tag: String,
    pub attributes: Vec<Attribute>,
    pub content: Vec<HtmlContent>,
}

#[derive(Debug, Clone)]
pub enum TextOrCode {
    Text(String),
    Code(CodeBlock),
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: TextOrCode,
    pub value: Option<TextOrCode>,
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    // pub is_async: bool,
    pub content: Vec<Element>,
}
