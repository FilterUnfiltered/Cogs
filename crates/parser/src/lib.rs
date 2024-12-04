use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::complete::{char, multispace0, space0, space1},
    combinator::{opt, peek},
    error::context,
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, tuple},
    Parser,
};
use tracing::debug;

type IResult<I, O> = nom::IResult<I, O, error::Error<I>>;
use error::Error;

use cogs_ast::{Attribute, CodeBlock, CodeElement, Component, Element, Expression, HtmlTag};
// reexport for cogs crate
#[doc(hidden)]
pub use nom;

pub mod error;

pub fn parse_cog(input: &str) -> IResult<&str, Component> {
    let (input, elements) = parse_consecutive_proper_elements(input)?;
    Ok((input, Component { elements }))
}

pub fn parse_consecutive_proper_elements(input: &str) -> IResult<&str, Vec<Element>> {
    let (input, _) = multispace0(input)?;
    let res = many0(parse_proper_element)(input);
    // dbg!(&res);
    res
}

fn parse_element(input: &str) -> IResult<&str, Element> {
    let (input, _) = multispace0(input)?;
    alt((parse_html_tag.map(Element::Html), context("code block", parse_code_block.map(Element::Block))))(input)
}

fn parse_proper_element(input: &str) -> IResult<&str, Element> {
    // dbg!(&input);
    let res = alt((parse_element, parse_text.map(Element::Text)))(input);
    // dbg!(&res);
    res
}

fn is_valid_tag_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-'
}

fn is_valid_attr_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_'
}

fn parse_tag_name(input: &str) -> IResult<&str, &str> {
    take_while1(is_valid_tag_name_char)(input)
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        delimited(char('{'), is_not("{}"), char('}'))
            .map(|code: &str| Expression::Code(code.to_string())),
        delimited(char('"'), is_not("\""), char('"'))
            .map(|code: &str| Expression::Text(code.to_string())),
    ))(input)
}

fn parse_attribute(input: &str) -> IResult<&str, Attribute> {
    // Shouldn't be needed with take_while1, re-add if fail
    /*
    if input.starts_with('>') || input.is_empty() {
        // Return a recoverable error so `separated_list0` stops parsing cleanly
        return Err(Error::eof(input));
    }
    */

    let (input, name) = take_while1(is_valid_attr_char)(input)?;
    let (input, value) = opt(preceded(tuple((tag("="), space0)), parse_expression))(input)?;

    Ok((
        input,
        Attribute {
            name: name.to_string(),
            value,
        },
    ))
}

fn parse_attributes(input: &str) -> IResult<&str, Vec<Attribute>> {
    let (input, attrs) =
        separated_list0(pair(alt((char(','), char(' '))), space0), parse_attribute)(input)?;

    let (input, _) = space0(input)?; // dump any trailing spaces

    // dbg!(&attrs);

    Ok((input, attrs))
}

fn parse_inside_html_opening_tag(input: &str) -> IResult<&str, HtmlTag> {
    let (input, tag) = parse_tag_name(input)?;
    // dbg!(&tag);
    let (input, _) = space0(input)?;
    let (input, attributes) = context("html attributes", parse_attributes)(input)?;
    // dbg!(&attributes);

    Ok((
        input,
        HtmlTag {
            tag: tag.to_string(),
            attributes,
            content: Vec::new(),
        },
    ))
}

fn parse_html_opening_tag(input: &str) -> IResult<&str, HtmlTag> {
    let (input, tag) = delimited(char('<'), parse_inside_html_opening_tag, char('>'))(input)?;

    // dbg!(&tag);

    Ok((input, tag))
}

fn parse_html_closing_tag(input: &str) -> IResult<&str, &str> {
    let (input, tag) = delimited(
        tag("</"),
        context("html tag name", parse_tag_name),
        char('>'),
    )(input)?;

    Ok((input, tag))
}

fn parse_text(input: &str) -> IResult<&str, String> {
    // dbg!(&input);
    let mut index = 0;
    while index < input.len() {
        let current_slice = &input[index..];

        if peek(parse_element)(current_slice).is_ok() {
            // dbg!(&current_slice);
            break;
        }

        if peek(parse_html_opening_tag)(current_slice).is_ok()
            || peek(parse_html_closing_tag)(current_slice).is_ok()
            || peek::<_, _, Error<&str>, _>(char('}'))(current_slice).is_ok()
        {
            // dbg!(&current_slice);
            break; // Stop if any of these parsers match
        }

        index += 1; // Increment to check the next character
    }

    if input[0..index].is_empty() {
        // dbg!(&input[0..index]);
        return Err(Error::eof(input));
    }

    // dbg!(&input[0..index]);

    Ok((&input[index..], input[0..index].to_string()))
}

fn parse_html_contents(input: &str) -> IResult<&str, Vec<Element>> {
    let (input, out) = parse_consecutive_proper_elements(input)?;

    // dbg!(&out);

    Ok((input, out))
}

fn parse_html_tag(input: &str) -> IResult<&str, HtmlTag> {
    let (input, _) = multispace0(input)?; // remove spaces when debugging is complete
    let (input, mut htag) = parse_html_opening_tag(input)?;
    let (input, content) = parse_html_contents(input)?; // parse_consecutive_elements(input)?;
    htag.content = content;

    let (input, close_name) = parse_html_closing_tag(input)?;
    if htag.tag != close_name {
        return Err(Error::custom_failure(
            input,
            format!(
                "expected closing tag `</{}>`, got `</{}>`",
                htag.tag, close_name
            ),
        ));
    }

    Ok((input, htag))
}

fn parse_code_element(input: &str) -> IResult<&str, CodeElement> {
    alt((
        parse_html_tag.map(CodeElement::Html),
        parse_text.map(CodeElement::Code)
    ))(input)
}

fn parse_code_elements(input: &str) -> IResult<&str, Vec<CodeElement>> {
    debug!("Attempting inside code block {input}");

    let (input, _) = multispace0(input)?;
    let (input, elems) = many0(parse_code_element)(input)?;

    debug!(?elems, "parsed inside code block");
    Ok((input, elems))
}

fn parse_code_block(input: &str) -> IResult<&str, CodeBlock> {
    if input.chars().nth(0) == Some('{') {
        debug!("Attempting code block on {input}");
    }
    let (input, content) = delimited(char('{'), parse_code_elements, char('}'))(input)?;

    Ok((
        input,
        CodeBlock {
            // is_async: is_async.unwrap_or(false),
            content,
        },
    ))
}
