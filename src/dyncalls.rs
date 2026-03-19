use dyncall::{ArgType, ArgVal, FuncDef, StructValue};

use crate::{Array, Interpreter, Token, Value};

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

enum StructArg {
    ByValue(StructValue),
    ByPointer { array_name: String, value: StructValue },
}

fn struct_array_name(raw_arg: &[Token]) -> Option<&str> {
    match raw_arg {
        [Token::Identifier(name)] => Some(name.as_str()),
        _ => None,
    }
}

fn push_number_field(struct_value: &mut StructValue, field_type: &ArgType, number: f64) -> Result<(), String> {
    let result = match field_type {
        ArgType::Char => struct_value.push_field(&(number as u8)),
        ArgType::I16 => struct_value.push_field(&(number as i16)),
        ArgType::U16 => struct_value.push_field(&(number as u16)),
        ArgType::I32 => struct_value.push_field(&(number as i32)),
        ArgType::U32 => struct_value.push_field(&(number as u32)),
        ArgType::I64 => struct_value.push_field(&(number as i64)),
        ArgType::U64 => struct_value.push_field(&(number as u64)),
        ArgType::F32 => struct_value.push_field(&(number as f32)),
        ArgType::F64 => struct_value.push_field(&number),
        _ => {
            return Err(format!(
                "unsupported BASIC struct field type: {:?}",
                field_type
            ))
        }
    };
    result.map_err(|err| err.to_string())
}

fn build_struct_value(arg_type: &ArgType, array: &Array) -> Result<StructValue, String> {
    let mut struct_value = StructValue::new(arg_type).map_err(|err| err.to_string())?;
    let field_count = struct_value.field_count();
    if array.data.len() < field_count {
        return Err(format!(
            "array has {} element(s), but struct argument requires {} field(s)",
            array.data.len(),
            field_count
        ));
    }

    for index in 0..field_count {
        let field_type = struct_value
            .struct_type()
            .field_type(index)
            .cloned()
            .ok_or_else(|| format!("struct field {} is out of range", index))?;
        let number = match &array.data[index] {
            Value::Number(number) => *number,
            Value::String(_) => {
                return Err(format!(
                    "struct field {} must come from a numeric BASIC array element",
                    index
                ))
            }
        };
        push_number_field(&mut struct_value, &field_type, number)?;
    }

    Ok(struct_value)
}

fn read_struct_field_as_number(struct_value: &StructValue, index: usize) -> Result<f64, String> {
    let field_type = struct_value
        .struct_type()
        .field_type(index)
        .ok_or_else(|| format!("struct field {} is out of range", index))?;

    let result = match field_type {
        ArgType::Char => struct_value
            .read_field::<u8>(index)
            .map(|value| value as f64),
        ArgType::I16 => struct_value
            .read_field::<i16>(index)
            .map(|value| value as f64),
        ArgType::U16 => struct_value
            .read_field::<u16>(index)
            .map(|value| value as f64),
        ArgType::I32 => struct_value
            .read_field::<i32>(index)
            .map(|value| value as f64),
        ArgType::U32 => struct_value
            .read_field::<u32>(index)
            .map(|value| value as f64),
        ArgType::I64 => struct_value
            .read_field::<i64>(index)
            .map(|value| value as f64),
        ArgType::U64 => struct_value
            .read_field::<u64>(index)
            .map(|value| value as f64),
        ArgType::F32 => struct_value
            .read_field::<f32>(index)
            .map(|value| value as f64),
        ArgType::F64 => struct_value.read_field::<f64>(index),
        _ => {
            return Err(format!(
                "unsupported BASIC struct field type: {:?}",
                field_type
            ))
        }
    };
    result.map_err(|err| err.to_string())
}

fn write_struct_back_to_array(array: &mut Array, struct_value: &StructValue) -> Result<(), String> {
    let field_count = struct_value.field_count();
    if array.data.len() < field_count {
        return Err(format!(
            "array has {} element(s), but mutated struct contains {} field(s)",
            array.data.len(),
            field_count
        ));
    }

    for index in 0..field_count {
        let value = read_struct_field_as_number(struct_value, index)?;
        array.data[index] = Value::Number(value);
    }

    Ok(())
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
        // Boxing gives each buffer a stable heap address for the duration of the call.
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

        let mut struct_args: Vec<Option<StructArg>> = (0..arg_count).map(|_| None).collect();
        for i in 0..arg_count {
            if i >= fdef.get_arg_count() {
                continue;
            }

            let arg_type = fdef.get_arg_type(i);
            let is_struct_arg = matches!(arg_type, ArgType::Struct(_))
                || matches!(arg_type, ArgType::Pointer(inner) if matches!(inner.as_ref(), ArgType::Struct(_)));
            if !is_struct_arg {
                continue;
            }

            let Some(raw_arg) = raw_args.get(i) else {
                eprintln!("Error: missing BASIC argument tokens for struct argument {}", i);
                result.push(Token::Number("0".to_string()));
                return;
            };
            let Some(array_name) = struct_array_name(raw_arg) else {
                eprintln!(
                    "Error: struct argument {} must be passed as a BASIC array variable name",
                    i
                );
                result.push(Token::Number("0".to_string()));
                return;
            };
            let Some(array) = self.arrays.get(array_name) else {
                eprintln!(
                    "Error: BASIC array '{}' was not found for struct argument {}",
                    array_name, i
                );
                result.push(Token::Number("0".to_string()));
                return;
            };
            let struct_value = match build_struct_value(arg_type, array) {
                Ok(struct_value) => struct_value,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    result.push(Token::Number("0".to_string()));
                    return;
                }
            };

            struct_args[i] = Some(match arg_type {
                ArgType::Struct(_) => StructArg::ByValue(struct_value),
                ArgType::Pointer(inner) if matches!(inner.as_ref(), ArgType::Struct(_)) => {
                    StructArg::ByPointer {
                        array_name: array_name.to_string(),
                        value: struct_value,
                    }
                }
                _ => unreachable!("checked above"),
            });
        }

        // Push args in order, using push_mut_arg for OCString and Pointer(numeric) slots
        for (i, arg_value) in arg_values.into_iter().enumerate() {
            if let Some(struct_arg) = struct_args.get_mut(i).and_then(Option::as_mut) {
                let push_result = match struct_arg {
                    StructArg::ByValue(value) => invoke.push_arg(value),
                    StructArg::ByPointer { value, .. } => invoke.push_mut_arg(value),
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
            };
            if let Err(e) = push_result {
                eprintln!("Error pushing argument {}: {}", i, e);
                result.push(Token::Number("0".to_string()));
                return;
            }
        }

        let ret = invoke.call();
        // If the errno flag was set, store the captured value in the BASIC
        // variable ERRNO so scripts can inspect it after the call.
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

        for struct_arg in struct_args.into_iter().flatten() {
            if let StructArg::ByPointer { array_name, value } = struct_arg {
                let Some(array) = self.arrays.get_mut(&array_name) else {
                    eprintln!(
                        "Error: BASIC array '{}' disappeared before struct writeback",
                        array_name
                    );
                    continue;
                };

                if let Err(err) = write_struct_back_to_array(array, &value) {
                    eprintln!("Error: {}", err);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use dyncall::StructType;

    use super::{build_struct_value, write_struct_back_to_array};
    use crate::{Array, Value};
    use dyncall::ArgType;

    #[test]
    fn builds_struct_value_from_basic_array() {
        let struct_type = StructType::new(vec![ArgType::U32, ArgType::U32]).unwrap();
        let arg_type = ArgType::Struct(struct_type);
        let mut array = Array::new(vec![0], vec![1]);
        array.data[0] = Value::Number(10.0);
        array.data[1] = Value::Number(32.0);

        let struct_value = build_struct_value(&arg_type, &array).unwrap();

        assert_eq!(struct_value.read_field::<u32>(0).unwrap(), 10);
        assert_eq!(struct_value.read_field::<u32>(1).unwrap(), 32);
    }

    #[test]
    fn writes_mutated_struct_back_into_basic_array() {
        let struct_type = StructType::new(vec![ArgType::I32, ArgType::F64]).unwrap();
        let arg_type = ArgType::Pointer(Box::new(ArgType::Struct(struct_type)));
        let mut array = Array::new(vec![1], vec![2]);
        let mut struct_value = build_struct_value(&arg_type, &array).unwrap();

        struct_value.reset();
        struct_value.push_field(&7i32).unwrap();
        struct_value.push_field(&3.5f64).unwrap();

        write_struct_back_to_array(&mut array, &struct_value).unwrap();

        match &array.data[0] {
            Value::Number(value) => assert_eq!(*value, 7.0),
            Value::String(_) => panic!("expected numeric field"),
        }
        match &array.data[1] {
            Value::Number(value) => assert_eq!(*value, 3.5),
            Value::String(_) => panic!("expected numeric field"),
        }
    }
}
