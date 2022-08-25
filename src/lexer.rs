pub mod lexer {
    use std::collections::HashMap;

    #[derive(Clone, Debug, PartialEq)]
    pub enum TokenKind {
        Let,
        Identifier,
        While,
        IntLiteral,
        DoubleEq,
        Colon,
        If,
        Else,
        Eq,
        Plus,
        PlusEq,
        Minus,
        MinusEq,
        LBrace,
        RBrace,
        Eof
    }

    #[derive(Debug)]
    pub struct Token {
        pub kind: TokenKind,
        pub value: Option<String>,
    }

    impl Token {
        pub fn new(kind: TokenKind, value: Option<String>) -> Token {
            Token { kind, value }
        }
    }

    pub fn tokenise(source: String) -> Vec<Token> {
        let token_kinds: HashMap<String, TokenKind> = HashMap::from([
            ("while".into(), TokenKind::While),
            ("let".into(), TokenKind::Let),
            ("if".into(), TokenKind::If),
            ("else".into(), TokenKind::Else),
            ("=".into(), TokenKind::Eq),
            ("==".into(), TokenKind::DoubleEq),
            ("+".into(), TokenKind::Plus),
            ("+=".into(), TokenKind::PlusEq),
            ("-".into(), TokenKind::Minus),
            ("-=".into(), TokenKind::MinusEq),
            (":".into(), TokenKind::Colon),
            ("{".into(), TokenKind::LBrace),
            ("}".into(), TokenKind::RBrace),
        ]);

        let digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut tokens: Vec<Token> = vec![];
        let mut idx = 0;
        let mut current;
        let mut identifier: String;

        while idx < source.len() {
            current = source.chars().nth(idx).unwrap();
            identifier = String::new();

            while current.is_whitespace() {
                idx += 1;
                current = source.chars().nth(idx).unwrap();
                continue;
            }

            // This is StrLiteral parsing, not wanted ATM
            // if current == '"' {
            //     idx += 1;
            //     current = source.chars().nth(idx).unwrap();
            //     while current != '"' && idx < source.len() {
            //         identifier.push(current);
            //         idx += 1;
            //         current = source.chars().nth(idx).unwrap();
            //     }
            //     idx += 1;
            //     tokens.push(Token::new(TokenKind::Literal(TokenLiteralKind::StrLiteral), Some(identifier)));
            //     continue;
            // }

            match &current.to_digit(10) {
                Some(n) => {
                    if digits.contains(n) {
                        while !current.is_whitespace() && idx < source.len() {
                            identifier.push(current);
                            idx += 1;
                            current = source.chars().nth(idx).unwrap();
                        }
                        idx += 1;
                        tokens.push(Token::new(TokenKind::IntLiteral, Some(identifier)));
                        continue;
                    }
                }
                None => {}
            }

            while !current.is_whitespace() && idx < source.len() {
                identifier.push(current);
                idx += 1;
                current = source.chars().nth(idx).unwrap_or(' ');
            }

            let tk = token_kinds
                .get(&identifier)
                .unwrap_or(&TokenKind::Identifier);

            tokens.push(Token::new(tk.clone(), identifier.into()));

            idx += 1;
        }

        tokens.push(Token::new(TokenKind::Eof, None));

        tokens
    }
}
