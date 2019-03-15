use logos::Logos;

#[cfg(test)]
mod tests {
    use jojolisp::Lexer;

    #[test]
    fn check_lexer() {
        assert_eq!(1,Lexer::new("").token);
        assert_eq!(1,Lexer::new("r#### {} [] 123 ### ####").token);
    }
}

#[derive(Logos, Debug, PartialEq)]
enum LogosToken {
    #[end]
    End,
    #[error]
    Error,
    #[regex=r#""([^"\\]|\\t|\\u|\\n|\\0x[0-9a-fA-F]+|\\[0-9]+|\\")*""#]
    LiteralString,
    #[regex="[-+]?[0-9]+[a-zA-Z-_]*"]
    LiteralInteger,
    #[token="("]
    ParensOpen,
    #[token=")"]
    ParensClose,
    #[token="{"]
    RecordOpen,
    #[token="}"]
    RecordClose,
    #[token="["]
    ArrayOpen,
    #[token="]"]
    ArrayClose,
    #[regex="[a-z0-9]+#+"]
    EscapeOpen,
    #[regex=r#"[;:|/\\<>,\._=~^a-zA-Z'$%&][^\s]*"#]
    Symbol,
}

pub struct StartIndex(usize);
pub struct EndIndex(usize);
pub struct SharpCount(usize);

pub enum Token<'a> {
    LiteralString(&'a str),
    LiteralInteger(&'a str),
    ParensOpen,
    ParensClose,
    RecordOpen,
    RecordClose,
    ArrayOpen,
    ArrayClose,
    ReaderEscape(&'a str, &'a str),
    End,
}

// There are no macros in jojolist, only applicative and operatives
// [1 2 3] ≡ (array 1 2 3)
// "abc" ≡ (packing --stuff--for-unicode-packing-- (array 65 66 67??))
// {.offset_member 1 @ptr_member 12 >"dict member" 77} ≡ (packing ~~~ (record ...))
// tilling [0 (n [1]) (n [2]) (n [3 4 5]) 6 7 (m [8])]
// align ...
// sizes ...
// packing tilling x align x size

pub struct Lexer<'a> {
    inner: logos::Lexer<LogosToken, &'a str>,
    source: &'a str,
    token: Token<'a>,
    inner_pivot_index: usize,
}

impl <'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        let mut lexer = Lexer{inner: LogosToken::lexer(source),
                              source: source,
                              token: Token::End,
                              inner_pivot_index: 0};
        lexer.update_token();
        lexer
    }

    pub fn advance(&mut self) {
        self.inner.advance();
        self.update_token();
    }

    pub fn update_token(&mut self) -> () {
        match self.inner.token {
            LogosToken::End => self.token = Token::End,
            LogosToken::Error => (),
            LogosToken::LiteralString => (),
            LogosToken::LiteralInteger => (),
            LogosToken::ParensOpen => (),
            LogosToken::ParensClose => (),
            LogosToken::RecordOpen => (),
            LogosToken::RecordClose => (),
            LogosToken::ArrayOpen => (),
            LogosToken::ArrayClose => (),
            LogosToken::Symbol => (),
            LogosToken::EscapeOpen => {
                // find the first # character
                // we deduce our ##### string
                // look forward in the source until we get #####
                // remake an inner lexer from there
                let slice = self.inner.slice();
                match slice.bytes().enumerate().find(|(_,b): &(usize, u8)| *b == b'#') {
                    Some((first_sharp, _)) => {
                        let rest_of_source = &self.source[self.inner_pivot_index+self.inner.range().end+1..];
                        let the_sharps = &self.inner.slice()[first_sharp..];
                        match rest_of_source.find(the_sharps) {
                            Some(start_of_close) => {
                                let reader_select = &self.inner.slice()[0..first_sharp];
                                let escaped_content = &rest_of_source[0..start_of_close];
                                self.token = Token::ReaderEscape(reader_select, escaped_content);
                                self.inner_pivot_index = self.inner_pivot_index + self.inner.range().end + 1
                                    + start_of_close + the_sharps.len();
                                self.inner = LogosToken::lexer(&self.source[self.inner_pivot_index..]);
                            },
                            None => {
                                self.token = Token::End
                                },
                        }
                    },
                    None => {
                        self.token = Token::End
                    }
                }
            },
        }
    }
}
