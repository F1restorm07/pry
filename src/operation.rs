use std::rc::Rc;
use logos::{ Logos, Span };
use fst::automaton::Complement;
use fst::Automaton;
use regex_automata::dfa::{ StartKind, dense::{ Config, DFA } };
use regex_automata::util::syntax;

use crate::filter::Filter;

/// the different sections of the query
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationKind {
    /// two words seaprated by OR token
    Or(Vec<OperationKind>),
    /// word prefixed by Exact token
    Exact(String),
    /// plain old word
    Subsequence(String),
    /// word prefixed with Prefix token
    Prefix(String),
    /// word or operation prefixed with Inverse token
    Inverse(Rc<OperationKind>),
    
}

// use builtin automations in fst
// use regex_automata (could open door for full regex support in future) <- implement with low
// priority
#[derive(Debug)]
pub struct Operation {
    operations: Vec<OperationKind>,
}

impl Operation{
    // construct the query operations 
    pub fn build(tokens: Vec<Token>) -> Self {
        // first create the operations
        let mut tokens = tokens.iter().peekable();
        let mut operations: Vec<OperationKind> = Vec::new();

        // stream the tokens
        while let Some(token) = tokens.next() {
            let default = &&Token::default();
            let next_token = tokens.peek().unwrap_or(default);

            match token.kind {
                TokenKind::Fuzzy => operations.push(OperationKind::Subsequence(token.text.clone())),
                TokenKind::Exact if next_token.kind == TokenKind::Fuzzy => {
                    operations.push(OperationKind::Exact(next_token.text.clone()));
                    tokens.next();
                }
                TokenKind::Prefix if next_token.kind == TokenKind::Fuzzy => {
                    operations.push(OperationKind::Prefix(next_token.text.clone()));
                    tokens.next();
                },
                // pop last operation off total vector, and place next operation into vector
                TokenKind::OR => {
                    // println!("or\nnext: {:?}", next_token);
                    let mut or_group: Vec<OperationKind> = Vec::new();
                    
                    if let OperationKind::Or(ops) = operations.last().unwrap() {
                        or_group = ops.to_owned();
                        println!("{:?}", operations.pop());
                    } else {
                        or_group.push(operations.pop().unwrap());
                    }
                    
                    match next_token.kind {
                        TokenKind::Fuzzy => or_group.push(OperationKind::Subsequence(next_token.text.clone())),
                        TokenKind::Exact => {
                            tokens.next();
                            let next_token = tokens.peek().unwrap_or(default);
                            or_group.push(OperationKind::Exact(next_token.text.clone()));
                        }
                        TokenKind::Prefix => {
                            tokens.next();
                            let next_token = tokens.peek().unwrap_or(default);
                            or_group.push(OperationKind::Prefix(next_token.text.clone()));
                        }
                        TokenKind::Inverse => {
                            tokens.next();
                            let next_token = tokens.peek().unwrap_or(default);
                            
                            match next_token.kind {
                                TokenKind::Fuzzy => or_group.push(OperationKind::Inverse(Rc::new(OperationKind::Subsequence(next_token.text.clone())))),
                                TokenKind::Exact => {
                                    tokens.next();
                                    let next_token = tokens.peek().unwrap_or(default);
                                    or_group.push(OperationKind::Inverse(Rc::new(OperationKind::Exact(next_token.text.clone()))));
                                }
                                TokenKind::Prefix => {
                                    tokens.next();
                                    let next_token = tokens.peek().unwrap_or(default);
                                    or_group.push(OperationKind::Inverse(Rc::new(OperationKind::Prefix(next_token.text.clone()))));
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    operations.push(OperationKind::Or(or_group));
                    tokens.next();
                },
                TokenKind::Inverse => {
                    // let next_token = next_token.clone();
                    match next_token.kind {
                        TokenKind::Fuzzy => operations.push(OperationKind::Inverse(Rc::new(OperationKind::Subsequence(next_token.text.clone())))),
                        TokenKind::Exact => {
                            tokens.next();
                            let next_token = tokens.peek().unwrap_or(default);
                            operations.push(OperationKind::Inverse(Rc::new(OperationKind::Exact(next_token.text.clone()))));
                        }
                        TokenKind::Prefix => {
                            tokens.next();
                            let next_token = tokens.peek().unwrap_or(default);
                            operations.push(OperationKind::Inverse(Rc::new(OperationKind::Prefix(next_token.text.clone()))));
                        }
                        _ => {}
                    }
                    tokens.next();
                }
                _ => {}
            }
        }

        Self { operations }

    }

    pub fn automate(&self) -> Vec<AutomatonKind>
    {
        let mut automatons: Vec<AutomatonKind> = Vec::new();
        let mut dfa_builder = DFA::builder()
            .syntax(syntax::Config::new().case_insensitive(true))
            .configure(DFA::config().start_kind(StartKind::Anchored));

        for operation in &self.operations {
            match operation {
                OperationKind::Exact(query) => {
                    let query = format!(".*{query}.*");
                    let dfa = dfa_builder.build(query.as_str()).unwrap();
                    automatons.push(AutomatonKind::Str(dfa));
                }
                OperationKind::Subsequence(query) => {
                    let chars = query.chars().map(|c: char| format!("{c}.*")).collect::<String>();
                    let query = format!(".*{chars}.*");
                    let dfa = dfa_builder.build(query.as_str()).unwrap();
                    automatons.push(AutomatonKind::Subsequence(dfa));
                }
                OperationKind::Prefix(query) => {
                    let query = format!("{query}.*");
                    let dfa = dfa_builder.build(query.as_str()).unwrap();
                    automatons.push(AutomatonKind::Prefix(dfa))
                }
                
                OperationKind::Inverse(op) => {
                    match op.as_ref() {
                        OperationKind::Exact(query) => {
                            // let query = format!(".*{query}.*");
                            // escaping the exact number match \/\/\/
                            let query = format!(".*([^{query}]{{{0}}}).*", query.len());
                            let dfa = dfa_builder.build(query.as_str()).unwrap();
                            automatons.push(AutomatonKind::Inverse(dfa));
                        }
                        OperationKind::Subsequence(query) => {
                            let chars = query.chars().map(|c: char| format!("[^{c}].*")).collect::<String>();
                            let query = format!(".*{chars}");
                            let dfa = dfa_builder.build(query.as_str()).unwrap();
                            automatons.push(AutomatonKind::Inverse(dfa));
                        }
                        OperationKind::Prefix(query) => {
                            let query = format!("[^{query}]{{{0}}}.*", query.len());
                            let dfa = dfa_builder.build(query.as_str()).unwrap();
                            automatons.push(AutomatonKind::Inverse(dfa));
                        }
                        _ => {}
                   }
                }
                OperationKind::Or(ops) => {
                    let or_vec: Vec<AutomatonKind> = vec![];
                    let or_group = DFA::builder()
                        .syntax(syntax::Config::new().case_insensitive(true))
                        .configure(DFA::config().start_kind(StartKind::Anchored));
                    let mut full_query = String::new();
                    for op in ops {
                        match op {
                        OperationKind::Exact(query) => {
                            or_vec.push(AutomatonKind::Str(fst::automaton::Str::new(query)));
                            full_query.push_str(format!("|.*{query}.*").as_str());
                        }
                        OperationKind::Subsequence(query) => {
                            let chars = query.chars().map(|c: char| format!("{c}.*")).collect::<String>();
                            full_query.push_str(format!("|.*{chars}").as_str());
                        }
                        OperationKind::Prefix(query) => {
                            full_query.push_str(format!("|{query}.*").as_str());
                        }
                        OperationKind::Inverse(op) => {
                            match op.as_ref() {
                                OperationKind::Exact(query) => {
                                    full_query.push_str(format!("|(.[^{query}]+)+.").as_str());
                                    let dfa = dfa_builder.build(query.as_str()).unwrap();
                                }
                                OperationKind::Subsequence(query) => {
                                    full_query.push_str(format!("|[^{query}]+").as_str());
                                    let dfa = dfa_builder.build(query.as_str()).unwrap();
                                }
                                OperationKind::Prefix(query) => {
                                    full_query.push_str(format!("|[^{query}]{{{0}}}.*", query.len()).as_str());
                                }
                                _ => {}
                            }
                        }
                        _ => {}

                        }
                    }
                    automatons.push(AutomatonKind::Or(
                            DFA::builder()
                                .syntax(syntax::Config::new().case_insensitive(true))
                                .configure(DFA::config().start_kind(StartKind::Anchored))
                                .build(full_query.split_at(1).1).unwrap()
                            )
                        )
                }
            }
        }
        automatons
    }
}

#[derive(Logos, Debug, Clone, PartialEq, Eq, Default)]
// replicate fzf query syntax
pub enum TokenKind {
    #[regex("[A-Za-z0-9]*")]
    Fuzzy,
    #[token("\'")]
    Exact,
    #[token("^")]
    Prefix,
    // #[token("$", logos::skip)] // not sure how to create this syntax <- can't use anchors
    // (should remove entirely most likely)
    // Suffix,
    #[token("|")]
    OR,
    #[token("!")]
    Inverse,
    #[default]
    #[regex(r"[ \t\f\n]+", logos::skip)]
    Whitespace,
}

/// an individual token
#[derive(Debug, Default)]
pub struct Token {
    kind: TokenKind,
    text: String,
    _span: Span, // placement of token
}

#[derive(Debug)]
pub enum AutomatonKind {
    Str(DFA<Vec<u32>>),
    Subsequence(DFA<Vec<u32>>),
    Prefix(DFA<Vec<u32>>),
    Inverse(DFA<Vec<u32>>),
    // Or(DFA<Vec<u32>>),
    Or(fst::automaton::Union<_, _>)
}



// impl<'f> AutomatonKind<'f> {
//     // fn inner(automaton: Self) -> impl fst::Automaton {
//     //
//     // }
// }


// take the input query and run it through a lexer to output a set of tokens
pub fn tokenize(query: &str) -> Vec<Token> {
    let mut lex = TokenKind::lexer(query);
    let mut tokens = Vec::new();

    while let Some(kind) = lex.next() {
        tokens.push(
            Token {
                kind: kind.unwrap(),
                text: lex.slice().to_string(),
                _span: lex.span(),
            }
        );
    }
    tokens
}

// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_build_subseq() {
//         assert_eq!(OperationKind::build(tokenize("rkyv")).pop(), Some(OperationKind::Subsequence(String::from("rkyv"))))
//     }
//     #[test]
//     fn test_automate_str() {
//         // assert_eq!(OperationKind::automate(OperationKind::build(tokenize("'rkyv"))), vec![AutomatonKind::Str(DenseDFA::new("rkyv"))])
//         println!("{:?}", OperationKind::automate(OperationKind::build(tokenize("'rkyv"))));
//     }
// }
