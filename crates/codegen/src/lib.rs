extern crate cogs_ast as ast;

mod generate;
use generate::*;
mod ir;
use ir::*;

// type aliases for:
// 1. convenience
// 2. easy swapping to InternedOrd if i ever find it better
type InternedStr = intern_arc::InternedHash<str>;
type StrInterner = intern_arc::HashInterner<str>;

pub fn generate(ast: &ast::Component) -> eyre::Result<String> {
    let mut generator = Generator {
        trees: Vec::new(),
        intern_str: StrInterner::new(),
    };
    for element in ast.elements.iter() {
        generator
            .trees
            .push(Tree::from_ast(element, &generator.intern_str));
    }

    let render = generator.to_format();
    Ok(format!(
        r#"
#[derive(Default)]
pub struct Cog;

impl cogs_runtime::Component for Cog {{
    type Props = ();
    type Error = core::convert::Infallible;
    fn render(&self, props: Self::Props) -> impl core::future::Future<Output = Result<String, Self::Error>> + core::marker::Send + '_ {{
        async move {{
            {render}
            Ok(__rendered)
        }}
    }}
}}
"#
    ))
}
