pub mod parser {
    use crate::{
        ast::{Ast, BinaryOperator, ComparisonOperator, Node},
        lexer::lexer::{tokenise, Token, TokenKind},
    };

    pub struct Parser {
        tokens: Vec<Token>,
        idx: usize,
    }

    impl Parser {
        pub fn new(source: String) -> Parser {
            let tokens = tokenise(source);
            Parser { tokens, idx: 0 }
        }

        fn expect(&self, kind: TokenKind) {
            if self.tokens[self.idx].kind != kind {
                panic!(
                    "Expected {:?}, but found {:?}",
                    kind, self.tokens[self.idx].kind
                );
            }
        }

        fn expect_multiple(&self, kinds: Vec<TokenKind>) {
            if !kinds.contains(&self.tokens[self.idx].kind) {
                panic!(
                    "Expected one of: {:?} but found {:?}",
                    kinds, self.tokens[self.idx].kind
                );
            }
        }

        fn match_single(&self, kind: TokenKind) -> bool {
            self.tokens[self.idx].kind == kind
        }

        fn match_multiple(&self, kinds: Vec<TokenKind>) -> bool {
            kinds.contains(&self.tokens[self.idx].kind)
        }

        fn advance(&mut self) {
            self.idx += 1;
        }

        fn parse_variable(&self) -> Node {
            Node::Variable {
                name: self.tokens[self.idx].value.as_ref().unwrap().to_string(),
            }
        }

        fn parse_int_literal(&self) -> Node {
            Node::IntLiteral {
                value: i64::from_str_radix(
                    &self.tokens[self.idx].value.as_ref().unwrap().to_string(),
                    10,
                )
                .unwrap(),
            }
        }

        fn parse_let(&mut self) -> Node {
            self.expect(TokenKind::Let);
            self.advance();
            self.expect(TokenKind::Identifier);

            let target = self.tokens[self.idx].value.as_ref().unwrap().to_string();

            self.advance();
            self.expect(TokenKind::Eq);
            self.advance();

            self.expect(TokenKind::IntLiteral);
            let value = self.parse_int_literal();

            Node::Declaration {
                target,
                value: Box::new(value),
            }
        }

        fn parse_condition(&mut self) -> Node {
            let lhs = match self.tokens[self.idx].kind {
                TokenKind::Identifier => self.parse_variable(),
                TokenKind::IntLiteral => self.parse_int_literal(),
                _ => panic!("Not Implemented"),
            };
            self.advance();

            let op = match self.tokens[self.idx].kind {
                TokenKind::DoubleEq => ComparisonOperator::Equal,
                _ => panic!("Not Implemented"),
            };
            self.advance();

            let rhs = match self.tokens[self.idx].kind {
                TokenKind::Identifier => self.parse_variable(),
                TokenKind::IntLiteral => self.parse_int_literal(),
                _ => panic!("Not Implemented"),
            };

            Node::ComparisonOp {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op,
            }
        }

        fn parse_statement(&mut self) -> Node {
            let lhs = match self.tokens[self.idx].kind {
                TokenKind::Identifier => self.parse_variable(),
                TokenKind::IntLiteral => self.parse_int_literal(),
                _ => panic!(
                    "Expected: {:?}, but found: {:?}",
                    vec![TokenKind::Identifier, TokenKind::IntLiteral],
                    self.tokens[self.idx].kind
                ),
            };
            self.advance();

            let op = match self.tokens[self.idx].kind {
                TokenKind::Plus => BinaryOperator::Plus,
                TokenKind::Minus => BinaryOperator::Minus,
                _ => panic!(
                    "Expected: {:?}, but found: {:?}",
                    vec![TokenKind::Plus, TokenKind::Minus],
                    self.tokens[self.idx].kind
                ),
            };
            self.advance();

            let rhs = match self.tokens[self.idx].kind {
                TokenKind::Identifier => self.parse_variable(),
                TokenKind::IntLiteral => self.parse_int_literal(),
                _ => panic!(
                    "Expected: {:?}, but found: {:?}",
                    vec![TokenKind::Identifier, TokenKind::IntLiteral],
                    self.tokens[self.idx].kind
                ),
            };
            Node::BinaryOp {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op,
            }
        }

        fn parse_assignment(&mut self) -> Node {
            self.expect(TokenKind::Identifier);
            let target = self.parse_variable();
            self.advance();

            self.expect(TokenKind::Eq);
            self.advance();

            Node::Assignment {
                target: Box::new(target),
                value: Box::new(value),
            }
        }

        fn parse_body(&mut self) -> Vec<Node> {
            let mut nodes: Vec<Node> = vec![];

            while !self.match_single(TokenKind::RBrace) {
                nodes.push(self.parse_statement());
            }

            nodes
        }

        fn parse_while(&mut self) -> Node {
            self.expect(TokenKind::While);
            self.advance();
            let cond = self.parse_condition();
            self.advance();
            self.expect(TokenKind::LBrace);
            self.advance();
            let body = self.parse_body();

            Node::While {
                condition: Box::new(cond),
                body: Box::new(body),
            }
        }

        pub fn parse(&mut self) -> Ast {
            let mut ast = Ast::new();
            while self.idx < self.tokens.len() {
                ast.nodes.push(match self.tokens[self.idx].kind {
                    TokenKind::Let => self.parse_let(),
                    TokenKind::While => self.parse_while(),
                    _ => Node::IntLiteral { value: 1 },
                });
                self.idx += 1;
            }

            ast
        }
    }
}
