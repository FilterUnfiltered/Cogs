//! The abstract syntax tree for the Cogs templating language.

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
    pub name: String,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    // pub is_async: bool,
    pub content: Vec<CodeElement>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Code(String),
    Text(String),
}

#[derive(Debug, Clone)]
pub enum CodeElement {
    Html(HtmlTag),
    Code(String)
}
