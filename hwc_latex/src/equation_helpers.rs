pub fn statements_as_function(statements : &hwc_lang_equation::format::Statements) -> Option<(&String, &Vec<hwc_lang_equation::format::Statements>)> {
    if let Some(expression) = statements_as_expression(statements) {
        expression_as_function(expression)
    } else {
        None
    }
}

pub fn statements_as_value(statements : &hwc_lang_equation::format::Statements) -> Option<&String> {
    if let Some(expression) = statements_as_expression(statements) {
        expression_as_value(expression)
    } else {
        None
    }
}

pub fn expression_as_value(expression : &hwc_lang_equation::format::Expression) -> Option<&String> {
    if let Some(brack) = expression_as_brack(expression) {
        if let hwc_lang_equation::format::Brack::Value(string) = brack {
            return Some(string);
        }
    }
    None
}

pub fn expression_as_function(expression : &hwc_lang_equation::format::Expression) -> Option<(&String, &Vec<hwc_lang_equation::format::Statements>)> {
    if let Some(brack) = expression_as_brack(expression) {
        if let hwc_lang_equation::format::Brack::Function(name, args) = brack {
            if let hwc_lang_equation::format::Brackets::CBraces = args.brackets() {
                return Some((&name, &args.args()));
            }
        }
    }
    None
}

pub fn expression_as_brack(expression : &hwc_lang_equation::format::Expression) -> Option<&hwc_lang_equation::format::Brack> {
    let sum = expression.sum();
    if sum.rep().is_empty() {
        let product = sum.first();
        if product.rep().is_empty() {
            let negate = product.first();
            if negate.ops().is_empty() {
                let exponent = negate.exponent();
                if exponent.rep().is_empty() {
                    let derivate = exponent.first();
                    if derivate.ops().is_empty() {
                        return Some(derivate.brack());
                    }
                }
            }
        }
    }
    None
}

pub fn statements_as_expression(statements : &hwc_lang_equation::format::Statements) -> Option<&hwc_lang_equation::format::Expression> {
    if statements.rep().is_empty() && statements.last().is_none() {
        if let Some(hwc_lang_equation::format::Statement::Relational(relational_statement)) = statements.first() {
            if relational_statement.rep().is_empty() && relational_statement.last().is_none() {
                return relational_statement.first().as_ref();
            }
        }
    }
    None
}
