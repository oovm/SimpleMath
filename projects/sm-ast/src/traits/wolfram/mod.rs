use crate::{ast::Symbol, AST};
use text_utils::capitalize_first_letter;
use wolfram_wxf::{ToWolfram, WolframValue};

impl ToWolfram for AST {
    fn to_wolfram(&self) -> WolframValue {
        match (*self).clone() {
            AST::EmptyStatement | AST::NewLine => WolframValue::Skip,
            //
            AST::Program(s) => {
                let v: Vec<_> = s.iter().map(|s| s.to_wolfram()).collect();
                WolframValue::Function(Box::from("CompoundExpression"), v)
            }
            AST::Expression { base, .. } => base.to_wolfram(),
            //
            AST::Function(s, p) => {
                if p.len() > 1 {
                    unimplemented!()
                }
                let arguments = &p[0].arguments;
                let options = &p[0].options;
                let mut vec = vec![];
                for arg in arguments {
                    vec.push(arg.to_wolfram())
                }
                for (k, v) in options {
                    vec.push(WolframValue::Function(Box::from("Rule"), vec![k.to_wolfram(), v.to_wolfram()]))
                }
                WolframValue::Function(Box::from(function_map(&s)), vec)
            }
            //
            AST::Null => WolframValue::new_symbol("None"),
            AST::Boolean(b) => {
                if b {
                    WolframValue::new_symbol("True")
                }
                else {
                    WolframValue::new_symbol("False")
                }
            }
            AST::Integer(i) => WolframValue::BigInteger(i),
            AST::Decimal(f) => WolframValue::BigDecimal(Box::from(format!("{}", f))),
            AST::Symbol(s) => WolframValue::Symbol(Box::from(s.name)),
            AST::String(s) => WolframValue::String(Box::from(s)),
        }
    }
}

pub fn function_map(s: &Symbol) -> String {
    let name = s.name.as_str();
    match name {
        "factor" => "FactorInteger".to_string(),
        _ => capitalize_first_letter(name),
    }
}
