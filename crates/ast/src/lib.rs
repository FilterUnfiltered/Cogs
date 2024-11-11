pub struct Component {
    pub elements: Vec<Element>,
}

pub enum Element {
    Html(HtmlTag),
    Block(CodeBlock),
}

pub struct HtmlTag {
    pub tag: String,
    pub attributes: Vec<Attribute>,
    pub content: Vec<Element>,
}

pub struct Attribute {
    pub name: String,
    pub value: String,
}

pub struct CodeBlock {
    pub is_async: bool,
    pub content: String,
}
