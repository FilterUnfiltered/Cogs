use nom::{
    IResult,
    character::{
        complete::{char, multispace0},
        streaming::none_of
    },
    combinator::{opt, value},
    branch::alt,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated},
};

use cogs_ast::{Component, Element, HtmlTag, Attribute, CodeBlock};

pub fn parse_cog(input: &str) -> IResult<&str, Component> {
    let (input, elements) = many0(parse_element)(input)?;
    Ok((input, Component { elements }))
}

fn parse_element(input: &str) -> IResult<&str, Element> {
    alt((parse_html_tag, parse_code_block))(input)
}

fn parse_html_tag(input: &str) -> IResult<&str, Element> {
    let (input, (tag, (attributes, content))) = delimited(
        char('<'),
        pair(
            terminated(many1(none_of(" >/\n")), many0(multispace0)),
            pair(
                opt(delimited(char(' '), many1(parse_attribute), char('>'))),
                many0(parse_element),
            ),
        ),
        char('>'),
    )(input)?;

    println!("got result {tag:?}");

    Ok((
        input,
        Element::Html(HtmlTag {
            tag: tag.iter().collect(),
            attributes: attributes.unwrap(),
            content,
        }),
    ))
}

fn parse_attribute(input: &str) -> IResult<&str, Attribute> {
    println!("Attempting to parse attribute");
    let (input, (name, value)) = pair(
        many1(none_of("= ")),
        preceded(char('='), delimited(char('"'), many0(none_of("\"")), char('"'))),
    )(input)?;

    Ok((
        input,
        Attribute {
            name: name.iter().collect(),
            value: value.iter().collect(),
        },
    ))
}

fn parse_code_block(input: &str) -> IResult<&str, Element> {
    let (input, (is_async, content)) = delimited(
        char('{'),
        pair(
            opt(value(true, preceded(char('a'), char('s')))),
            many1(none_of("}")),
        ),
        char('}'),
    )(input)?;

    Ok((
        input,
        Element::Block(CodeBlock {
            is_async: is_async.unwrap_or(false),
            content: content.into_iter().collect(),
        }),
    ))
}