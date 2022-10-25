use logos::{ Logos, Span, Lexer, Filter };
use fst::{ self, Streamer, IntoStreamer };
use std::fs::File;

fn tokenize(query: &str) -> Vec<Token> {
    let mut lex = TokenKind::lexer(query);
    let mut tokens = Vec::new();

    while let Some(kind) = lex.next() {
        tokens.push(
            Token {
                kind,
                text: lex.slice(),
                span: lex.span(),
            }
        );
    }
    tokens
}

#[derive(Debug, Default)]
pub struct Token<'token> {
    kind: TokenKind,
    text: &'token str,
    span: Span, // placement of token
}

pub struct Query<'query> {
    query: Vec<Token<'query>>,
    limit: usize, // maximum number of results returned (may remove later)
}

// break query into tokens
// group tokens together
// create a union of the fst being searched by the different groups of tokens
// create automata from the groups of tokens
// how to build suffix automata
impl<'query> Query<'query> {
    pub fn new(query: &'query str) -> Self {
        Query { query: tokenize(query), limit: 10 }
    }

    // use the different modes depending on token qroups when building stream
    pub fn build_stream(&'query self, fst: &'query fst::Set<memmap::Mmap>) -> fst::set::Intersection<'query> {
        let mut queries: Vec<fst::set::StreamBuilder<_>> = Vec::new();
        let mut or_queries = Vec::new();
        let mut query = self.query.iter().peekable();
        let fst = Box::new(fst);

        while let Some(token) = query.next() {
            let def = &Token::default();
            let next_token = query.peek().unwrap_or(&def);
            let kind = &token.kind;

            match kind {
                &TokenKind::Fuzzy => {
                    // if next_token.kind == TokenKind::Suffix {
                    //     let automation = QueryAutomation { query: token.text, mode: Mode::Suffix };
                    //     queries.push(fst.search(automation));
                    // } else {
                    println!("test");
                        let automation = QueryAutomation { query: token.text, mode: Mode::Subsequence };
                        queries.push(fst.search(automation));
                    // }
                }
                &TokenKind::Prefix if next_token.kind == TokenKind::Fuzzy => {
                    let automation = QueryAutomation { query: next_token.text, mode: Mode::Prefix };
                    queries.push(fst.search(automation));
                    query.next();
                }
                &TokenKind::Exact if next_token.kind == TokenKind::Fuzzy => {
                    let automation = QueryAutomation { query: next_token.text, mode: Mode::Exact };
                    // let automation = ExactAutomation { query: next_token.text };
                    queries.push(fst.search(automation));
                    query.next();
                }
                // when an or token is encountered, use or_union to create subgroup of queries
                // how to index the OpBuilder to add the last token
                &TokenKind::OR => {
                    or_queries.push(queries.pop().unwrap());
                }
                _ => {}
        
            }
        }
        let or_union = fst::set::OpBuilder::from_iter(or_queries);
        let op = fst::set::OpBuilder::from_iter(queries);
        op.add(or_union.union()).intersection()
        // println!("{:?}", op.);
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    Prefix,
    #[default]
    Subsequence,
    Exact,
    Suffix,
}

// counts number of bytes matched
#[derive(Debug, Clone, Copy)]
pub struct QueryAutomation<'quemation> {
    query: &'quemation str,
    mode: Mode,
}

impl<'quemation> fst::Automaton for QueryAutomation<'quemation> {
    type State = Option<usize>;

    fn start(&self) -> Option<usize> {
        Some(0)
    }
    fn is_match(&self, &state: &Option<usize>) -> bool {
        state == Some(self.query.len())
    }
    fn accept(&self, &state: &Option<usize>, byte: u8) -> Option<usize> {
        if state == Some(!0) {
            return state;
        }
        if state == Some(self.query.len()) {
            return state;
        }
        if byte == self.query.as_bytes()[state.unwrap()] {

            // println!("match {:?}", String::from_utf8(vec![byte]));
            return Some(state.unwrap() + 1);
        }
        // println!("{}|{:?}|{}", state.unwrap(), String::from_utf8(vec![byte]), self.query);
        let exact = if Mode::Exact == self.mode {
            if let Some(state) = state {
                if self.query.chars().nth(state) == Some(byte as char) {
                    println!("match {:?}", String::from_utf8(vec![byte]));
                    return Some(state + 1);
                }
            }
            None
        } else { None };
        match self.mode {
            Mode::Prefix => Some(!0),
            Mode::Subsequence => state,
            Mode::Exact => exact,
            _ => Some(!0)
        }
    }
    fn can_match(&self, &state: &Option<usize>) -> bool {
        state.is_some()
    }
    fn will_always_match(&self, &state: &Option<usize>) -> bool {
        state == Some(self.query.len())
    }
}

pub struct ExactAutomation<'e> {
    query: &'e str,
}

impl<'e> fst::Automaton for ExactAutomation<'e> {
    type State = Option<usize>;

    fn start(&self) -> Self::State {
        Some(0)
    }
    fn is_match(&self, &state: &Self::State) -> bool {
        state == Some(self.query.len())
    }
    fn can_match(&self, &state: &Self::State) -> bool {
        state.is_some()
    }
    fn will_always_match(&self, &state: &Self::State) -> bool {
        state == Some(self.query.len())
    }
    fn accept(&self, &state: &Self::State, byte: u8) -> Self::State {
        if let Some(state) = state {
            if self.query.chars().nth(state) == Some(byte as char) {
                return Some(state + 1);
            }
        }
        None
       
    }
}

pub struct SuffixAutomation<'s> {
    query: &'s str,
}

// find a way to reverse the fst and match starting from final node
// how to use accept function to reverse fst using only single bytes
// impl<'s> fst::Automaton for SuffixAutomation<'s> {
//
// }

#[derive(Logos, Debug, Clone, PartialEq, Default)]
// replicate fzf query syntax
pub enum TokenKind {
    #[regex("[A-Za-z0-9]*")]
    Fuzzy,
    #[token("\'")]
    Exact,
    #[token("^")]
    Prefix,
    #[token("$", logos::skip)] // not sure how to create this syntax
    Suffix,
    #[token("|")]
    OR,
    #[token("!")]
    Inverse,
    #[default]
    #[regex(r"[ \t\f\n]+", logos::skip)]
    #[error]
    Whitespace,
}

// fn exact_prefix<'t>(lex: &'t mut Lexer<'t, TokenKind<'t>>) -> Option<&'t str> {
//     let slice = lex.slice();
//     Some(format!("'{}'", lex.remainder()).as_str())
// }
//
// fn exact_suffix<'t>(lex: &'t mut Lexer<'t, TokenKind<'t>>) -> Option<String> {
//     let slice = lex.slice();
//     Filter::Emit(TokenKind::Exact)
// }
//
fn exact_strip(lex: &mut Lexer<TokenKind>) -> Option<String> {
    let slice = lex.slice().parse::<String>();
    println!("exact");
    Some(slice.unwrap().replace('\'', "").to_string())
}

fn exact_prefix(lex: &mut Lexer<TokenKind>) -> String {
    let slice = lex.slice();
    format!("'{}'", lex.remainder())
}

fn exact_suffix(lex: &mut Lexer<TokenKind>) -> String {
    let slice = lex.slice();
    format!("'{}'", lex.source())
}

pub fn search(query: &str) {
    let mem_map = unsafe { 
        memmap::Mmap::map(&File::open(format!("{}/.cache/map.fst", env!("HOME"))).unwrap()).unwrap()
    };

    let set = fst::Set::new(mem_map).unwrap();
    let set_strs = set.stream().into_strs();
    // let rev_set = set_strs.iter().rev();
    // let mut fst = set.as_fst().stream();
    
    // println!("{rev_set:?}");
    // println!("{:?}", fst::Set::new(fst));
    // println!("{query:#?}");
    let query = Query::new(query);
    let mut stream = query.build_stream(&set);
    
    while let Some(entries) = stream.next() {
        println!("{:?}", String::from_utf8(entries.to_vec()));
        // for e in entries {
        //     println!("{e}");
        // }
    };

}
