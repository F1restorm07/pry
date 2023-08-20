use logos::{ Lexer, Logos, Span };

// key (condition) value
#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConditionKind {
    #[token("=")]
    Equals,
    #[token("!=")]
    Notequals,
    #[token(">")]
    Greaterthan,
    #[token("<")]
    Lessthan,
    #[token(">=")]
    Greaterequals,
    #[token("<=")]
    Lessequals,
    #[regex("from")]
    From,
    #[token("to")]
    To,
    #[regex("[a-zA-Z0-9]*")]
    Ident,
    #[regex(r"[ \t\f\n]+", logos::skip)]
    #[error]
    Whitespace,
}

#[derive(Debug)]
pub struct Condition {
    kind: ConditionKind,
    text: Option<String>,
    _span: Span,
}

pub fn tokenize(filters: &str) -> Vec<Condition> {
    let mut lex = ConditionKind::lexer(filters);
    let mut tokens = Vec::new();

    while let Some(token) = lex.next() {
        tokens.push(Condition {
                kind: token,
                text: if token == ConditionKind::Ident { Some(lex.slice().to_string()) } else { None },
                _span: lex.span(),
            });
    }
    tokens
}

// fn extract_ends(lex: &mut Lexer<ConditionKind>) -> Option<&str> {
//     let slice = lex.slice();
// }
#[derive(Debug, PartialEq, Eq)]
 pub struct Filter {
     key: String,
     value: String,
     condition: ConditionKind,
 }

impl Filter {
    pub fn build(conditions: Vec<Condition>) -> Vec<Self> {
        let tokens = conditions.iter();
        let mut filters: Vec<Self> = Vec::new();

        for (idx, condition) in tokens.enumerate().skip(1) {
            if condition.kind != ConditionKind::Ident {
                filters.push(Filter {
                    key: conditions.get(idx-1).unwrap().text.as_ref().unwrap_or(&"".to_string()).to_string(),
                    value: conditions.get(idx+1).unwrap().text.as_ref().unwrap_or(&"".to_string()).to_string(),
                    condition: condition.kind,
                });
            } else { continue; }
        }
        filters
    }
}
