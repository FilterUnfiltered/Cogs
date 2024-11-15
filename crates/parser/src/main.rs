use cogs_parser::parse_cog;

fn main() {
    println!("{:#?}", parse_cog("
<body>
    <h1>Yo.</h1>
    <a src=\"https://www.youtube.com/watch?v=dQw4w9WgXcQ\">Click this</a>
</body>"));
}