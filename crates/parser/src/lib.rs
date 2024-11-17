use nom::{
    branch::alt, bytes::complete::{is_not, tag, take_while1}, character::complete::{char, multispace0, one_of, space0, space1}, combinator::{opt, peek}, error::{Error, ErrorKind}, multi::{many0, many1, separated_list0}, sequence::{delimited, pair, preceded, terminated, tuple}, Err, IResult, InputLength
};

use cogs_ast::{Attribute, CodeBlock, Component, Element, HtmlTag};

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
    alt((parse_html, parse_code_block))(input)
}

fn parse_proper_element(input: &str) -> IResult<&str, Element> {
    // dbg!(&input);
    let res = alt((parse_element, parse_text))(input);
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

fn parse_attribute(input: &str) -> IResult<&str, Attribute> {
    // Shouldn't be needed with take_while1, re-add if fail
    /*
    if input.starts_with('>') || input.is_empty() {
        // Return a recoverable error so `separated_list0` stops parsing cleanly
        return Err(Err::Error(Error::new(input, nom::error::ErrorKind::Eof)));
    }
    */

    let (input, key) = take_while1(is_valid_attr_char)(input)?;
    let (input, value) = opt(preceded(
        tuple((tag("="), space0)),
        delimited(tag("\""), is_not("\""), tag("\""))
    ))(input)?;

    let mut resulting_value = None;

    if value.is_some_and(|x| !x.is_empty()) {
        resulting_value = Some(Element::Text(value.unwrap().to_string()));
    }

    Ok((
        input,
        Attribute {
            name: Element::Text(key.to_string()),
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

    // dbg!(&attrs);

    Ok((
        input,
        attrs
    ))
}

fn parse_inside_html_opening_tag(input: &str) -> IResult<&str, HtmlTag> {
    let (input, tag) = parse_tag_name(input)?;
    // dbg!(&tag);
    let (input, _) = space0(input)?;
    let (input, attributes) = parse_attributes(input)?;
    // dbg!(&attributes);

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

    // dbg!(&tag);

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


fn parse_text(input: &str) -> IResult<&str, Element> {
    // dbg!(&input);
    let mut index = 0;
    while index < input.len() {
        let current_slice = &input[index..];

        
        if peek(parse_element)(current_slice).is_ok() {
            // dbg!(&current_slice);
            break;
        }
        
        if peek(parse_html_opening_tag)(current_slice).is_ok() ||
           peek(parse_html_closing_tag)(current_slice).is_ok() ||
           peek::<_, _, Error<&str>, _>(char('}'))(current_slice).is_ok(){
                // dbg!(&current_slice);
                break; // Stop if any of these parsers match
        }

        index += 1; // Increment to check the next character
    }

    if input[0..index].is_empty() {
        // dbg!(&input[0..index]);
        return Err(Err::Error(Error::new(input, ErrorKind::Eof)));
    }

    // dbg!(&input[0..index]);

    Ok((&input[index..], Element::Text(input[0..index].to_string())))
}

fn parse_html_contents(input: &str) -> IResult<&str, Vec<Element>> {
    let (input, out) = parse_consecutive_proper_elements(input)?;

    // dbg!(&out);

    Ok((
        input,
        out
    ))
}

fn parse_html(input: &str) -> IResult<&str, Element> {
    let (input, _) = multispace0(input)?; // remove spaces when debugging is complete
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

/*
fn parse_code_until_interrupted(input: &str) -> IResult<&str, Element> {
    let (input, code) = pair(is_not("{};"), one_of("{};"))(input)?;

    Ok((
        input,
        Element::Text(format!("{}{}", code.0, code.1))
    ))
}
*/

fn parse_inside_code_block(input: &str) -> IResult<&str, Vec<Element>> {
    println!("Attempting inside code block {input}");
    let (input, elems) = parse_consecutive_proper_elements(input)?;

    dbg!(&elems);
    Ok((
        input,
        elems
    ))

}


fn parse_code_block(input: &str) -> IResult<&str, Element> {
    if input.chars().nth(0) == Some('{') {
        println!("Attempting code block on {input}");
    }
    let (input, content) = delimited(
        char('{'),
        parse_inside_code_block, // get here eventually, currently code blocks do not work at all.
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