# Cogs - Rust in HTML

Cogs is a fullstack framework for building HATEOAS-based web applications using a custom templating language that compiles directly to highly-optimized Rust code.

[HTMX](https://htmx.org/) is recommended alongside Cogs as the frontend piece of the HATEOAS stack.

## Examples
```html
{ let random: u32 = rand::random(); }
<html>
  <head>
    <title>cogs - small_to_console</title>
  </head>
  <body>
    <h1>Hello World!</h1>
    <p>The random number is: <h1>{random}</h1></p>
    <p>Want more?</p>
    <ul>
      {
        for _ in 0..5 {
          let num: u32 = rand::random();
          <li>{num}</li>
        }
      }
    </ul>
  </body>
</html>
```

## Roadmap

- [x] Basic syntax and parsing
- [x] Code generation
- [x] Basic axum example
- [ ] Full axum integration
- [ ] (?) Custom codegen based HTMX alternative
- [ ] (?) WASM-based "Client Components"
