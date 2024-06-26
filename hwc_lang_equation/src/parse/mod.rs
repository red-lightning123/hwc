use crate::lex::Token;

pub fn parse_geo_proof_tokens(tokens: &[Token]) -> Result<GeoProof, String> {
    let mut new_tokens = tokens;
    let file = GeoProof::parse_mut_tokens(&mut new_tokens);
    match file {
        Ok(file) => {
            if new_tokens.is_empty() {
                Ok(file)
            } else {
                Err(format!(
                    "while parsing {}: some tokens weren't consumed. text dump:\n{}",
                    "equation",
                    crate::highlight_tokens(tokens)
                ))
            }
        }
        Err(err) => Err(format!(
            "while parsing {}: parser error: {}. text dump:\n{}",
            "equation",
            err,
            crate::highlight_tokens(tokens)
        )),
    }
}

pub fn parse_multiline_stmts_tokens(tokens: &[Token]) -> Result<MultilineStmts, String> {
    let mut new_tokens = tokens;
    let file = MultilineStmts::parse_mut_tokens(&mut new_tokens);
    match file {
        Ok(file) => {
            if new_tokens.is_empty() {
                Ok(file)
            } else {
                Err(format!(
                    "while parsing {}: some tokens weren't consumed. text dump:\n{}",
                    "equation",
                    crate::highlight_tokens(tokens)
                ))
            }
        }
        Err(err) => Err(format!(
            "while parsing {}: parser error: {}. text dump:\n{}",
            "equation",
            err,
            crate::highlight_tokens(tokens)
        )),
    }
}

pub fn parse_stmts_tokens(tokens: &[Token]) -> Result<Stmts, String> {
    let mut new_tokens = tokens;
    let file = Stmts::parse_mut_tokens(&mut new_tokens);
    match file {
        Ok(file) => {
            if new_tokens.is_empty() {
                Ok(file)
            } else {
                Err(format!(
                    "while parsing {}: some tokens weren't consumed. text dump:\n{}",
                    "equation",
                    crate::highlight_tokens(tokens)
                ))
            }
        }
        Err(err) => Err(format!(
            "while parsing {}: parser error: {}. text dump:\n{}",
            "equation",
            err,
            crate::highlight_tokens(tokens)
        )),
    }
}

pub fn parse_expr_tokens(tokens: &[Token]) -> Result<Expr, String> {
    let mut new_tokens = tokens;
    let file = Expr::parse_mut_tokens(&mut new_tokens);
    match file {
        Ok(file) => {
            if new_tokens.is_empty() {
                Ok(file)
            } else {
                Err(format!(
                    "while parsing {}: some tokens weren't consumed. text dump:\n{}",
                    "equation",
                    crate::highlight_tokens(tokens)
                ))
            }
        }
        Err(err) => Err(format!(
            "while parsing {}: parser error: {}. text dump:\n{}",
            "equation",
            err,
            crate::highlight_tokens(tokens)
        )),
    }
}

macro_rules! define_parser_combinator {
    ( $name:ident, $parse_type:ty ) => {
        type ParseType = $parse_type;

        #[derive(Debug)]
        pub struct $name(pub <ParseType as ParseTokens>::Output);

        impl ParseTokens for $name {
            type Output = Self;
            fn parse_mut_tokens<'a>(tokens: &mut &'a [Token<'a>]) -> Result<Self::Output, String> {
                Ok(Self(ParseType::parse_mut_tokens(tokens)?))
            }
        }
    };
}

mod combinators;
pub use combinators::any_variants;
use combinators::{Any, Group, Optional, Repeat};

mod parse_tokens;
use parse_tokens::ParseTokens;

mod geo_proof;
pub use geo_proof::GeoProof;

mod geo_step;
pub use geo_step::GeoStep;

mod geo_expl;
pub use geo_expl::GeoExpl;

mod multiline_stmts;
pub use multiline_stmts::MultilineStmts;

mod stmts;
pub use stmts::Stmts;

mod stmt;
pub use stmt::Stmt;

mod rel_stmt;
pub use rel_stmt::RelStmt;

mod is_stmt;
pub use is_stmt::IsStmt;

mod expr;
pub use expr::Expr;

mod sum;
pub use sum::Sum;

mod prod;
pub use prod::Prod;

mod neg;
pub use neg::Neg;

mod exp;
pub use exp::Exp;

mod der;
pub use der::Der;

mod brack;
pub use brack::Brack;

mod bracked_args;
pub use bracked_args::BrackedArgs;

mod expr_args;
pub use expr_args::ExprArgs;

mod sum_op;
pub use sum_op::SumOp;

mod prod_op;
pub use prod_op::ProdOp;

mod neg_op;
pub use neg_op::NegOp;

mod exp_op;
pub use exp_op::ExpOp;

mod der_op;
pub use der_op::DerOp;

mod expr_rel;
pub use expr_rel::ExprRel;

mod stmt_rel;
pub use stmt_rel::StmtRel;

mod single_token;
pub use single_token::{
    ApproxEquals, At, Caret, Comma, Equals, Ge, Gt, Hash, InvEquals, InvRArrow, Is, LBrace, LParen,
    Le, Lt, Minus, MinusPlus, Newline, NotEquals, Plus, PlusMinus, RArrow, RBrace, RParen, Slash,
    Star, Tag, Value,
};
