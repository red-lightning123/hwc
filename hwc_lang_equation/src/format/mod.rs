use crate::parse;

#[derive(Debug, Clone)]
pub struct GeoProof {
    steps: Vec<GeoStep>,
}

impl GeoProof {
    pub fn steps(&self) -> &Vec<GeoStep> {
        &self.steps
    }

    fn new(steps: Vec<GeoStep>) -> GeoProof {
        GeoProof { steps }
    }
}

impl TryFrom<parse::GeoProof> for GeoProof {
    type Error = String;
    fn try_from(geo_proof: parse::GeoProof) -> Result<Self, Self::Error> {
        let parse::GeoProof((geo_proof_first, geo_proof_rep)) = geo_proof;
        let mut steps = vec![];
        let first = GeoStep::try_from(geo_proof_first)?;
        steps.push(first);
        for (_hash, geo_step) in geo_proof_rep {
            steps.push(GeoStep::try_from(geo_step)?);
        }
        Ok(GeoProof::new(steps))
    }
}

#[derive(Debug, Clone)]
pub struct GeoStep {
    explanation: GeoExplanation,
    multiline_statements: MultilineStatements,
}

impl GeoStep {
    pub fn explanation(&self) -> &GeoExplanation {
        &self.explanation
    }

    pub fn multiline_statements(&self) -> &MultilineStatements {
        &self.multiline_statements
    }

    fn new(explanation: GeoExplanation, multiline_statements: MultilineStatements) -> GeoStep {
        GeoStep {
            explanation,
            multiline_statements,
        }
    }
}

impl TryFrom<parse::GeoStep> for GeoStep {
    type Error = String;
    fn try_from(geo_step: parse::GeoStep) -> Result<Self, Self::Error> {
        let parse::GeoStep((geo_step_geo_expl, _, geo_step_multiline_stmts)) = geo_step;
        let explanation = GeoExplanation::try_from(geo_step_geo_expl)?;
        let multiline_statements = MultilineStatements::try_from(geo_step_multiline_stmts)?;
        Ok(GeoStep::new(explanation, multiline_statements))
    }
}

#[derive(Debug, Clone)]
pub struct GeoExplanation {
    expression: Expression,
}

impl GeoExplanation {
    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    fn new(expression: Expression) -> GeoExplanation {
        GeoExplanation { expression }
    }
}

impl TryFrom<parse::GeoExpl> for GeoExplanation {
    type Error = String;
    fn try_from(geo_expl: parse::GeoExpl) -> Result<Self, Self::Error> {
        let parse::GeoExpl(geo_expl_expr) = geo_expl;
        let expression = Expression::try_from(geo_expl_expr)?;
        Ok(GeoExplanation::new(expression))
    }
}

#[derive(Debug, Clone)]
pub struct MultilineStatements {
    items: Vec<MultilineStatementsItem>,
}

impl MultilineStatements {
    pub fn items(&self) -> &Vec<MultilineStatementsItem> {
        &self.items
    }

    fn new(items: Vec<MultilineStatementsItem>) -> MultilineStatements {
        MultilineStatements { items }
    }
}

impl TryFrom<parse::MultilineStmts> for MultilineStatements {
    type Error = String;
    fn try_from(multiline_stmts: parse::MultilineStmts) -> Result<Self, Self::Error> {
        let parse::MultilineStmts((multiline_stmts_first, multiline_stmts_rep)) = multiline_stmts;
        let mut items = vec![];
        items.push(MultilineStatementsItem::Statements(Statements::try_from(
            multiline_stmts_first,
        )?));
        for (newline, stmts) in multiline_stmts_rep {
            items.push(MultilineStatementsItem::Newline(Newline::try_from(
                newline,
            )?));
            items.push(MultilineStatementsItem::Statements(Statements::try_from(
                stmts,
            )?));
        }
        Ok(MultilineStatements::new(items))
    }
}

#[derive(Debug, Clone)]
pub enum MultilineStatementsItem {
    Statements(Statements),
    Newline(Newline),
}

#[derive(Debug, Clone)]
pub struct Newline;

impl TryFrom<parse::Newline> for Newline {
    type Error = String;
    fn try_from(_newline: parse::Newline) -> Result<Self, Self::Error> {
        Ok(Newline)
    }
}

#[derive(Debug, Clone)]
pub struct Statements {
    first: Option<Statement>,
    rep: Vec<(StatementRelation, Statement)>,
    last: Option<StatementRelation>,
}

impl Statements {
    pub fn first(&self) -> &Option<Statement> {
        &self.first
    }

    pub fn rep(&self) -> &Vec<(StatementRelation, Statement)> {
        &self.rep
    }

    pub fn last(&self) -> &Option<StatementRelation> {
        &self.last
    }

    fn new(
        first: Option<Statement>,
        rep: Vec<(StatementRelation, Statement)>,
        last: Option<StatementRelation>,
    ) -> Statements {
        Statements { first, rep, last }
    }
}

impl TryFrom<parse::Stmts> for Statements {
    type Error = String;
    fn try_from(stmts: parse::Stmts) -> Result<Self, Self::Error> {
        let parse::Stmts((stmts_first, stmts_rep, stmts_last)) = stmts;
        let first = match stmts_first {
            Some(stmt) => Some(Statement::try_from(stmt)?),
            None => None,
        };
        let mut rep = vec![];
        for (stmt_rel, stmt) in stmts_rep {
            rep.push((
                StatementRelation::try_from(stmt_rel)?,
                Statement::try_from(stmt)?,
            ));
        }
        let last = match stmts_last {
            Some(stmt_rel) => Some(StatementRelation::try_from(stmt_rel)?),
            None => None,
        };
        Ok(Statements::new(first, rep, last))
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Relational(RelationalStatement),
    Is(IsStatement),
}

impl TryFrom<parse::Stmt> for Statement {
    type Error = String;
    fn try_from(stmt: parse::Stmt) -> Result<Self, Self::Error> {
        let parse::Stmt(stmt_variant) = stmt;
        Ok(match stmt_variant {
            parse::any_variants::AnyVariants2::V1(rel_stmt) => {
                Statement::Relational(RelationalStatement::try_from(rel_stmt)?)
            }
            parse::any_variants::AnyVariants2::V2(is_stmt) => {
                Statement::Is(IsStatement::try_from(is_stmt)?)
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct IsStatement {
    subject: Expression,
    object: Expression,
}

impl IsStatement {
    pub fn subject(&self) -> &Expression {
        &self.subject
    }

    pub fn object(&self) -> &Expression {
        &self.object
    }

    fn new(subject: Expression, object: Expression) -> IsStatement {
        IsStatement { subject, object }
    }
}

impl TryFrom<parse::IsStmt> for IsStatement {
    type Error = String;
    fn try_from(is_stmt: parse::IsStmt) -> Result<Self, Self::Error> {
        let parse::IsStmt((is_stmt_subject, _, is_stmt_object)) = is_stmt;
        let subject = Expression::try_from(is_stmt_subject)?;
        let object = Expression::try_from(is_stmt_object)?;
        Ok(IsStatement::new(subject, object))
    }
}

#[derive(Debug, Clone)]
pub struct RelationalStatement {
    first: Option<Expression>,
    rep: Vec<(ExpressionRelation, Expression)>,
    last: Option<ExpressionRelation>,
}

impl RelationalStatement {
    pub fn first(&self) -> &Option<Expression> {
        &self.first
    }

    pub fn rep(&self) -> &Vec<(ExpressionRelation, Expression)> {
        &self.rep
    }

    pub fn last(&self) -> &Option<ExpressionRelation> {
        &self.last
    }

    fn new(
        first: Option<Expression>,
        rep: Vec<(ExpressionRelation, Expression)>,
        last: Option<ExpressionRelation>,
    ) -> RelationalStatement {
        RelationalStatement { first, rep, last }
    }
}

impl TryFrom<parse::RelStmt> for RelationalStatement {
    type Error = String;
    fn try_from(rel_stmt: parse::RelStmt) -> Result<Self, Self::Error> {
        let parse::RelStmt((rel_stmt_first, rel_stmt_rep, rel_stmt_last)) = rel_stmt;
        let first = match rel_stmt_first {
            Some(expr) => Some(Expression::try_from(expr)?),
            None => None,
        };
        let mut rep = vec![];
        for (expr_rel, expr) in rel_stmt_rep {
            rep.push((
                ExpressionRelation::try_from(expr_rel)?,
                Expression::try_from(expr)?,
            ));
        }
        let last = match rel_stmt_last {
            Some(expr_rel) => Some(ExpressionRelation::try_from(expr_rel)?),
            None => None,
        };
        Ok(RelationalStatement::new(first, rep, last))
    }
}

#[derive(Debug, Clone)]
pub struct Expression {
    sum: Sum,
}

impl Expression {
    pub fn sum(&self) -> &Sum {
        &self.sum
    }

    fn new(sum: Sum) -> Expression {
        Expression { sum }
    }
}

impl TryFrom<parse::Expr> for Expression {
    type Error = String;
    fn try_from(expr: parse::Expr) -> Result<Self, Self::Error> {
        let parse::Expr(expr_sum) = expr;
        let sum = Sum::try_from(expr_sum)?;
        Ok(Expression::new(sum))
    }
}

#[derive(Debug, Clone)]
pub struct Sum {
    first: Product,
    rep: Vec<(SumOp, Product)>,
}

impl Sum {
    pub fn first(&self) -> &Product {
        &self.first
    }

    pub fn rep(&self) -> &Vec<(SumOp, Product)> {
        &self.rep
    }

    fn new(first: Product, rep: Vec<(SumOp, Product)>) -> Sum {
        Sum { first, rep }
    }
}

impl TryFrom<parse::Sum> for Sum {
    type Error = String;
    fn try_from(sum: parse::Sum) -> Result<Self, Self::Error> {
        let parse::Sum((sum_first, sum_rep)) = sum;
        let first = Product::try_from(sum_first)?;
        let mut rep = vec![];
        for (sum_op, product) in sum_rep {
            rep.push((SumOp::try_from(sum_op)?, Product::try_from(product)?));
        }
        Ok(Sum::new(first, rep))
    }
}

#[derive(Debug, Clone)]
pub struct Product {
    first: Negate,
    rep: Vec<(ProductOp, Negate)>,
}

impl Product {
    pub fn first(&self) -> &Negate {
        &self.first
    }

    pub fn rep(&self) -> &Vec<(ProductOp, Negate)> {
        &self.rep
    }

    fn new(first: Negate, rep: Vec<(ProductOp, Negate)>) -> Product {
        Product { first, rep }
    }
}

impl TryFrom<parse::Prod> for Product {
    type Error = String;
    fn try_from(prod: parse::Prod) -> Result<Self, Self::Error> {
        let parse::Prod((prod_first, prod_rep)) = prod;
        let first = Negate::try_from(prod_first)?;
        let mut rep = vec![];
        for (product_op, negate) in prod_rep {
            rep.push((ProductOp::try_from(product_op)?, Negate::try_from(negate)?));
        }
        Ok(Product::new(first, rep))
    }
}

#[derive(Debug, Clone)]
pub struct Negate {
    ops: Vec<NegateOp>,
    exponent: Exponent,
}

impl Negate {
    pub fn ops(&self) -> &Vec<NegateOp> {
        &self.ops
    }

    pub fn exponent(&self) -> &Exponent {
        &self.exponent
    }

    fn new(ops: Vec<NegateOp>, exponent: Exponent) -> Negate {
        Negate { ops, exponent }
    }
}

impl TryFrom<parse::Neg> for Negate {
    type Error = String;
    fn try_from(neg: parse::Neg) -> Result<Self, Self::Error> {
        let parse::Neg((neg_ops, neg_exponent)) = neg;
        let mut ops = vec![];
        for op in neg_ops {
            ops.push(NegateOp::try_from(op)?);
        }
        let exponent = Exponent::try_from(neg_exponent)?;
        Ok(Negate::new(ops, exponent))
    }
}

#[derive(Debug, Clone)]
pub struct Exponent {
    first: Derivate,
    rep: Vec<(ExponentOp, Derivate)>,
}

impl Exponent {
    pub fn first(&self) -> &Derivate {
        &self.first
    }

    pub fn rep(&self) -> &Vec<(ExponentOp, Derivate)> {
        &self.rep
    }

    fn new(first: Derivate, rep: Vec<(ExponentOp, Derivate)>) -> Exponent {
        Exponent { first, rep }
    }
}

impl TryFrom<parse::Exp> for Exponent {
    type Error = String;
    fn try_from(exp: parse::Exp) -> Result<Self, Self::Error> {
        let parse::Exp((exp_first, exp_rep)) = exp;
        let first = Derivate::try_from(exp_first)?;
        let mut rep = vec![];
        for (exponent_op, derivate) in exp_rep {
            rep.push((
                ExponentOp::try_from(exponent_op)?,
                Derivate::try_from(derivate)?,
            ));
        }
        Ok(Exponent::new(first, rep))
    }
}

#[derive(Debug, Clone)]
pub struct Derivate {
    brack: Brack,
    ops: Vec<DerivateOp>,
}

impl Derivate {
    pub fn brack(&self) -> &Brack {
        &self.brack
    }

    pub fn ops(&self) -> &Vec<DerivateOp> {
        &self.ops
    }

    fn new(brack: Brack, ops: Vec<DerivateOp>) -> Derivate {
        Derivate { brack, ops }
    }
}

impl TryFrom<parse::Der> for Derivate {
    type Error = String;
    fn try_from(der: parse::Der) -> Result<Self, Self::Error> {
        let parse::Der((der_brack, der_ops)) = der;
        let brack = Brack::try_from(der_brack)?;
        let mut ops = vec![];
        for op in der_ops {
            ops.push(DerivateOp::try_from(op)?);
        }
        Ok(Derivate::new(brack, ops))
    }
}

#[derive(Debug, Clone)]
pub enum Brack {
    Function(String, BrackedArgs),
    Expression(BrackedArgs),
    Value(String),
}

impl TryFrom<parse::Brack> for Brack {
    type Error = String;
    fn try_from(brack: parse::Brack) -> Result<Self, Self::Error> {
        let parse::Brack(brack_variant) = brack;
        Ok(match brack_variant {
            parse::any_variants::AnyVariants3::V1((value, bracked_args)) => {
                Brack::Function(value_to_string(value), BrackedArgs::try_from(bracked_args)?)
            }
            parse::any_variants::AnyVariants3::V2(bracked_args) => {
                Brack::Expression(BrackedArgs::try_from(bracked_args)?)
            }
            parse::any_variants::AnyVariants3::V3(value) => Brack::Value(value_to_string(value)),
        })
    }
}

#[derive(Debug, Clone)]
pub struct BrackedArgs {
    brackets: Brackets,
    args: Vec<Statements>,
}

#[derive(Debug, Clone)]
pub enum Brackets {
    Parens,
    CBraces,
}

impl BrackedArgs {
    pub fn brackets(&self) -> &Brackets {
        &self.brackets
    }

    pub fn args(&self) -> &Vec<Statements> {
        &self.args
    }

    fn new(brackets: Brackets, args: Vec<Statements>) -> BrackedArgs {
        BrackedArgs { brackets, args }
    }
}

impl TryFrom<parse::BrackedArgs> for BrackedArgs {
    type Error = String;
    fn try_from(bracked_args: parse::BrackedArgs) -> Result<Self, Self::Error> {
        let parse::BrackedArgs(bracked_args_variant) = bracked_args;
        match bracked_args_variant {
            parse::any_variants::AnyVariants2::V1((_lparen, args, _rparen)) => {
                let args = expr_args_to_vec(args)?;
                Ok(BrackedArgs::new(Brackets::Parens, args))
            }
            parse::any_variants::AnyVariants2::V2((_lbrace, args, _rbrace)) => {
                let args = expr_args_to_vec(args)?;
                Ok(BrackedArgs::new(Brackets::CBraces, args))
            }
        }
    }
}

fn value_to_string(value: parse::Value) -> String {
    let parse::Value(string) = value;
    string
}

fn expr_args_to_vec(expr_args: parse::ExprArgs) -> Result<Vec<Statements>, String> {
    let parse::ExprArgs(expr_args_option) = expr_args;
    match expr_args_option {
        None => Ok(vec![]),
        Some((first, rep)) => {
            let mut args = vec![Statements::try_from(*first)?];

            for (_, expr_arg) in rep {
                args.push(Statements::try_from(*expr_arg)?);
            }

            Ok(args)
        }
    }
}

#[derive(Debug, Clone)]
pub enum SumOp {
    Plus,
    Minus,
    PlusMinus,
    MinusPlus,
}

impl TryFrom<parse::SumOp> for SumOp {
    type Error = String;
    fn try_from(sum_op: parse::SumOp) -> Result<Self, Self::Error> {
        let parse::SumOp(sum_op_variant) = sum_op;
        let op = match sum_op_variant {
            parse::any_variants::AnyVariants4::V1(_plus) => SumOp::Plus,
            parse::any_variants::AnyVariants4::V2(_minus) => SumOp::Minus,
            parse::any_variants::AnyVariants4::V3(_plusminus) => SumOp::PlusMinus,
            parse::any_variants::AnyVariants4::V4(_minusplus) => SumOp::MinusPlus,
        };
        Ok(op)
    }
}

#[derive(Debug, Clone)]
pub enum ProductOp {
    At,
    Star,
    Slash,
}

impl TryFrom<parse::ProdOp> for ProductOp {
    type Error = String;
    fn try_from(prod_op: parse::ProdOp) -> Result<Self, Self::Error> {
        let parse::ProdOp(prod_op_variant) = prod_op;
        let op = match prod_op_variant {
            parse::any_variants::AnyVariants3::V1(_at) => ProductOp::At,
            parse::any_variants::AnyVariants3::V2(_star) => ProductOp::Star,
            parse::any_variants::AnyVariants3::V3(_slash) => ProductOp::Slash,
        };
        Ok(op)
    }
}

#[derive(Debug, Clone)]
pub enum NegateOp {
    Minus,
    PlusMinus,
    MinusPlus,
}

impl TryFrom<parse::NegOp> for NegateOp {
    type Error = String;
    fn try_from(neg_op: parse::NegOp) -> Result<Self, Self::Error> {
        let parse::NegOp(neg_op_variant) = neg_op;
        let op = match neg_op_variant {
            parse::any_variants::AnyVariants3::V1(_minus) => NegateOp::Minus,
            parse::any_variants::AnyVariants3::V2(_plusminus) => NegateOp::PlusMinus,
            parse::any_variants::AnyVariants3::V3(_minusplus) => NegateOp::MinusPlus,
        };
        Ok(op)
    }
}

#[derive(Debug, Clone)]
pub enum ExponentOp {
    Caret,
}

impl TryFrom<parse::ExpOp> for ExponentOp {
    type Error = String;
    fn try_from(_exp_op: parse::ExpOp) -> Result<Self, Self::Error> {
        Ok(ExponentOp::Caret)
    }
}

#[derive(Debug, Clone)]
pub enum DerivateOp {
    Tag,
}

impl TryFrom<parse::DerOp> for DerivateOp {
    type Error = String;
    fn try_from(_der_op: parse::DerOp) -> Result<Self, Self::Error> {
        Ok(DerivateOp::Tag)
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionRelation {
    InvEquals,
    Equals,
    NotEquals,
    ApproxEquals,
    Lt,
    Gt,
    Le,
    Ge,
}

impl TryFrom<parse::ExprRel> for ExpressionRelation {
    type Error = String;
    fn try_from(expr_rel: parse::ExprRel) -> Result<Self, Self::Error> {
        let parse::ExprRel(expr_rel_variant) = expr_rel;
        let relation = match expr_rel_variant {
            parse::any_variants::AnyVariants8::V1(_inv_equals) => ExpressionRelation::InvEquals,
            parse::any_variants::AnyVariants8::V2(_equals) => ExpressionRelation::Equals,
            parse::any_variants::AnyVariants8::V3(_notequals) => ExpressionRelation::NotEquals,
            parse::any_variants::AnyVariants8::V4(_approxequals) => {
                ExpressionRelation::ApproxEquals
            }
            parse::any_variants::AnyVariants8::V5(_lt) => ExpressionRelation::Lt,
            parse::any_variants::AnyVariants8::V6(_gt) => ExpressionRelation::Gt,
            parse::any_variants::AnyVariants8::V7(_le) => ExpressionRelation::Le,
            parse::any_variants::AnyVariants8::V8(_ge) => ExpressionRelation::Ge,
        };
        Ok(relation)
    }
}

#[derive(Debug, Clone)]
pub enum StatementRelation {
    InvRArrow,
    RArrow,
}

impl TryFrom<parse::StmtRel> for StatementRelation {
    type Error = String;
    fn try_from(stmt_rel: parse::StmtRel) -> Result<Self, Self::Error> {
        let parse::StmtRel(stmt_rel_variant) = stmt_rel;
        let relation = match stmt_rel_variant {
            parse::any_variants::AnyVariants2::V1(_inv_rarrow) => StatementRelation::InvRArrow,
            parse::any_variants::AnyVariants2::V2(_rarrow) => StatementRelation::RArrow,
        };
        Ok(relation)
    }
}
