use combine::{ Parser, ParseError, Stream, token, many1, many, between, parser::char::{alpha_num,spaces}, choice, attempt, sep_by1, optional };

#[derive(Debug)]
pub enum Query {
    Exact(String),
    Subsequence(String),
    Complement(Box<Self>),
    Prefix(String),
    Suffix(String),
    Union(Vec<Self>),
}

pub fn parse_query<Input>() -> impl Parser<Input, Output = Vec<Query>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    let query_parsers = choice((
        attempt(parse_complement()),
        attempt(parse_exact()),
        attempt(parse_prefix()),
        attempt(parse_suffix()),
        attempt(parse_union()),
        many1(alpha_num()).map(Query::Subsequence)
                        ));

    many(query_parsers.skip(spaces()))
}

pub fn parse_complement<Input>() -> impl Parser<Input, Output = Query>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    println!("inside complement ---");

    let query_parsers = choice((
            attempt(parse_exact()),
            attempt(parse_prefix()),
            attempt(parse_suffix()),
            attempt(many1(alpha_num()).map(Query::Subsequence))
                               ));

    println!("parsed complement ---");

    token('!').with(query_parsers).map(|q| Query::Complement(Box::new(q))).message("complement parse failed")
}

pub fn parse_exact<Input>() -> impl Parser<Input, Output = Query>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    println!("parse exact");

    token('\'').with(many1(alpha_num())).map(Query::Exact).message("exact parse failed")
}

pub fn parse_prefix<Input>() -> impl Parser<Input, Output = Query>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    println!("parse prefix");

    token('^').with(many1(alpha_num())).map(Query::Prefix).message("prefix parse failed")
}

pub fn parse_suffix<Input>() -> impl Parser<Input, Output = Query>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    println!("parse suffix");

    many1(alpha_num()).and(token('$')).map(|q| Query::Suffix(q.0)).message("suffix parse failed")
}

pub fn parse_union<Input>() -> impl Parser<Input, Output = Query>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    println!("inside union ---");

    let query_parsers = choice((
            attempt(parse_exact()),
            attempt(parse_complement()),
            attempt(parse_prefix()),
            attempt(parse_suffix()),
            many1(alpha_num()).map(Query::Subsequence)
                               ));

    println!("parsed union ---");

    sep_by1(query_parsers.skip(optional(spaces())), token('|').skip(optional(spaces()))).map(Query::Union).message("union parse failed")
}
