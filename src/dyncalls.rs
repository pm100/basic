use dyncall::{ArgVal, FuncDef};

use crate::{Interpreter, Token, Value};

impl Interpreter {
    pub(crate) fn handle_external_call(
        &self,
        fdef: &FuncDef,
        arg_values: Vec<Value>,
        result: &mut Vec<Token>,
    ) {
        let mut invoke = fdef.prep();
        for arg_value in arg_values {
            match arg_value {
                Value::Number(num) => invoke.push_arg(&(num as i64)),
                Value::String(str) => invoke.push_arg(&str),
            }
        }
        let ret = invoke.call();
        match ret {
            ArgVal::I32(n) => {
                result.push(Token::Number(n.to_string()));
            }
            ArgVal::RustString(s) => {
                let string_val = unsafe { (*s).clone() };
                result.push(Token::StringLiteral(string_val));
            }
            _ => {
                result.push(Token::Number("0".to_string()));
            }
        }
        println!("External FN  returned {:?}", ret);
    }
}
