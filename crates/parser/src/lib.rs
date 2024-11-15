use nom::{
    branch::alt, bytes::complete::{is_not, tag, take_while1}, character::complete::{char, multispace0, space0, space1}, combinator::{opt, peek}, error::{Error, ErrorKind}, multi::{many0, many1, separated_list0}, sequence::{delimited, pair, preceded, terminated, tuple}, Err, IResult, InputLength
};

use cogs_ast::{Attribute, CodeBlock, Component, Element, HtmlContent, HtmlTag, TextOrCode};

pub fn parse_cog(input: &str) -> IResult<&str, Component> {
    let (input, elements) = parse_consecutive_elements(input)?;
    Ok((input, Component { elements }))
}

pub fn parse_consecutive_elements(input: &str) -> IResult<&str, Vec<Element>> {
    let (input, _) = multispace0(input)?;
    many0(parse_element)(input)
}

fn parse_element(input: &str) -> IResult<&str, Element> {
    let (input, _) = multispace0(input)?;
    alt((parse_html, parse_code_block))(input)
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

fn parse_attribute(input: &str) -> IResult<&str, Attribute> {
    if input.starts_with('>') || input.is_empty() {
        // Return a recoverable error so `separated_list0` stops parsing cleanly
        return Err(Err::Error(Error::new(input, nom::error::ErrorKind::Eof)));
    }

    let (input, key) = take_while1(is_valid_attr_char)(input)?;
    let (input, value) = opt(preceded(
        tuple((tag("="), space0)),
        delimited(tag("\""), is_not("\""), tag("\""))
    ))(input)?;

    let mut resulting_value = None;

    if value.is_some_and(|x| !x.is_empty()) {
        resulting_value = Some(TextOrCode::Text(value.unwrap().to_string()));
    }

    Ok((
        input,
        Attribute {
            name: TextOrCode::Text(key.to_string()),
            value: resulting_value,
        },
    ))
}


fn parse_attributes(input: &str) -> IResult<&str, Vec<Attribute>> {
    let (input, attrs) = separated_list0(
        pair(alt((char(','), char(' '))), space0),
        parse_attribute,
    )(input)?;

    let (input, _) = space0(input)?; // dump any trailing spaces

    dbg!(&attrs);

    Ok((
        input,
        attrs
    ))
}

fn parse_inside_html_opening_tag(input: &str) -> IResult<&str, HtmlTag> {
    let (input, tag) = parse_tag_name(input)?;
    dbg!(&tag);
    let (input, _) = space0(input)?;
    let (input, attributes) = parse_attributes(input)?;
    dbg!(&attributes);

    Ok((
        input,
        HtmlTag{
            tag: tag.to_string(),
            attributes,
            content: Vec::new()
        }
    ))    
}

fn parse_html_opening_tag(input: &str) -> IResult<&str, HtmlTag> {
    let (input, tag) = delimited(char('<'), parse_inside_html_opening_tag, char('>'))(input)?;

    dbg!(&tag);

    Ok((
        input,
        tag
    ))
}

fn parse_html_closing_tag(input: &str) -> IResult<&str, &str> {
    let (input, tag) = delimited(tag("</"), parse_tag_name, char('>'))(input)?;

    Ok((
        input,
        tag
    ))
}


fn parse_text(input: &str) -> IResult<&str, &str> {
    dbg!(&input);
    let mut index = 0;
    while index < input.len() {
        let current_slice = &input[index..];

        
        if peek(parse_consecutive_elements)(current_slice).is_ok() && !peek(parse_consecutive_elements)(current_slice).unwrap().1.is_empty() {
            dbg!(&current_slice);
            break;
        }
        
        if peek(parse_html_closing_tag)(current_slice).is_ok() {
            dbg!(&current_slice);
            break; // Stop if any of these parsers match
        }

        index += 1; // Increment to check the next character
    }
    dbg!(&input[0..index]);

    Ok((&input[index..], &input[0..index]))
}

fn parse_single_html_content(input: &str) -> IResult<&str, HtmlContent> {
    let (input, content) = alt((
        |input| parse_element(input).map(|(next, res)| (next, HtmlContent::Element(res))),
        |input| parse_text(input).map(|(next, res)| (next, HtmlContent::Text(res.to_string()))),
    ))(input)?;

    match content.clone() {
        HtmlContent::Text(text) => {
            if text.is_empty() {
                return Err(Err::Error(Error::new(input, ErrorKind::NonEmpty)));
            }
        },
        HtmlContent::Element(_) => {}
    }

    Ok((
        input,
        content
    ))
}

fn parse_html_contents(input: &str) -> IResult<&str, Vec<HtmlContent>> {
    let (input, out) = many0(parse_single_html_content)(input)?;

    dbg!(&out);

    Ok((
        input,
        out
    ))
}

fn parse_html(input: &str) -> IResult<&str, Element> {
    let (input, mut htag) = parse_html_opening_tag(input)?;
    let (input, content) = parse_html_contents(input)?; // parse_consecutive_elements(input)?;
    htag.content = content;

    let (input, close_name) = parse_html_closing_tag(input)?;
    if htag.tag != close_name {
        return Err(Err::Failure(Error::new(input, ErrorKind::Fail))); // Is there a way to give a custom error message?
    }

    Ok((
        input,
        Element::Html(htag)
    ))
}

fn parse_code_block(input: &str) -> IResult<&str, Element> {
    let (input, content) = delimited(
        char('{'),
        parse_consecutive_elements, // get here eventually, currently code blocks do not work at all.
        char('}'),
    )(input)?;

    Ok((
        input,
        Element::Block(CodeBlock {
            // is_async: is_async.unwrap_or(false),
            content: content
        }),
    ))
}