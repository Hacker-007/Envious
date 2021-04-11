use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::{
        expression::{Expression, ExpressionKind, Identifier},
        Parser,
    },
};

use super::prefix_parselet::PrefixParselet;

macro_rules! get {
    ($token: ident, $pattern: pat, $expression: expr) => {
        if let $pattern = $token.1 {
            $expression
        } else {
            unreachable!()
        };
    };
}

pub struct IdentifierParselet;
impl<'a> PrefixParselet<'a> for IdentifierParselet {
    fn parse(
        &self,
        _: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
        token: Token<'a>,
    ) -> Result<Expression<'a>, Error<'a>> {
        let id = get!(token, TokenKind::Identifier(id), id);
        Ok((token.0, ExpressionKind::Identifier(Identifier(id))))
    }
}