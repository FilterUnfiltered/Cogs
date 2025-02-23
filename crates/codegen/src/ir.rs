use super::*;

pub enum Expression {
    Text(InternedStr),
    Code(InternedStr),
}

pub struct HtmlAttribute {
    pub name: InternedStr,
    pub value: Option<Expression>,
}

pub struct HtmlTag {
    pub tag: InternedStr,
    pub attributes: Vec<HtmlAttribute>,
    pub content: Vec<Tree>,
}

pub enum CodeTree {
    HtmlTag(HtmlTag),
    Code(InternedStr),
}

pub struct CodeBlock {
    pub has_html: bool,
    pub content: Vec<CodeTree>,
}

pub enum Tree {
    HtmlText(InternedStr),
    HtmlTag(HtmlTag),
    CodeBlock(CodeBlock),
}

impl Tree {
    pub fn from_ast(value: &ast::Element, intern: &StrInterner) -> Self {
        match value {
            ast::Element::Text(text) => Tree::HtmlText(intern.intern_ref(text)),
            ast::Element::Html(html) => Tree::HtmlTag(HtmlTag::from_ast(html, intern)),
            ast::Element::Block(block) => Tree::CodeBlock(CodeBlock::from_ast(block, intern)),
        }
    }
}

impl HtmlTag {
    pub fn from_ast(value: &ast::HtmlTag, intern: &StrInterner) -> Self {
        let tag = intern.intern_ref(&value.tag);
        let attributes = value
            .attributes
            .iter()
            .map(|attr| HtmlAttribute::from_ast(attr, intern))
            .collect();
        let content = value
            .content
            .iter()
            .map(|elem| Tree::from_ast(elem, intern))
            .collect();
        HtmlTag {
            tag,
            attributes,
            content,
        }
    }
}

impl HtmlAttribute {
    pub fn from_ast(value: &ast::Attribute, intern: &StrInterner) -> Self {
        let name = intern.intern_ref(&value.name);
        let value = value
            .value
            .as_ref()
            .map(|value| Expression::from_ast(value, intern));
        HtmlAttribute { name, value }
    }
}

impl CodeBlock {
    pub fn from_ast(value: &ast::CodeBlock, intern: &StrInterner) -> Self {
        let mut has_html = false;
        let content = value
            .content
            .iter()
            .map(|elem| {
                if matches!(elem, ast::CodeElement::Html(_)) {
                    has_html = true
                };
                CodeTree::from_ast(elem, intern)
            })
            .collect();
        CodeBlock { content, has_html }
    }
}

impl Expression {
    pub fn from_ast(value: &ast::Expression, intern: &StrInterner) -> Self {
        match value {
            ast::Expression::Code(code) => Expression::Code(intern.intern_ref(code)),
            ast::Expression::Text(text) => Expression::Text(intern.intern_ref(text)),
        }
    }
}

impl CodeTree {
    pub fn from_ast(value: &ast::CodeElement, intern: &StrInterner) -> Self {
        match value {
            ast::CodeElement::Html(html) => CodeTree::HtmlTag(HtmlTag::from_ast(html, intern)),
            ast::CodeElement::Code(code) => CodeTree::Code(intern.intern_ref(code)),
        }
    }
}
