use super::*;

pub struct Generator {
    pub trees: Vec<Tree>,
    pub intern_str: StrInterner,
}

macro_rules! push {
    (@noquote $cx:expr, $fmt:expr, $($arg:expr),+$(,)?) => {{
        $cx.format.push_str($fmt);
        $cx.arguments.extend([$($arg),+]);
    }};
    ($cx:expr, $fmt:expr, $($arg:expr),+$(,)?) => {{
        $cx.format.push_str($fmt);
        $cx.arguments.extend([$(quoted(&$arg)),+]);
    }};
}

struct AppendContext {
    pre: String,
    format: String,
    arguments: Vec<String>,
}

impl AppendContext {
    #[inline]
    fn push(&mut self, format: &str) {
        self.format.push_str(format);
    }

    #[inline]
    fn push_arg(&mut self, value: String) {
        self.push("{}");
        self.arguments.push(value);
    }
}

fn quoted(s: &str) -> String {
    format!("r#\"{}\"#", s)
}

impl Expression {
    fn append(&self, cx: &mut AppendContext) {
        match self {
            Expression::Text(literal) => push!(cx, "\"{}\"", literal),
            Expression::Code(code) => cx.push_arg(code.trim().to_string()),
        }
    }
}

impl HtmlTag {
    fn append(&self, cx: &mut AppendContext) {
        push!(
            cx,
            // add space after, if there are attributes
            if self.attributes.is_empty() {
                "<{}"
            } else {
                "<{} "
            },
            self.tag,
        );

        for attr in &self.attributes {
            push!(
                cx,
                if attr.value.is_some() { "{}=" } else { "{}" },
                attr.name
            );
            if let Some(value) = &attr.value {
                value.append(cx);
            }
        }
        if self.content.is_empty() {
            cx.push("/>");
            return;
        }

        cx.push(">");

        for tree in &self.content {
            tree.append(cx);
        }

        push!(cx, "</{}>", self.tag);
    }
}

impl CodeTree {
    fn append(&self, cx: &mut AppendContext) {
        match self {
            CodeTree::Code(code) => {
                let trimmed = code.trim();
                if trimmed.is_empty() {
                    return;
                }
                cx.pre.push_str(code.trim());
            }
            CodeTree::HtmlTag(html_tag) => html_tag.append(cx),
        }
    }
}

impl CodeBlock {
    fn append(&self, cx: &mut AppendContext) {
        let mut my_cx = AppendContext {
            pre: String::new(),
            format: String::new(),
            arguments: Vec::new(),
        };
        for code in self.content.iter() {
            code.append(&mut my_cx);
        }

        let fmt = if my_cx.format.is_empty() {
            my_cx.pre
        } else {
            format!(
                r##"{{{pre}
format!(r#"{format}"#, {arguments})
}}"##,
                pre = my_cx.pre,
                format = my_cx.format,
                arguments = my_cx.arguments.join(","),
            )
        };

        push!(@noquote cx, "{}", fmt);
    }
}

impl Tree {
    fn append(&self, cx: &mut AppendContext) {
        match self {
            Tree::HtmlText(text) => {
                // this adds {} to the format string and adds "{text}" to the args
                cx.push_arg(quoted(text.trim()));
            }
            Tree::HtmlTag(html_tag) => html_tag.append(cx),
            Tree::CodeBlock(code_block) => code_block.append(cx),
        }
    }
}

impl Generator {
    pub fn to_format(&self) -> String {
        let mut cx = AppendContext {
            pre: String::new(),
            format: String::new(),
            arguments: Vec::new(),
        };
        for tree in self.trees.iter() {
            if let Tree::CodeBlock(code_block) = tree {
                if !code_block.has_html {
                    for code in code_block.content.iter() {
                        let CodeTree::Code(code) = code else {
                            panic!("has_html = false, but got CodeTree::HtmlTag")
                        };
                        let trimmed = code.trim();
                        if trimmed.is_empty() {
                            continue;
                        }
                        cx.pre.push_str(code.trim());
                    }
                }
            } else {
                tree.append(&mut cx);
            }
        }
        format!(
            "{pre}let __rendered = format!(r#\"{format}\"#, {arguments});",
            pre = cx.pre,
            format = cx.format,
            arguments = cx.arguments.join(","),
        )
    }
}
