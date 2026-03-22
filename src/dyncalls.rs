use dyncall::{ArgType, ArgVal, FuncDef, ScriptVal, StructValue};

use crate::{Interpreter, Token, Value};

/// Holds a heap-allocated numeric output buffer for a Pointer(T) dyncall argument.
/// Boxing ensures the address stays stable while the invocation stores raw pointers to it.
enum OutNum {
    Char(Box<u8>),
    I16(Box<i16>),
    U16(Box<u16>),
    I32(Box<i32>),
    U32(Box<u32>),
    I64(Box<i64>),
    U64(Box<u64>),
    F32(Box<f32>),
    F64(Box<f64>),
}

impl OutNum {
    fn to_f64(&self) -> f64 {
        match self {
            OutNum::Char(v) => **v as f64,
            OutNum::I16(v) => **v as f64,
            OutNum::U16(v) => **v as f64,
            OutNum::I32(v) => **v as f64,
            OutNum::U32(v) => **v as f64,
            OutNum::I64(v) => **v as f64,
            OutNum::U64(v) => **v as f64,
            OutNum::F32(v) => **v as f64,
            OutNum::F64(v) => **v,
        }
    }
}

/// Holds a struct argument that was built from a BASIC array.
/// `writeback_array` is set for `*{...}` (pointer-to-struct) args so that mutated
/// fields can be written back after the call.
struct StructSlot {
    sv: StructValue,
    writeback_array: Option<String>,
}

fn result_to_tokens(ret: &ArgVal, result: &mut Vec<Token>) {
    match ret {
        ArgVal::Char(n) => result.push(Token::Number(n.to_string())),
        ArgVal::I16(n) => result.push(Token::Number(n.to_string())),
        ArgVal::U16(n) => result.push(Token::Number(n.to_string())),
        ArgVal::I32(n) => result.push(Token::Number(n.to_string())),
        ArgVal::U32(n) => result.push(Token::Number(n.to_string())),
        ArgVal::I64(n) => result.push(Token::Number(n.to_string())),
        ArgVal::U64(n) => result.push(Token::Number(n.to_string())),
        ArgVal::F32(n) => result.push(Token::Number(n.to_string())),
        ArgVal::F64(n) => result.push(Token::Number(n.to_string())),
        ArgVal::RustString(s) => {
            let string_val = unsafe { (*(*s)).clone() };
            result.push(Token::StringLiteral(string_val));
        }
        ArgVal::Pointer(p) => result.push(Token::Number((*p as i64).to_string())),
        // Struct returns become an opaque Token::Struct; LET stores it as a
        // struct-backed array so that field reads (e.g. lc(0)) work naturally.
        ArgVal::StructValue(sv) => result.push(Token::Struct(sv.clone())),
        _ => result.push(Token::Number("0".to_string())),
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
        let arg_count = arg_values.len();

        // Pre-allocate buffers for OCString args so they have stable addresses
        // during the call (the invocation stores raw pointers into them).
        let mut oc_strings: Vec<Option<String>> = (0..arg_count).map(|_| None).collect();
        for (i, v) in arg_values.iter().enumerate() {
            if i < fdef.get_arg_count() && matches!(fdef.get_arg_type(i), ArgType::OCString(_)) {
                oc_strings[i] = Some(match v {
                    Value::String(s) => s.clone(),
                    _ => String::new(),
                });
            }
        }

        // Pre-allocate boxed buffers for Pointer(numeric) output args.
        let mut out_nums: Vec<Option<OutNum>> = (0..arg_count).map(|_| None).collect();
        for (i, v) in arg_values.iter().enumerate() {
            if i < fdef.get_arg_count() {
                if let ArgType::Pointer(inner) = fdef.get_arg_type(i) {
                    let init = match v {
                        Value::Number(n) => *n,
                        _ => 0.0,
                    };
                    out_nums[i] = Some(match inner.as_ref() {
                        ArgType::Char => OutNum::Char(Box::new(init as u8)),
                        ArgType::I16 => OutNum::I16(Box::new(init as i16)),
                        ArgType::U16 => OutNum::U16(Box::new(init as u16)),
                        ArgType::I32 => OutNum::I32(Box::new(init as i32)),
                        ArgType::U32 => OutNum::U32(Box::new(init as u32)),
                        ArgType::I64 => OutNum::I64(Box::new(init as i64)),
                        ArgType::U64 => OutNum::U64(Box::new(init as u64)),
                        ArgType::F32 => OutNum::F32(Box::new(init as f32)),
                        ArgType::F64 => OutNum::F64(Box::new(init)),
                        _ => continue,
                    });
                }
            }
        }

        // Build StructSlots for struct/pointer-to-struct arguments from BASIC arrays.
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
            let script_vals: Vec<dyncall::ScriptVal> = array.data.iter().map(|v| match v {
                Value::Number(n) => dyncall::ScriptVal::Number(*n),
                Value::String(s) => dyncall::ScriptVal::Str(s.clone()),
                Value::Struct(_) => dyncall::ScriptVal::Number(0.0),
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
            if let Some(ref mut out_num) = out_nums[i] {
                let push_result = match out_num {
                    OutNum::Char(v) => invoke.push_mut_arg(v.as_mut()),
                    OutNum::I16(v) => invoke.push_mut_arg(v.as_mut()),
                    OutNum::U16(v) => invoke.push_mut_arg(v.as_mut()),
                    OutNum::I32(v) => invoke.push_mut_arg(v.as_mut()),
                    OutNum::U32(v) => invoke.push_mut_arg(v.as_mut()),
                    OutNum::I64(v) => invoke.push_mut_arg(v.as_mut()),
                    OutNum::U64(v) => invoke.push_mut_arg(v.as_mut()),
                    OutNum::F32(v) => invoke.push_mut_arg(v.as_mut()),
                    OutNum::F64(v) => invoke.push_mut_arg(v.as_mut()),
                };
                if let Err(e) = push_result {
                    eprintln!("Error pushing output argument {}: {}", i, e);
                    result.push(Token::Number("0".to_string()));
                    return;
                }
                continue;
            }
            let push_result = match arg_value {
                Value::Number(num) => {
                    let ptr_slot = i < fdef.get_arg_count()
                        && matches!(fdef.get_arg_type(i), ArgType::OpaquePointer);
                    if ptr_slot {
                        let p = num as i64 as *mut std::ffi::c_void;
                        invoke.push_arg(&ArgVal::Pointer(p))
                    } else {
                        invoke.push_arg(&(num as i64))
                    }
                }
                Value::String(s) => {
                    if let Some(ref mut oc_str) = oc_strings[i] {
                        invoke.push_mut_arg(oc_str)
                    } else {
                        invoke.push_arg(&s)
                    }
                }
                Value::Struct(_) => continue, // pushed via struct_slots above
            };
            if let Err(e) = push_result {
                eprintln!("Error pushing argument {}: {}", i, e);
                result.push(Token::Number("0".to_string()));
                return;
            }
        }

        let ret = match invoke.call() {
            Ok(val) => val,
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

        result_to_tokens(&ret, result);

        // Write back updated OCString buffers to the corresponding BASIC variables
        for (i, oc_str_opt) in oc_strings.into_iter().enumerate() {
            if let Some(updated) = oc_str_opt {
                if i < raw_args.len() && raw_args[i].len() == 1 {
                    if let Token::Identifier(var_name) = &raw_args[i][0] {
                        self.variables
                            .insert(var_name.clone(), Value::String(updated));
                    }
                }
            }
        }

        // Write back updated numeric output buffers to the corresponding BASIC variables
        for (i, out_num_opt) in out_nums.into_iter().enumerate() {
            if let Some(out_num) = out_num_opt {
                if i < raw_args.len() && raw_args[i].len() == 1 {
                    if let Token::Identifier(var_name) = &raw_args[i][0] {
                        self.variables
                            .insert(var_name.clone(), Value::Number(out_num.to_f64()));
                    }
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
                    Err(_) => break,
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
        let arr = make_array_with_vals(&[0.0, 0.0]);
        let mut sv = StructValue::from_script_vals(&arg_type, &[ScriptVal::Number(0.0), ScriptVal::Number(0.0)]).unwrap();
        sv.reset();
        sv.push_field(&7i32).unwrap();
        sv.push_field(&3.5f64).unwrap();

        let mut arr2 = arr;
        for fi in 0..sv.field_count() {
            arr2.data[fi] = match sv.script_read(fi).unwrap() {
                ScriptVal::Number(n) => Value::Number(n),
                ScriptVal::Str(s) => Value::String(s),
            };
        }

        assert!(matches!(&arr2.data[0], Value::Number(n) if *n == 7.0));
        assert!(matches!(&arr2.data[1], Value::Number(n) if *n == 3.5));
    }
}
