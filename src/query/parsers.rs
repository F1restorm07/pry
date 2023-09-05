use super::Query;
use combine::{ Parser, ParseError, Stream, token, many1, many, between, parser::char::{alpha_num,spaces}, choice, attempt, sep_by1, optional, not_followed_by, satisfy };

pub fn parse_query<Input>() -> impl Parser<Input, Output = Vec<Query>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    // cycle through all the parsers
    // check if any parse is followed by a space and | token, if so fail and cycle to union parser
    let query_parsers = choice((
        // attempt(parse_union()),
        // parse_union(),
        attempt(parse_complement().and(not_followed_by(optional(spaces()).with(token('|')))).map(|x| x.0)),
        attempt(parse_exact().and(not_followed_by(optional(spaces()).with(token('|')))).map(|x| x.0)),
        attempt(parse_prefix().and(not_followed_by(optional(spaces()).with(token('|')))).map(|x| x.0)),
        attempt(parse_suffix().and(not_followed_by(optional(spaces()).with(token('|')))).map(|x| x.0)),
        attempt(parse_union()),
        // parse_union(),
        many1(alpha_num()).map(Query::Subsequence)
                        ));

    many(query_parsers
         .skip(spaces()))
}

pub fn parse_complement<Input>() -> impl Parser<Input, Output = Query>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    let query_parsers = choice((
            attempt(parse_exact()),
            attempt(parse_prefix()),
            attempt(parse_suffix()),
            attempt(many1(alpha_num()).map(Query::Subsequence))
                               ));

    token('!').with(query_parsers).map(|q| Query::Complement(Box::new(q))).message("complement parse failed")
}

pub fn parse_exact<Input>() -> impl Parser<Input, Output = Query>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    token('\'').with(many1(alpha_num())).map(Query::Exact).message("exact parse failed")
}

pub fn parse_prefix<Input>() -> impl Parser<Input, Output = Query>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    token('^').with(many1(alpha_num())).map(Query::Prefix).message("prefix parse failed")
}

pub fn parse_suffix<Input>() -> impl Parser<Input, Output = Query>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    many1(alpha_num()).and(token('$')).map(|q| Query::Suffix(q.0)).message("suffix parse failed")
}

pub fn parse_union<Input>() -> impl Parser<Input, Output = Query>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>
{
    let query_parsers = choice((
            attempt(parse_exact()),
            attempt(parse_complement()),
            attempt(parse_prefix()),
            attempt(parse_suffix()),
            many1(alpha_num()).map(Query::Subsequence)
                               ));
    let union_sep = token('|').and(optional(spaces()));
    let union_variant = query_parsers
        .skip(optional(spaces()))
        .message("union variant parser failed");

    sep_by1(union_variant, union_sep.message("union sep parse failed")).map(Query::Union).message("union parse failed")
}
