use dyncall::{ArgType, FuncDef, ScriptResult, ScriptVal, StructValue};

use crate::{Interpreter, Token, Value};

/// Holds a struct argument that was built from a BASIC array.
/// `writeback_array` is set for `*{...}` (pointer-to-struct) args so that mutated
/// fields can be written back after the call.
struct StructSlot {
    sv: StructValue,
    writeback_array: Option<String>,
}

fn script_val_to_token(val: ScriptVal) -> Token {
    match val {
        ScriptVal::Integer(n) => Token::Number(n.to_string()),
        ScriptVal::Number(f) => Token::Number(f.to_string()),
        ScriptVal::Str(s)    => Token::StringLiteral(s),
        ScriptVal::Pointer(p) => Token::Number((p as i64).to_string()),
        ScriptVal::Nil       => Token::Number("0".to_string()),
    }
}

impl Interpreter {
    pub(crate) fn handle_external_call(
        &mut self,
        fdef: &FuncDef,
        raw_args: &[Vec<Token>],
        arg_values: Vec<Value>,
        result: &mut Vec<Token>,
    ) {
        let mut invoke = fdef.prep();

        // Build StructSlots for struct/pointer-to-struct arguments from BASIC arrays.
        let arg_count = arg_values.len();
        let mut struct_slots: Vec<Option<StructSlot>> = (0..arg_count).map(|_| None).collect();
        for (i, slot) in struct_slots.iter_mut().enumerate() {
            if i >= fdef.get_arg_count() {
                continue;
            }
            let arg_type = fdef.get_arg_type(i);
            let is_ptr = match arg_type {
                ArgType::Struct(_) => false,
                ArgType::Pointer(inner) if matches!(inner.as_ref(), ArgType::Struct(_)) => true,
                _ => continue,
            };

            let Some(raw_arg) = raw_args.get(i) else {
                eprintln!("Error: missing argument tokens for struct argument {}", i);
                result.push(Token::Number("0".to_string()));
                return;
            };
            let [Token::Identifier(array_name)] = raw_arg.as_slice() else {
                eprintln!(
                    "Error: struct argument {} must be a BASIC array variable name",
                    i
                );
                result.push(Token::Number("0".to_string()));
                return;
            };
            let Some(array) = self.arrays.get(array_name.as_str()) else {
                eprintln!(
                    "Error: BASIC array '{}' not found for struct argument {}",
                    array_name, i
                );
                result.push(Token::Number("0".to_string()));
                return;
            };
            let script_vals: Vec<ScriptVal> = array.data.iter().map(|v| match v {
                Value::Number(n) => ScriptVal::Number(*n),
                Value::String(s) => ScriptVal::Str(s.clone()),
                Value::Struct(_) => ScriptVal::Number(0.0),
            }).collect();
            let sv = match StructValue::from_script_vals(arg_type, &script_vals) {
                Ok(sv) => sv,
                Err(e) => {
                    eprintln!("Error building struct argument {}: {}", i, e);
                    result.push(Token::Number("0".to_string()));
                    return;
                }
            };
            *slot = Some(StructSlot {
                sv,
                writeback_array: if is_ptr { Some(array_name.to_string()) } else { None },
            });
        }

        for (i, arg_value) in arg_values.into_iter().enumerate() {
            if i >= fdef.get_arg_count() {
                break;
            }
            if let Some(ref mut slot) = struct_slots[i] {
                let push_result = if slot.writeback_array.is_some() {
                    invoke.push_mut_arg(&mut slot.sv)
                } else {
                    invoke.push_arg(&slot.sv)
                };
                if let Err(e) = push_result {
                    eprintln!("Error pushing struct argument {}: {}", i, e);
                    result.push(Token::Number("0".to_string()));
                    return;
                }
                continue;
            }
            let sv = match arg_value {
                Value::Number(n) => ScriptVal::Number(n),
                Value::String(s) => ScriptVal::Str(s),
                Value::Struct(_) => continue, // pushed via struct_slots above
            };
            if let Err(e) = invoke.push_script_val(sv) {
                eprintln!("Error pushing argument {}: {}", i, e);
                result.push(Token::Number("0".to_string()));
                return;
            }
        }

        let script_result: ScriptResult = match invoke.call_scripted() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error calling external function: {}", e);
                result.push(Token::Number("0".to_string()));
                return;
            }
        };
        if let Some(errno_val) = invoke.last_errno() {
            self.variables
                .insert("ERRNO".to_string(), Value::Number(errno_val as f64));
        }
        result.push(script_val_to_token(script_result.return_val));

        // Write back output-pointer and OCString values to BASIC variables.
        for (arg_idx, out_val) in script_result.outputs {
            if arg_idx < raw_args.len() && raw_args[arg_idx].len() == 1 {
                if let Token::Identifier(var_name) = &raw_args[arg_idx][0] {
                    let basic_val = match out_val {
                        ScriptVal::Str(s) => Value::String(s),
                        ScriptVal::Integer(n) => Value::Number(n as f64),
                        ScriptVal::Number(f) => Value::Number(f),
                        ScriptVal::Pointer(p) => Value::Number(p as i64 as f64),
                        ScriptVal::Nil => Value::Number(0.0),
                    };
                    self.variables.insert(var_name.clone(), basic_val);
                }
            }
        }

        // Write back mutated *{...} struct fields to their source BASIC arrays
        for slot in struct_slots.into_iter().flatten() {
            let Some(array_name) = slot.writeback_array else {
                continue;
            };
            let Some(array) = self.arrays.get_mut(&array_name) else {
                eprintln!(
                    "Error: BASIC array '{}' disappeared before struct writeback",
                    array_name
                );
                continue;
            };
            for fi in 0..slot.sv.field_count() {
                if fi >= array.data.len() {
                    break;
                }
                array.data[fi] = match slot.sv.script_read(fi) {
                    Ok(ScriptVal::Number(n)) => Value::Number(n),
                    Ok(ScriptVal::Str(s)) => Value::String(s),
                    Ok(ScriptVal::Integer(n)) => Value::Number(n as f64),
                    _ => break,
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Array, Value};
    use dyncall::{ArgType, StructType};

    fn make_array_with_vals(vals: &[f64]) -> Array {
        let mut arr = Array::new(vec![0], vec![vals.len().saturating_sub(1)]);
        for (i, v) in vals.iter().enumerate() {
            arr.data[i] = Value::Number(*v);
        }
        arr
    }

    #[test]
    fn builds_struct_from_script_vals() {
        let struct_type = StructType::new(vec![ArgType::U32, ArgType::U32]).unwrap();
        let arg_type = ArgType::Struct(struct_type);
        let arr = make_array_with_vals(&[10.0, 32.0]);

        let vals: Vec<ScriptVal> = arr.data.iter().map(|v| match v {
            Value::Number(n) => ScriptVal::Number(*n),
            _ => ScriptVal::Number(0.0),
        }).collect();
        let sv = StructValue::from_script_vals(&arg_type, &vals).unwrap();

        assert_eq!(sv.read_field::<u32>(0).unwrap(), 10);
        assert_eq!(sv.read_field::<u32>(1).unwrap(), 32);
    }

    #[test]
    fn writes_mutated_struct_back_via_script_read() {
        let struct_type = StructType::new(vec![ArgType::I32, ArgType::F64]).unwrap();
        let arg_type = ArgType::Pointer(Box::new(ArgType::Struct(struct_type)));
        let mut sv = StructValue::from_script_vals(&arg_type, &[ScriptVal::Number(0.0), ScriptVal::Number(0.0)]).unwrap();
        let arr = make_array_with_vals(&[0.0, 0.0]);
        sv.reset();
        sv.push_field(&7i32).unwrap();
        sv.push_field(&3.5f64).unwrap();

        let mut arr2 = arr;
        for fi in 0..sv.field_count() {
            arr2.data[fi] = match sv.script_read(fi).unwrap() {
                ScriptVal::Number(n) => Value::Number(n),
                ScriptVal::Str(s) => Value::String(s),
                ScriptVal::Integer(n) => Value::Number(n as f64),
                _ => break,
            };
        }

        assert!(matches!(&arr2.data[0], Value::Number(n) if *n == 7.0));
        assert!(matches!(&arr2.data[1], Value::Number(n) if *n == 3.5));
    }
}
