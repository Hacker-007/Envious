use crate::{
    error::Span,
    parser::expression::{BinaryOperation, UnaryOperation},
};

use super::monotype::MonotypeRef;

/// Represents an expression that is generated by the `Parser`.
/// Each expression consists of a span (the location information of the expression)
/// and the kind of the expression.
pub type TypedExpression<'a> = (Span<'a>, TypedExpressionKind<'a>);

/// Enum that details the different types of expressions that can be produced
/// by the `Expression`. The `ExpressionKind` should strive to only store types that
/// are small in nature and any other types (i.e. String) should be stored in the
/// `Interner`.
#[derive(Debug)]
pub enum TypedExpressionKind<'a> {
    Int(i64),
    Float(f64),
    Boolean(bool),
    Char(char),
    // The actual value for the `Identifier` are
    // stored in the `Interner` to reduce redundency in values. Instead,
    // the id's are stored in the variant.
    Identifier(TypedIdentifier),
    Unary(TypedUnary<'a>),
    Binary(TypedBinary<'a>),
    If(TypedIf<'a>),
    Let(TypedLet<'a>),
    Block(Vec<TypedExpression<'a>>),
    Application(TypedApplication<'a>),
    While(TypedWhile<'a>),
    Return(Option<Box<TypedExpression<'a>>>),
}

#[derive(Debug)]
pub struct TypedIdentifier {
    pub id: usize,
    pub ty: MonotypeRef,
}

#[derive(Debug)]
pub struct TypedUnary<'a> {
    pub operation: UnaryOperation,
    pub expression: Box<TypedExpression<'a>>,
    pub ty: MonotypeRef,
}

#[derive(Debug)]
pub struct TypedBinary<'a> {
    pub operation: BinaryOperation,
    pub left: Box<TypedExpression<'a>>,
    pub right: Box<TypedExpression<'a>>,
    pub ty: MonotypeRef,
}

#[derive(Debug)]
pub struct TypedIf<'a> {
    pub condition: Box<TypedExpression<'a>>,
    pub then_branch: Box<TypedExpression<'a>>,
    pub else_branch: Option<Box<TypedExpression<'a>>>,
    pub ty: MonotypeRef,
}

#[derive(Debug)]
pub struct TypedLet<'a> {
    pub name: (Span<'a>, TypedIdentifier),
    pub given_type: Option<MonotypeRef>,
    pub expression: Box<TypedExpression<'a>>,
    pub ty: MonotypeRef,
}

#[derive(Debug)]
pub struct TypedApplication<'a> {
    pub function_name: (Span<'a>, TypedIdentifier),
    pub parameters: Vec<TypedExpression<'a>>,
    pub ty: MonotypeRef,
}

#[derive(Debug)]
pub struct TypedWhile<'a> {
    pub condition: Box<TypedExpression<'a>>,
    pub expression: Box<TypedExpression<'a>>,
}