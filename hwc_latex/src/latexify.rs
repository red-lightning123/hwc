use hwc_lang_equation::format;

pub fn latexify_statements(statements : &format::Statements) -> String {
    let mut s = String::new();
    if let Some(first) = statements.first() {
        s += &latexify_statement(first);
    }
    for (statement_relation, statement) in statements.rep() {
        s += &latexify_statement_relation(statement_relation);
        s += &latexify_statement(statement);
    }
    if let Some(last) = statements.last() {
        s += &latexify_statement_relation(last);
    }
    s
}

pub fn latexify_statement(statement : &format::Statement) -> String {
    match statement {
        format::Statement::Relational(relational_statement) => latexify_relational_statement(relational_statement),
        format::Statement::Is(is_statement) => latexify_is_statement(is_statement)
    }
}

pub fn latexify_is_statement(_is_statement : &format::IsStatement) -> String {
    unimplemented!("latex converter doesn't handle is statement")
}

pub fn latexify_relational_statement(relational_statement : &format::RelationalStatement) -> String {
    let mut s = String::new();
    if let Some(first) = relational_statement.first() {
        s += &latexify_expression(first);
    }
    for (expression_relation, expression) in relational_statement.rep() {
        s += &latexify_expression_relation(expression_relation);
        s += &latexify_expression(expression);
    }
    if let Some(last) = relational_statement.last() {
        s += &latexify_expression_relation(last);
    }
    s
}

pub fn latexify_expression(expression : &format::Expression) -> String {
    latexify_sum(expression.sum())
}

pub fn latexify_sum(sum : &format::Sum) -> String {
    let mut s = String::new();
    s += &latexify_product(sum.first());
    for (sum_op, product) in sum.rep() {
        s += &latexify_sum_op(sum_op);
        s += &latexify_product(product);
    }
    s
}

pub fn latexify_product(product : &format::Product) -> String {
    let mut s = String::new();
    let mut last_negate = Some(product.first());

    for (product_op, negate) in product.rep() {
        match product_op {
            format::ProductOp::At => {
                if let Some(last_negate) = last_negate {
                    s += &latexify_negate(last_negate);
                }
                s += " ";
                last_negate = Some(negate);
            }
            format::ProductOp::Star => {
                if let Some(last_negate) = last_negate {
                    s += &latexify_negate(last_negate);
                }
                s += "\\cdot ";
                last_negate = Some(negate);
            }
            format::ProductOp::Slash => {
                s += "\\dfrac{";
                s += &latexify_negate(last_negate.expect("latexify_product found two sequential divide ops in product. Illegal"));
                s += "}{";
                s += &latexify_negate(negate);
                s += "} ";
                last_negate = None;
            }
        }
    }
    if let Some(last_negate) = last_negate {
        s += &latexify_negate(last_negate);
    }
    s
}

pub fn latexify_negate(negate : &format::Negate) -> String {
    let mut s = String::new();
    for negate_op in negate.ops() {
        s += &latexify_negate_op(negate_op);
    }
    let exponent = negate.exponent();
    s += &latexify_exponent(exponent);
    s
}

pub fn latexify_exponent(exponent : &format::Exponent) -> String {
    let mut s = String::new();
    s += "{ ";
    s += &latexify_derivate(exponent.first());
    s += "} ";
    for (exponent_op, derivate) in exponent.rep() {
        s += &latexify_exponent_op(exponent_op);
        s += "{ ";
        s += &latexify_derivate(derivate);
        s += "} ";
    }
    s
}

pub fn latexify_derivate(derivate : &format::Derivate) -> String {
    let mut s = String::new();
    s += &latexify_brack(derivate.brack());
    for derivate_op in derivate.ops() {
        s += &latexify_derivate_op_spaceless(derivate_op); // spaceless since { f ' ' } gives a double superscript error while { f '' } works
    }
    s += " ";
    s
}

pub fn latexify_brack(brack : &format::Brack) -> String {
    let mut s = String::new();
    match brack {
        format::Brack::Function(name, bracked_args) => {
            if name == "abs" {
                s += "\\left| { ";
                s += &latexify_args(bracked_args.args());
                s += "} \\right| ";
            } else if name == "sqrt" {
                s += "\\sqrt{ ";
                s += &latexify_args(bracked_args.args());
                s += "} ";
            } else if name == "cbraces" {
                s += "\\left\\{ { ";
                s += &latexify_args(bracked_args.args());
                s += "} \\right\\} ";
            } else if name == "floor" {
                s += "\\left\\lfloor { ";
                s += &latexify_args(bracked_args.args());
                s += "} \\right\\rfloor ";
            } else if name == "ceil" {
                s += "\\left\\lceil { ";
                s += &latexify_args(bracked_args.args());
                s += "} \\right\\rceil ";
            } else if name == "sumlu" {
                s += "\\sum\\limits_{ ";
                s += &latexify_statements(&bracked_args.args()[0]);
                s += "}^{ ";
                s += &latexify_statements(&bracked_args.args()[1]);
                s += "} ";
            } else if name == "prodlu" {
                s += "\\prod\\limits_{ ";
                s += &latexify_statements(&bracked_args.args()[0]);
                s += "}^{ ";
                s += &latexify_statements(&bracked_args.args()[1]);
                s += "} ";
            } else if name == "intlu" {
                s += "\\int\\limits_{ ";
                s += &latexify_statements(&bracked_args.args()[0]);
                s += "}^{ ";
                s += &latexify_statements(&bracked_args.args()[1]);
                s += "} ";
            } else if name == "lim" {
                s += "\\lim\\limits_{ ";
                s += &latexify_statements(&bracked_args.args()[0]);
                s += "} ";
            } else if name == "linex" {
                s += "\\overleftrightarrow{";
                s += &latexify_args(&bracked_args.args());
                s += "}";
            } else if name == "in" {
                s += "{{ ";
                s += &latexify_statements(&bracked_args.args()[0]);
                s += "} \\in { ";
                s += &latexify_statements(&bracked_args.args()[1]);
                s += "}} ";
            } else if name == "choose" {
                s += "{{ ";
                s += &latexify_statements(&bracked_args.args()[0]);
                s += "} \\choose { ";
                s += &latexify_statements(&bracked_args.args()[1]);
                s += "}} ";
            } else if name == "stack" {
                s += "\\substack { ";
                if let Some(first) = bracked_args.args().get(0) {
                    s += &latexify_statements(first);
                    for arg in &bracked_args.args()[1..] {
                        s += " \\\\ ";
                        s += &latexify_statements(arg);
                    }
                }
                s += "} ";
            } else if name == "ang" {
                s += "\\sangle { ";
                s += &latexify_args(bracked_args.args());
                s += "} ";
            } else if name == "cang" {
                s += "\\angle { ";
                s += &latexify_args(bracked_args.args());
                s += "} ";
            } else if name == "deg" {
                s += "\\ang { ";
                s += &latexify_args(bracked_args.args());
                s += "} ";
            } else if name == "sin" {
                s += &("{\\sin ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "cos" {
                s += &("{\\cos ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "tan" {
                s += &("{\\tan ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "sin2" {
                s += &("{\\sin^2 ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "cos2" {
                s += &("{\\cos^2 ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "tan2" {
                s += &("{\\tan^2 ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "sin3" {
                s += &("{\\sin^3 ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "cos3" {
                s += &("{\\cos^3 ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "tan3" {
                s += &("{\\tan^3 ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "sin4" {
                s += &("{\\sin^4 ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "cos4" {
                s += &("{\\cos^4 ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "tan4" {
                s += &("{\\tan^4 ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "acos" {
                s += &("{\\arccos ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "asin" {
                s += &("{\\arcsin ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "atan" {
                s += &("{\\arctan ".to_string() + &latexify_bracked_args(bracked_args) + "}");
            } else if name == "or" {
                if let Some(first) = bracked_args.args().get(0) {
                    s += "{ ";
                    s += &latexify_statements(first);
                    s += "} ";
                    for arg in &bracked_args.args()[1..] {
                        s += "\\lor { ";
                        s += &latexify_statements(arg);
                        s += "} ";
                    }
                }
            } else if name == "and" {
                if let Some(first) = bracked_args.args().get(0) {
                    s += "{ ";
                    s += &latexify_statements(first);
                    s += "} ";
                    for arg in &bracked_args.args()[1..] {
                        s += "\\land { ";
                        s += &latexify_statements(arg);
                        s += "} ";
                    }
                }
            } else if name == "gor" {
                if let Some(first) = bracked_args.args().get(0) {
                    s += "{ ";
                    s += &latexify_statements(first);
                    s += "} ";
                    for arg in &bracked_args.args()[1..] {
                        s += "\\cup { ";
                        s += &latexify_statements(arg);
                        s += "} ";
                    }
                }
            } else if name == "gand" {
                if let Some(first) = bracked_args.args().get(0) {
                    s += "{ ";
                    s += &latexify_statements(first);
                    s += "} ";
                    for arg in &bracked_args.args()[1..] {
                        s += "\\cap { ";
                        s += &latexify_statements(arg);
                        s += "} ";
                    }
                }
            } else if name == "gfrom" {
                if let Some(first) = bracked_args.args().get(0) {
                    s += "{ ";
                    s += &latexify_statements(first);
                    s += "} ";
                    for arg in &bracked_args.args()[1..] {
                        s += "| { ";
                        s += &latexify_statements(arg);
                        s += "} ";
                    }
                }
            } else if name == "gnot" {
                match &bracked_args.args()[..] {
                    [] => panic!("gnot can't have 0 args"),
                    [arg] => {
                        s += "\\neg { ";
                        s += &latexify_statements(arg);
                        s += "} ";
                    }
                    _ => panic!("gnot can't have more than 1 arg")
                }
            } else if name == "text" {
                s += "\\text{";
                // TODO: this function should do actual input validation
                if bracked_args.args().len() != 1 {
                    panic!("latexify::latexify_brack - text function should have one arg")
                }
                if let Some(text_statements) = bracked_args.args().first() {
                    if let Some(text_statement) = text_statements.first() {
                        if let hwc_lang_equation::format::Statement::Relational(text_statement) = text_statement {
                            if let Some(text_expression) = text_statement.first() {
                                if let format::Brack::Value(value) = text_expression.sum().first().first().exponent().first().brack() {
                                    s += value;
                                } else {
                                    panic!("latexify::latexify_brack - text function arg isn't a value")
                                }
                            }
                        }
                    }
                }
                s += "} ";
            } else {
                s += &latexify_string(name);
                s += &latexify_bracked_args(bracked_args);
            }
        }
        format::Brack::Expression(bracked_args) => {
            s += &latexify_bracked_args(bracked_args);
        }
        format::Brack::Value(value) => {
            if value == "%inf" {
                s += "\\infty ";
            } else if value == "%ldots" {
                s += "\\ldots ";
            } else if value == "%setN" {
                s += "\\mathbb{N} ";
            } else if value == "%setZ" {
                s += "\\mathbb{Z} ";
            } else if value == "%setR" {
                s += "\\mathbb{R} ";
            } else if value == "%in" {
                s += "\\in ";
            } else if value == "%notin" {
                s += "\\notin ";
            } else if value == "%impspace" {
                s += "\\quad ";
            } else if value == "%or" {
                s += "\\lor ";
            } else if value == "%and" {
                s += "\\land ";
            } else if value == "%gor" {
                s += "\\cup ";
            } else if value == "%gand" {
                s += "\\cap ";
            } else if value == "%gfrom" {
                s += " | ";
            } else if value == "%gnot" {
                s += " \\neg ";
            } else if value == "%rarrow" {
                s += " \\rightarrow ";
            } else if value == "%larrow" {
                s += " \\leftarrow ";
            } else if value == "%subset" {
                s += " \\subset ";
            } else if value == "%subseteq" {
                s += " \\subseteq ";
            } else if value == "%a" {
                s += " \\alpha ";
            } else if value == "%b" {
                s += " \\beta ";
            } else if value == "%c" {
                s += " \\gamma ";
            } else if value == "%t" {
                s += " \\theta ";
            } else if value == "%par" {
                s += " \\parallel ";
            } else if value == "%perp" {
                s += " \\perp ";
            } else if value == "%empty" {
                s += " \\emptyset ";
            } else if value == "%pi" {
                s += " \\pi ";
            } else if value.contains("%") {
                panic!("\"{}\" value macro isn't recognized", value)
            } else {
                s += &latexify_string(value);
            }
        }
    }
    s
}

pub fn latexify_bracked_args(bracked_args : &format::BrackedArgs) -> String {
    let mut s = String::new();
    match bracked_args.brackets() {
        format::Brackets::Parens => {
            s += "\\left(";
            s += " ";

            s += &latexify_args(bracked_args.args());

            s += "\\right)";
            s += " ";
        }
        format::Brackets::CBraces => {
            s += "{";
            s += " ";

            s += &latexify_args(bracked_args.args());

            s += "}";
            s += " ";
        }
    }
    s
}

pub fn latexify_args(args : &[format::Statements]) -> String {
    let mut s = String::new();
    if let Some(first) = args.first() {
        s += &latexify_statements(first);
        for expression in &args[1..] {
            s += ", ";
            s += &latexify_statements(expression);
        }
    }
    s
}

pub fn latexify_sum_op(sum_op : &format::SumOp) -> String {
    let op =
        match sum_op {
            format::SumOp::Plus => "+",
            format::SumOp::Minus => "-",
            format::SumOp::PlusMinus => "\\pm",
            format::SumOp::MinusPlus => "\\mp"
        };
    format!("{} ", op)
}

// latexify_product_op doesn't exist because product ops need special handling of precedence

pub fn latexify_negate_op(negate_op : &format::NegateOp) -> String {
    let op =
        match negate_op {
            //format::NegateOp::Plus => "+",
            format::NegateOp::Minus => "-",
            format::NegateOp::PlusMinus => "\\pm",
            format::NegateOp::MinusPlus => "\\mp"
        };
    format!("{} ", op)
}

pub fn latexify_exponent_op(exponent_op : &format::ExponentOp) -> String {
    let op =
        match exponent_op {
            format::ExponentOp::Caret => "^"
        };
    format!("{} ", op)
}

pub fn latexify_derivate_op_spaceless(derivate_op : &format::DerivateOp) -> String {
    let op =
        match derivate_op {
            format::DerivateOp::Tag => "'"
        };
    format!("{}", op)
}

pub fn latexify_expression_relation(expression_relation : &format::ExpressionRelation) -> String {
    let relation =
        match expression_relation {
            format::ExpressionRelation::InvEquals => "",
            format::ExpressionRelation::Equals => "=",
            format::ExpressionRelation::NotEquals => "\\neq",
            format::ExpressionRelation::ApproxEquals => "\\approx",
            format::ExpressionRelation::Lt => "<",
            format::ExpressionRelation::Gt => ">",
            format::ExpressionRelation::Le => "\\leq",
            format::ExpressionRelation::Ge => "\\geq"
        };
    format!("{} ", relation)
}

pub fn latexify_statement_relation(statement_relation : &format::StatementRelation) -> String {
    let relation =
        match statement_relation {
            format::StatementRelation::InvRArrow => "",
            format::StatementRelation::RArrow => "\\drarrow"
        };
    format!("{} ", relation)
}

pub fn latexify_string(string : &str) -> String {
    format!("{} ", string)
}
