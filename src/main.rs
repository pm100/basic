use crossterm::event::{poll, read, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use dyncall::{DynCaller, FuncDef};
use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::time::Duration;
mod dyncalls;

fn init_logger() {
    let env = env_logger::Env::default().default_filter_or("warn");
    let mut builder = env_logger::Builder::from_env(env);
    builder.format_timestamp(None);
    let _ = builder.try_init();
}

fn main() {
    init_logger();

    // Set up panic hook to ensure raw mode is disabled on panic
    std::panic::set_hook(Box::new(|panic_info| {
        let _ = disable_raw_mode();
        eprintln!("{}", panic_info);
    }));

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // File mode
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(code) => {
                let mut interpreter = Interpreter::new();

                // Parse the entire file and store lines
                for line in code.lines() {
                    let tokens = tokenize(line);

                    if let Some(Token::Number(line_num)) = tokens.first() {
                        let line_num: i32 = line_num.parse().unwrap_or(0);
                        let stmt_tokens = tokens[1..].to_vec();
                        let statements = parse(&stmt_tokens);

                        if let Some(stmt) = statements.first() {
                            interpreter.store_line(line_num, stmt);
                        }
                    }
                }

                // Enable raw mode for INKEY$ and other terminal operations
                let _ = enable_raw_mode();

                // Small delay to let any pending input settle
                std::thread::sleep(Duration::from_millis(50));

                // Flush any buffered input before starting
                while poll(Duration::from_millis(0)).unwrap_or(false) {
                    let _ = read();
                }

                // Do one final consuming read to ensure buffer is completely clear
                if poll(Duration::from_millis(0)).unwrap_or(false) {
                    let _ = read();
                }

                // Run the stored program
                interpreter.run();

                // Disable raw mode when done
                let _ = disable_raw_mode();
            }
            Err(e) => {
                eprintln!("Error reading file '{}': {}", filename, e);
                std::process::exit(1);
            }
        }
    } else {
        // Interactive mode
        let mut interactive = Interactive::new();
        interactive.run();
    }
}

#[derive(Debug, Clone)]
enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    StringLiteral(String),
    Operator(char),
    Comparison(String), // <, >, <=, >=, =, <>
    Equal,
    LeftParen,
    RightParen,
    Newline,
    To,
    Step,
    Function(String), // Built-in functions
    Comma,
    Semicolon,
    And,
    Or,
    Not,
    Mod,
    Colon,
    Struct(dyncall::StructValue), // opaque struct return value from a C call
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Keyword(a), Token::Keyword(b)) => a == b,
            (Token::Identifier(a), Token::Identifier(b)) => a == b,
            (Token::Number(a), Token::Number(b)) => a == b,
            (Token::StringLiteral(a), Token::StringLiteral(b)) => a == b,
            (Token::Operator(a), Token::Operator(b)) => a == b,
            (Token::Comparison(a), Token::Comparison(b)) => a == b,
            (Token::Equal, Token::Equal) => true,
            (Token::LeftParen, Token::LeftParen) => true,
            (Token::RightParen, Token::RightParen) => true,
            (Token::Newline, Token::Newline) => true,
            (Token::To, Token::To) => true,
            (Token::Step, Token::Step) => true,
            (Token::Function(a), Token::Function(b)) => a == b,
            (Token::Comma, Token::Comma) => true,
            (Token::Semicolon, Token::Semicolon) => true,
            (Token::And, Token::And) => true,
            (Token::Or, Token::Or) => true,
            (Token::Not, Token::Not) => true,
            (Token::Mod, Token::Mod) => true,
            (Token::Colon, Token::Colon) => true,
            (Token::Struct(_), Token::Struct(_)) => false, // opaque; not comparable
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
enum PrintItem {
    Expr(Vec<Token>),
    Comma,     // Tab separator
    Semicolon, // No space separator
}

#[derive(Debug, Clone)]
enum Statement {
    Compound(Vec<Statement>), // Multiple statements separated by :
    Let {
        var: String,
        expr: Vec<Token>,
    },
    LetArray {
        var: String,
        subscripts: Vec<Token>,
        expr: Vec<Token>,
    },
    Print {
        items: Vec<PrintItem>,
        no_newline: bool,
    },
    Input {
        prompt: Option<String>,
        vars: Vec<String>,
    },
    Goto {
        line: i32,
    },
    Gosub {
        line: i32,
    },
    Return,
    If {
        condition: Vec<Token>,
        then_stmt: Option<Box<Statement>>,
        else_stmt: Option<Box<Statement>>,
        then_line: Option<i32>,
    },
    Swap {
        var1: String,
        var2: String,
    },
    For {
        var: String,
        start: Vec<Token>,
        end: Vec<Token>,
        step: Option<Vec<Token>>,
    },
    Next {
        var: String,
    },
    While {
        condition: Vec<Token>,
    },
    Wend,
    End,
    Data {
        values: Vec<Token>,
    },
    Read {
        vars: Vec<String>,
    },
    Restore,
    Cls,
    Locate {
        row: Vec<Token>,
        col: Vec<Token>,
    },
    Dim {
        var: String,
        dims: Vec<usize>,
    },
    OptionBase {
        base: usize,
    },
    MidAssign {
        var: String,
        start_expr: Vec<Token>,
        length_expr: Vec<Token>,
        value_expr: Vec<Token>,
    },
    Randomize {
        seed: Option<Vec<Token>>,
    },
    OnGoto {
        expr: Vec<Token>,
        lines: Vec<i32>,
    },
    OnGosub {
        expr: Vec<Token>,
        lines: Vec<i32>,
    },
    DefFn {
        name: String,
        param: String,
        expr: Vec<Token>,
    },
    DefXfn {
        name: String,
        defstr: String,
    },
    ExitFor,
    ExitWhile,
    SelectCase {
        expr: Vec<Token>,
    },
    Case {
        values: Vec<Vec<Token>>,
    },
    CaseElse,
    EndSelect,
    LineInput {
        prompt: Option<String>,
        var: String,
    },
    LineInputFile {
        file_num: i32,
        var: String,
    },
    Open {
        filename: Vec<Token>,
        mode: String,
        file_num: i32,
    },
    Close {
        file_num: i32,
    },
    PrintFile {
        file_num: i32,
        items: Vec<PrintItem>,
    },
    InputFile {
        file_num: i32,
        vars: Vec<String>,
    },
    Write {
        items: Vec<Vec<Token>>,
    },
    WriteFile {
        file_num: i32,
        items: Vec<Vec<Token>>,
    },
    OnError {
        line: i32,
    },
    Resume {
        next: bool,
    },
    // Standalone expression statement (e.g. `fn mci(...)` for side effects)
    Expr {
        expr: Vec<Token>,
    },
}

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    String(String),
    Struct(dyncall::StructValue), // opaque struct returned from a C call
}

#[derive(Debug, Clone)]
struct ForLoop {
    var: String,
    end_value: f64,
    step: f64,
    return_pc: usize,
}

#[derive(Debug, Clone)]
struct WhileLoop {
    condition: Vec<Token>,
    start_pc: usize,
}

#[derive(Debug, Clone)]
struct Array {
    lower_bounds: Vec<usize>, // Lower bound for each dimension
    upper_bounds: Vec<usize>, // Upper bound for each dimension
    data: Vec<Value>,         // Flattened data (unused when struct_val is Some)
    struct_val: Option<dyncall::StructValue>, // set when this array backs a C struct return
}

impl Array {
    fn new(lower_bounds: Vec<usize>, upper_bounds: Vec<usize>) -> Self {
        let total_size: usize = lower_bounds
            .iter()
            .zip(upper_bounds.iter())
            .map(|(l, u)| u - l + 1)
            .product();
        Array {
            lower_bounds,
            upper_bounds,
            data: vec![Value::Number(0.0); total_size],
            struct_val: None,
        }
    }

    /// Create an array backed by a C struct return value.
    /// Field access is forwarded to [`dyncall::StructValue::script_read`].
    fn from_struct_val(sv: dyncall::StructValue) -> Self {
        let n = sv.field_count().saturating_sub(1);
        Array {
            lower_bounds: vec![0],
            upper_bounds: vec![n],
            data: vec![],
            struct_val: Some(sv),
        }
    }

    fn get(&self, subscripts: &[usize]) -> Option<&Value> {
        if subscripts.len() != self.lower_bounds.len() {
            return None;
        }

        // Check bounds
        for (i, &sub) in subscripts.iter().enumerate() {
            if sub < self.lower_bounds[i] || sub > self.upper_bounds[i] {
                return None;
            }
        }

        // Calculate flattened index (normalize to 0-based)
        let mut index = 0;
        let mut multiplier = 1;
        for i in (0..subscripts.len()).rev() {
            let normalized = subscripts[i] - self.lower_bounds[i];
            index += normalized * multiplier;
            multiplier *= self.upper_bounds[i] - self.lower_bounds[i] + 1;
        }

        self.data.get(index)
    }

    fn set(&mut self, subscripts: &[usize], value: Value) -> bool {
        if subscripts.len() != self.lower_bounds.len() {
            return false;
        }

        // Check bounds
        for (i, &sub) in subscripts.iter().enumerate() {
            if sub < self.lower_bounds[i] || sub > self.upper_bounds[i] {
                return false;
            }
        }

        // Calculate flattened index (normalize to 0-based)
        let mut index = 0;
        let mut multiplier = 1;
        for i in (0..subscripts.len()).rev() {
            let normalized = subscripts[i] - self.lower_bounds[i];
            index += normalized * multiplier;
            multiplier *= self.upper_bounds[i] - self.lower_bounds[i] + 1;
        }

        if index < self.data.len() {
            self.data[index] = value;
            true
        } else {
            false
        }
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    // Check for REM comment - ignore rest of line
    let trimmed = input.trim();
    if trimmed.starts_with("REM ") || trimmed.starts_with("REM\t") || trimmed == "REM" {
        return vec![];
    }
    // Also support ' for comments (common in later BASIC)
    if trimmed.starts_with('\'') {
        return vec![];
    }

    let keywords = [
        "PRINT",
        "LET",
        "IF",
        "THEN",
        "ELSE",
        "GOTO",
        "GOSUB",
        "RETURN",
        "FOR",
        "NEXT",
        "INPUT",
        "END",
        "TO",
        "STEP",
        "WHILE",
        "WEND",
        "DATA",
        "READ",
        "RESTORE",
        "CLS",
        "DIM",
        "OPTION",
        "BASE",
        "RANDOMIZE",
        "AND",
        "OR",
        "NOT",
        "MOD",
        "ON",
        "SWAP",
        "DEF",
        "FN",
        "EXIT",
        "SELECT",
        "CASE",
        "LINE",
        "OPEN",
        "CLOSE",
        "AS",
        "ERROR",
        "RESUME",
        "WRITE",
        "LOCATE",
        "XFN",
    ];
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut string_content = String::new();

    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if in_string {
            if c == '"' {
                tokens.push(Token::StringLiteral(string_content.clone()));
                string_content.clear();
                in_string = false;
            } else {
                string_content.push(c);
            }
        } else if c == '"' {
            if !current.is_empty() {
                push_token(&mut tokens, &current, &keywords);
                current.clear();
            }
            in_string = true;
        } else if c == '\n' {
            if !current.is_empty() {
                push_token(&mut tokens, &current, &keywords);
                current.clear();
            }
            tokens.push(Token::Newline);
        } else if c.is_whitespace() {
            if !current.is_empty() {
                if current == "REM" {
                    return tokens; // rest of line is a comment
                }
                push_token(&mut tokens, &current, &keywords);
                current.clear();
            }
        } else if c.is_alphabetic() || c == '$' {
            current.push(c.to_ascii_uppercase());
        } else if c.is_numeric() || c == '.' {
            current.push(c);
        } else if c == '<' {
            if !current.is_empty() {
                push_token(&mut tokens, &current, &keywords);
                current.clear();
            }
            if chars.peek() == Some(&'=') {
                chars.next();
                tokens.push(Token::Comparison("<=".to_string()));
            } else if chars.peek() == Some(&'>') {
                chars.next();
                tokens.push(Token::Comparison("<>".to_string()));
            } else {
                tokens.push(Token::Comparison("<".to_string()));
            }
        } else if c == '>' {
            if !current.is_empty() {
                push_token(&mut tokens, &current, &keywords);
                current.clear();
            }
            if chars.peek() == Some(&'=') {
                chars.next();
                tokens.push(Token::Comparison(">=".to_string()));
            } else {
                tokens.push(Token::Comparison(">".to_string()));
            }
        } else if "+-*/^#".contains(c) {
            if !current.is_empty() {
                push_token(&mut tokens, &current, &keywords);
                current.clear();
            }
            tokens.push(Token::Operator(c));
        } else if c == '=' {
            if !current.is_empty() {
                push_token(&mut tokens, &current, &keywords);
                current.clear();
            }
            tokens.push(Token::Equal);
        } else if c == '(' {
            if !current.is_empty() {
                push_token(&mut tokens, &current, &keywords);
                current.clear();
            }
            tokens.push(Token::LeftParen);
        } else if c == ')' {
            if !current.is_empty() {
                push_token(&mut tokens, &current, &keywords);
                current.clear();
            }
            tokens.push(Token::RightParen);
        } else if c == ',' {
            if !current.is_empty() {
                push_token(&mut tokens, &current, &keywords);
                current.clear();
            }
            tokens.push(Token::Comma);
        } else if c == ';' {
            if !current.is_empty() {
                push_token(&mut tokens, &current, &keywords);
                current.clear();
            }
            tokens.push(Token::Semicolon);
        } else if c == ':' {
            if !current.is_empty() {
                push_token(&mut tokens, &current, &keywords);
                current.clear();
            }
            tokens.push(Token::Colon);
        } else if c == '\'' {
            // Comment - ignore rest of line
            break;
        }
    }

    if !current.is_empty() {
        push_token(&mut tokens, &current, &keywords);
    }

    tokens
}

fn push_token(tokens: &mut Vec<Token>, current: &str, keywords: &[&str]) {
    let functions = [
        "ABS", "ATN", "COS", "EXP", "INT", "LOG", "RND", "SIN", "SQR", "TAN", "VAL", "TAB", "SPC",
        "EOF", "SGN", "FIX", "CINT",
    ];
    let string_functions = [
        "LEN", "LEFT$", "RIGHT$", "MID$", "UCASE$", "LCASE$", "INSTR$", "CHR$", "ASC", "STR$",
        "SPACE$", "STRING$", "LTRIM$", "RTRIM$", "TRIM$", "HEX$", "OCT$", "INKEY$",
    ];

    if keywords.contains(&current) {
        match current {
            "TO" => tokens.push(Token::To),
            "STEP" => tokens.push(Token::Step),
            "AND" => tokens.push(Token::And),
            "OR" => tokens.push(Token::Or),
            "NOT" => tokens.push(Token::Not),
            "MOD" => tokens.push(Token::Mod),
            _ => tokens.push(Token::Keyword(current.to_string())),
        }
    } else if functions.contains(&current) || string_functions.contains(&current) {
        tokens.push(Token::Function(current.to_string()));
    } else if current.chars().all(|ch| ch.is_numeric() || ch == '.')
        && current.contains(|ch: char| ch.is_numeric())
    {
        tokens.push(Token::Number(current.to_string()));
    } else {
        tokens.push(Token::Identifier(current.to_string()));
    }
}

fn parse(tokens: &[Token]) -> Vec<Statement> {
    // First split by colons to handle multiple statements per line
    let mut statement_groups = Vec::new();
    let mut current_group = Vec::new();

    for token in tokens {
        if token == &Token::Colon {
            if !current_group.is_empty() {
                statement_groups.push(current_group.clone());
                current_group.clear();
            }
        } else {
            current_group.push(token.clone());
        }
    }
    if !current_group.is_empty() {
        statement_groups.push(current_group);
    }

    // Parse each statement group
    let mut all_statements = Vec::new();
    for group in statement_groups {
        all_statements.extend(parse_single_statement(&group));
    }

    // If there's more than one statement, wrap in a Compound statement
    if all_statements.len() > 1 {
        vec![Statement::Compound(all_statements)]
    } else {
        all_statements
    }
}

fn parse_single_statement(tokens: &[Token]) -> Vec<Statement> {
    let mut statements = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        // Check for MID$ assignment: MID$(var, start, length) = value
        if let Token::Function(func) = &tokens[i] {
            if func == "MID$" && tokens.get(i + 1) == Some(&Token::LeftParen) {
                // Find the closing paren
                let mut j = i + 2;
                let mut depth = 1;
                while j < tokens.len() && depth > 0 {
                    if tokens[j] == Token::LeftParen {
                        depth += 1;
                    } else if tokens[j] == Token::RightParen {
                        depth -= 1;
                    }
                    j += 1;
                }

                // Check if followed by =
                if j < tokens.len() && tokens[j] == Token::Equal {
                    // Parse arguments inside MID$(...)
                    let args_tokens = &tokens[i + 2..j - 1];

                    // Split by commas to get var, start, length
                    let mut parts = Vec::new();
                    let mut current_part = Vec::new();
                    let mut paren_depth = 0;

                    for token in args_tokens {
                        match token {
                            Token::LeftParen => {
                                paren_depth += 1;
                                current_part.push(token.clone());
                            }
                            Token::RightParen => {
                                paren_depth -= 1;
                                current_part.push(token.clone());
                            }
                            Token::Comma if paren_depth == 0 => {
                                if !current_part.is_empty() {
                                    parts.push(current_part.clone());
                                    current_part.clear();
                                }
                            }
                            _ => {
                                current_part.push(token.clone());
                            }
                        }
                    }
                    if !current_part.is_empty() {
                        parts.push(current_part);
                    }

                    if parts.len() == 3 {
                        // parts[0] = variable name, parts[1] = start, parts[2] = length
                        let var = if let Some(Token::Identifier(v)) = parts[0].first() {
                            v.clone()
                        } else {
                            String::new()
                        };

                        let start_expr = parts[1].clone();
                        let length_expr = parts[2].clone();

                        // Get the value expression after =
                        let mut value_expr = Vec::new();
                        let mut k = j + 1;
                        while k < tokens.len() && tokens[k] != Token::Newline {
                            value_expr.push(tokens[k].clone());
                            k += 1;
                        }

                        statements.push(Statement::MidAssign {
                            var,
                            start_expr,
                            length_expr,
                            value_expr,
                        });
                        i = k;
                        continue;
                    }
                }
            }
        }

        if let Token::Keyword(kw) = &tokens[i] {
            match kw.as_str() {
                "LET" => {
                    if let Some(Token::Identifier(var)) = tokens.get(i + 1) {
                        // Check if it's an array assignment: LET A(5) = expr
                        // Array if followed by left paren and not a function name
                        let is_function = [
                            "ABS", "ATN", "COS", "EXP", "INT", "LOG", "RND", "SIN", "SQR", "TAN",
                            "VAL", "TAB",
                        ]
                        .contains(&var.as_str());
                        if !is_function && tokens.get(i + 2) == Some(&Token::LeftParen) {
                            // Find the closing paren for subscripts
                            let mut j = i + 3;
                            let mut depth = 1;
                            while j < tokens.len() && depth > 0 {
                                if tokens[j] == Token::LeftParen {
                                    depth += 1;
                                } else if tokens[j] == Token::RightParen {
                                    depth -= 1;
                                }
                                j += 1;
                            }

                            // j now points past the closing paren
                            if j < tokens.len() && tokens[j] == Token::Equal {
                                let subscripts = tokens[i + 3..j - 1].to_vec();
                                let mut expr = Vec::new();
                                let mut k = j + 1;
                                while k < tokens.len() && tokens[k] != Token::Newline {
                                    expr.push(tokens[k].clone());
                                    k += 1;
                                }
                                statements.push(Statement::LetArray {
                                    var: var.clone(),
                                    subscripts,
                                    expr,
                                });
                                i = k;
                            }
                        } else if tokens.get(i + 2) == Some(&Token::Equal) {
                            // Regular variable assignment
                            let mut expr = Vec::new();
                            let mut j = i + 3;
                            while j < tokens.len() && tokens[j] != Token::Newline {
                                expr.push(tokens[j].clone());
                                j += 1;
                            }
                            statements.push(Statement::Let {
                                var: var.clone(),
                                expr,
                            });
                            i = j;
                        }
                    }
                }
                "WRITE" => {
                    // Check for WRITE # (file output)
                    if i + 1 < tokens.len() && tokens[i + 1] == Token::Operator('#') {
                        let mut j = i + 2;
                        if let Some(Token::Number(file_num_str)) = tokens.get(j) {
                            let file_num = file_num_str.parse::<i32>().unwrap_or(0);
                            j += 1;

                            // Skip comma after file number
                            if j < tokens.len() && tokens[j] == Token::Comma {
                                j += 1;
                            }

                            // Parse items separated by commas
                            let mut items = Vec::new();
                            let mut current_expr = Vec::new();
                            let mut paren_depth = 0;

                            while j < tokens.len() && tokens[j] != Token::Newline {
                                match &tokens[j] {
                                    Token::LeftParen => {
                                        paren_depth += 1;
                                        current_expr.push(tokens[j].clone());
                                    }
                                    Token::RightParen => {
                                        paren_depth -= 1;
                                        current_expr.push(tokens[j].clone());
                                    }
                                    Token::Comma if paren_depth == 0 => {
                                        if !current_expr.is_empty() {
                                            items.push(current_expr.clone());
                                            current_expr.clear();
                                        }
                                    }
                                    _ => {
                                        current_expr.push(tokens[j].clone());
                                    }
                                }
                                j += 1;
                            }

                            if !current_expr.is_empty() {
                                items.push(current_expr);
                            }

                            statements.push(Statement::WriteFile { file_num, items });
                            i = j;
                        }
                    } else {
                        // Regular WRITE to console
                        let mut j = i + 1;
                        let mut items = Vec::new();
                        let mut current_expr = Vec::new();
                        let mut paren_depth = 0;

                        while j < tokens.len() && tokens[j] != Token::Newline {
                            match &tokens[j] {
                                Token::LeftParen => {
                                    paren_depth += 1;
                                    current_expr.push(tokens[j].clone());
                                }
                                Token::RightParen => {
                                    paren_depth -= 1;
                                    current_expr.push(tokens[j].clone());
                                }
                                Token::Comma if paren_depth == 0 => {
                                    if !current_expr.is_empty() {
                                        items.push(current_expr.clone());
                                        current_expr.clear();
                                    }
                                }
                                _ => {
                                    current_expr.push(tokens[j].clone());
                                }
                            }
                            j += 1;
                        }

                        if !current_expr.is_empty() {
                            items.push(current_expr);
                        }

                        statements.push(Statement::Write { items });
                        i = j;
                    }
                }
                "PRINT" => {
                    // Check for PRINT # (file output)
                    if i + 1 < tokens.len() && tokens[i + 1] == Token::Operator('#') {
                        let mut j = i + 2;
                        if let Some(Token::Number(file_num_str)) = tokens.get(j) {
                            let file_num = file_num_str.parse::<i32>().unwrap_or(0);
                            j += 1;

                            // Skip comma or semicolon after file number
                            if j < tokens.len()
                                && (tokens[j] == Token::Comma || tokens[j] == Token::Semicolon)
                            {
                                j += 1;
                            }

                            // Parse print items
                            let mut items = Vec::new();
                            let mut current_expr = Vec::new();
                            let mut paren_depth = 0;

                            while j < tokens.len() && tokens[j] != Token::Newline {
                                match &tokens[j] {
                                    Token::LeftParen => {
                                        paren_depth += 1;
                                        current_expr.push(tokens[j].clone());
                                    }
                                    Token::RightParen => {
                                        paren_depth -= 1;
                                        current_expr.push(tokens[j].clone());
                                    }
                                    Token::Comma if paren_depth == 0 => {
                                        if !current_expr.is_empty() {
                                            items.push(PrintItem::Expr(current_expr.clone()));
                                            current_expr.clear();
                                        }
                                        items.push(PrintItem::Comma);
                                    }
                                    Token::Semicolon if paren_depth == 0 => {
                                        if !current_expr.is_empty() {
                                            items.push(PrintItem::Expr(current_expr.clone()));
                                            current_expr.clear();
                                        }
                                        items.push(PrintItem::Semicolon);
                                    }
                                    _ => {
                                        current_expr.push(tokens[j].clone());
                                    }
                                }
                                j += 1;
                            }

                            if !current_expr.is_empty() {
                                items.push(PrintItem::Expr(current_expr));
                            }

                            statements.push(Statement::PrintFile { file_num, items });
                            i = j;
                        }
                    } else {
                        // Regular PRINT to screen
                        let mut items = Vec::new();
                        let mut current_expr = Vec::new();
                        let mut j = i + 1;
                        let mut no_newline = false;
                        let mut paren_depth = 0;

                        while j < tokens.len() && tokens[j] != Token::Newline {
                            match &tokens[j] {
                                Token::LeftParen => {
                                    paren_depth += 1;
                                    current_expr.push(tokens[j].clone());
                                }
                                Token::RightParen => {
                                    paren_depth -= 1;
                                    current_expr.push(tokens[j].clone());
                                }
                                Token::Comma if paren_depth == 0 => {
                                    if !current_expr.is_empty() {
                                        items.push(PrintItem::Expr(current_expr.clone()));
                                        current_expr.clear();
                                    }
                                    items.push(PrintItem::Comma);
                                }
                                Token::Semicolon if paren_depth == 0 => {
                                    if !current_expr.is_empty() {
                                        items.push(PrintItem::Expr(current_expr.clone()));
                                        current_expr.clear();
                                    }
                                    items.push(PrintItem::Semicolon);
                                }
                                _ => {
                                    current_expr.push(tokens[j].clone());
                                }
                            }
                            j += 1;
                        }

                        // Add any remaining expression
                        if !current_expr.is_empty() {
                            items.push(PrintItem::Expr(current_expr));
                        }

                        // Check if last item is a separator (means no newline)
                        if let Some(last) = items.last() {
                            if matches!(last, PrintItem::Comma | PrintItem::Semicolon) {
                                no_newline = true;
                            }
                        }

                        statements.push(Statement::Print { items, no_newline });
                        i = j;
                    }
                }
                "INPUT" => {
                    // Check for INPUT # (file input)
                    if i + 1 < tokens.len() && tokens[i + 1] == Token::Operator('#') {
                        let mut j = i + 2;
                        if let Some(Token::Number(file_num_str)) = tokens.get(j) {
                            let file_num = file_num_str.parse::<i32>().unwrap_or(0);
                            j += 1;

                            // Skip comma or semicolon after file number
                            if j < tokens.len()
                                && (tokens[j] == Token::Comma || tokens[j] == Token::Semicolon)
                            {
                                j += 1;
                            }

                            // Get variable list
                            let mut vars = Vec::new();
                            while j < tokens.len() && tokens[j] != Token::Newline {
                                if let Token::Identifier(var) = &tokens[j] {
                                    vars.push(var.clone());
                                }
                                j += 1;
                            }

                            if !vars.is_empty() {
                                statements.push(Statement::InputFile { file_num, vars });
                            }
                            i = j;
                        }
                    } else {
                        // Regular INPUT from keyboard
                        let mut j = i + 1;
                        let mut prompt = None;

                        // Check for prompt string
                        if let Some(Token::StringLiteral(s)) = tokens.get(j) {
                            prompt = Some(s.clone());
                            j += 1;
                            // Skip semicolon or comma after prompt
                            if matches!(tokens.get(j), Some(Token::Semicolon) | Some(Token::Comma))
                            {
                                j += 1;
                            }
                        }

                        // Collect variable names
                        let mut vars = Vec::new();
                        while j < tokens.len() && tokens[j] != Token::Newline {
                            if let Token::Identifier(var) = &tokens[j] {
                                vars.push(var.clone());
                            }
                            j += 1;
                        }

                        if !vars.is_empty() {
                            statements.push(Statement::Input { prompt, vars });
                        }
                        i = j;
                    }
                }
                "SWAP" => {
                    if let (
                        Some(Token::Identifier(var1)),
                        Some(Token::Comma),
                        Some(Token::Identifier(var2)),
                    ) = (tokens.get(i + 1), tokens.get(i + 2), tokens.get(i + 3))
                    {
                        statements.push(Statement::Swap {
                            var1: var1.clone(),
                            var2: var2.clone(),
                        });
                        i += 4;
                    }
                }
                "LINE" => {
                    // LINE INPUT ["prompt";] var  OR  LINE INPUT #n, var
                    if i + 1 < tokens.len() {
                        if let Token::Keyword(kw) = &tokens[i + 1] {
                            if kw == "INPUT" {
                                let mut j = i + 2;

                                // Check for # (file input)
                                if j < tokens.len() && tokens[j] == Token::Operator('#') {
                                    j += 1;
                                    if let Some(Token::Number(file_num_str)) = tokens.get(j) {
                                        let file_num = file_num_str.parse::<i32>().unwrap_or(0);
                                        j += 1;

                                        // Skip comma after file number
                                        if j < tokens.len() && tokens[j] == Token::Comma {
                                            j += 1;
                                        }

                                        // Get variable name
                                        if let Some(Token::Identifier(var)) = tokens.get(j) {
                                            statements.push(Statement::LineInputFile {
                                                file_num,
                                                var: var.clone(),
                                            });
                                            i = j + 1;
                                        }
                                    }
                                } else {
                                    // Regular LINE INPUT from keyboard
                                    let mut prompt = None;

                                    // Check for optional prompt
                                    if j < tokens.len() {
                                        if let Token::StringLiteral(s) = &tokens[j] {
                                            prompt = Some(s.clone());
                                            j += 1;
                                            // Skip semicolon after prompt
                                            if j < tokens.len() && tokens[j] == Token::Semicolon {
                                                j += 1;
                                            }
                                        }
                                    }

                                    // Get variable name
                                    if j < tokens.len() {
                                        if let Token::Identifier(var) = &tokens[j] {
                                            statements.push(Statement::LineInput {
                                                prompt,
                                                var: var.clone(),
                                            });
                                            i = j + 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                "OPEN" => {
                    // OPEN filename FOR mode AS #n
                    let mut j = i + 1;
                    let mut filename = Vec::new();

                    // Get filename expression until FOR
                    while j < tokens.len() {
                        if let Token::Keyword(kw) = &tokens[j] {
                            if kw == "FOR" {
                                break;
                            }
                        }
                        filename.push(tokens[j].clone());
                        j += 1;
                    }

                    if j < tokens.len() {
                        j += 1; // Skip FOR
                        let mode = if j < tokens.len() {
                            if let Token::Keyword(m) = &tokens[j] {
                                j += 1;
                                m.clone()
                            } else if let Token::Identifier(m) = &tokens[j] {
                                j += 1;
                                m.to_uppercase()
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        };

                        // Skip AS
                        if j < tokens.len() {
                            if let Token::Keyword(kw) = &tokens[j] {
                                if kw == "AS" {
                                    j += 1;
                                }
                            }
                        }

                        // Get file number (skip # if present)
                        if j < tokens.len() && tokens[j] == Token::Operator('#') {
                            j += 1;
                        }

                        if j < tokens.len() {
                            if let Token::Number(n_str) = &tokens[j] {
                                let file_num = n_str.parse::<i32>().unwrap_or(0);
                                statements.push(Statement::Open {
                                    filename,
                                    mode,
                                    file_num,
                                });
                                i = j + 1;
                            }
                        }
                    }
                }
                "CLOSE" => {
                    // CLOSE #n
                    let mut j = i + 1;
                    if j < tokens.len() && tokens[j] == Token::Operator('#') {
                        j += 1;
                    }
                    if j < tokens.len() {
                        if let Token::Number(n_str) = &tokens[j] {
                            let file_num = n_str.parse::<i32>().unwrap_or(0);
                            statements.push(Statement::Close { file_num });
                            i = j + 1;
                        }
                    }
                }
                "ON" => {
                    // Check for ON ERROR GOTO
                    if let Some(Token::Keyword(kw)) = tokens.get(i + 1) {
                        if kw == "ERROR" {
                            // ON ERROR GOTO line
                            if let Some(Token::Keyword(goto_kw)) = tokens.get(i + 2) {
                                if goto_kw == "GOTO" {
                                    if let Some(Token::Number(line)) = tokens.get(i + 3) {
                                        let line_num = line.parse().unwrap_or(0);
                                        statements.push(Statement::OnError { line: line_num });
                                        i += 4;
                                        continue;
                                    }
                                }
                            }
                        }
                    }

                    // ON expr GOTO/GOSUB line1, line2, line3...
                    let mut j = i + 1;
                    let mut expr = Vec::new();

                    // Get expression until GOTO or GOSUB
                    while j < tokens.len() {
                        if let Token::Keyword(kw) = &tokens[j] {
                            if kw == "GOTO" || kw == "GOSUB" {
                                break;
                            }
                        }
                        expr.push(tokens[j].clone());
                        j += 1;
                    }

                    if j < tokens.len() {
                        if let Token::Keyword(kw) = &tokens[j] {
                            let is_goto = kw == "GOTO";
                            j += 1; // Skip GOTO/GOSUB

                            // Collect line numbers separated by commas
                            let mut lines = Vec::new();
                            while j < tokens.len() && tokens[j] != Token::Newline {
                                if let Token::Number(line) = &tokens[j] {
                                    lines.push(line.parse().unwrap_or(0));
                                }
                                j += 1;
                            }

                            if is_goto {
                                statements.push(Statement::OnGoto { expr, lines });
                            } else {
                                statements.push(Statement::OnGosub { expr, lines });
                            }
                            i = j;
                        }
                    }
                }
                "IF" => {
                    // Find THEN keyword
                    let mut j = i + 1;
                    let mut condition = Vec::new();
                    while j < tokens.len() {
                        if let Token::Keyword(kw) = &tokens[j] {
                            if kw == "THEN" {
                                break;
                            }
                        }
                        condition.push(tokens[j].clone());
                        j += 1;
                    }

                    j += 1; // Skip THEN

                    // Check if it's a line number or inline statement
                    let then_line;
                    let mut then_stmt = None;
                    let mut else_stmt = None;

                    if let Some(Token::Number(line)) = tokens.get(j) {
                        // Traditional IF...THEN line_number
                        then_line = Some(line.parse().unwrap_or(0));
                        i = j + 1;
                    } else {
                        // Inline statement after THEN
                        then_line = None;
                        let mut then_tokens = Vec::new();

                        // Collect tokens until ELSE or newline
                        while j < tokens.len() && tokens[j] != Token::Newline {
                            if let Token::Keyword(kw) = &tokens[j] {
                                if kw == "ELSE" {
                                    break;
                                }
                            }
                            then_tokens.push(tokens[j].clone());
                            j += 1;
                        }

                        // Parse the THEN statement
                        if !then_tokens.is_empty() {
                            then_tokens.push(Token::Newline);
                            let then_stmts = parse(&then_tokens);
                            if let Some(stmt) = then_stmts.first() {
                                then_stmt = Some(Box::new(stmt.clone()));
                            }
                        }

                        // Check for ELSE
                        if j < tokens.len() {
                            if let Token::Keyword(kw) = &tokens[j] {
                                if kw == "ELSE" {
                                    j += 1; // Skip ELSE
                                    let mut else_tokens = Vec::new();

                                    // Collect tokens until newline
                                    while j < tokens.len() && tokens[j] != Token::Newline {
                                        else_tokens.push(tokens[j].clone());
                                        j += 1;
                                    }

                                    // Parse the ELSE statement
                                    if !else_tokens.is_empty() {
                                        else_tokens.push(Token::Newline);
                                        let else_stmts = parse(&else_tokens);
                                        if let Some(stmt) = else_stmts.first() {
                                            else_stmt = Some(Box::new(stmt.clone()));
                                        }
                                    }
                                }
                            }
                        }

                        i = j;
                    }

                    statements.push(Statement::If {
                        condition,
                        then_stmt,
                        else_stmt,
                        then_line,
                    });
                }
                "FOR" => {
                    if let Some(Token::Identifier(var)) = tokens.get(i + 1) {
                        if let Some(Token::Equal) = tokens.get(i + 2) {
                            let mut j = i + 3;
                            let mut start_expr = Vec::new();

                            // Get start expression (until TO)
                            while j < tokens.len() {
                                if tokens[j] == Token::To {
                                    break;
                                }
                                start_expr.push(tokens[j].clone());
                                j += 1;
                            }

                            j += 1; // Skip TO
                            let mut end_expr = Vec::new();

                            // Get end expression (until STEP or end of line)
                            while j < tokens.len() {
                                if tokens[j] == Token::Step || tokens[j] == Token::Newline {
                                    break;
                                }
                                end_expr.push(tokens[j].clone());
                                j += 1;
                            }

                            let mut step_expr = None;
                            if j < tokens.len() && tokens[j] == Token::Step {
                                j += 1;
                                let mut step = Vec::new();
                                while j < tokens.len() && tokens[j] != Token::Newline {
                                    step.push(tokens[j].clone());
                                    j += 1;
                                }
                                step_expr = Some(step);
                            }

                            statements.push(Statement::For {
                                var: var.clone(),
                                start: start_expr,
                                end: end_expr,
                                step: step_expr,
                            });
                            i = j;
                        }
                    }
                }
                "NEXT" => {
                    let var = if let Some(Token::Identifier(v)) = tokens.get(i + 1) {
                        v.clone()
                    } else {
                        // If no variable specified, use empty string (we'll handle this in interpreter)
                        String::new()
                    };
                    statements.push(Statement::Next { var });
                    i += if tokens.get(i + 1).is_some()
                        && matches!(tokens.get(i + 1), Some(Token::Identifier(_)))
                    {
                        2
                    } else {
                        1
                    };
                }
                "WHILE" => {
                    let mut j = i + 1;
                    let mut condition = Vec::new();
                    while j < tokens.len() && tokens[j] != Token::Newline {
                        condition.push(tokens[j].clone());
                        j += 1;
                    }
                    statements.push(Statement::While { condition });
                    i = j;
                }
                "WEND" => {
                    statements.push(Statement::Wend);
                    i += 1;
                }
                "DEF" => {
                    // DEF FN name(param) = expression
                    if let Some(Token::Keyword(fn_kw)) = tokens.get(i + 1) {
                        if fn_kw == "FN" {
                            if let Some(Token::Identifier(name)) = tokens.get(i + 2) {
                                // Expect (param)
                                if matches!(tokens.get(i + 3), Some(Token::LeftParen)) {
                                    if let Some(Token::Identifier(param)) = tokens.get(i + 4) {
                                        if matches!(tokens.get(i + 5), Some(Token::RightParen)) {
                                            // Expect =
                                            if matches!(tokens.get(i + 6), Some(Token::Equal)) {
                                                // Collect expression
                                                let mut j = i + 7;
                                                let mut expr = Vec::new();
                                                while j < tokens.len()
                                                    && tokens[j] != Token::Newline
                                                {
                                                    expr.push(tokens[j].clone());
                                                    j += 1;
                                                }
                                                statements.push(Statement::DefFn {
                                                    name: name.clone(),
                                                    param: param.clone(),
                                                    expr,
                                                });
                                                i = j;
                                            }
                                        }
                                    }
                                }
                            }
                        } else if fn_kw == "XFN" {
                            if let Some(Token::Identifier(name)) = tokens.get(i + 2) {
                                // Expect (param)
                                if matches!(tokens.get(i + 3), Some(Token::LeftParen)) {
                                    if let Some(Token::StringLiteral(defstr)) = tokens.get(i + 4) {
                                        if matches!(tokens.get(i + 5), Some(Token::RightParen)) {
                                            statements.push(Statement::DefXfn {
                                                name: name.clone(),
                                                defstr: defstr.clone(),
                                            });
                                            i += 6;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                "EXIT" => {
                    // EXIT FOR or EXIT WHILE
                    if let Some(Token::Keyword(kw)) = tokens.get(i + 1) {
                        match kw.as_str() {
                            "FOR" => {
                                statements.push(Statement::ExitFor);
                                i += 2;
                            }
                            "WHILE" => {
                                statements.push(Statement::ExitWhile);
                                i += 2;
                            }
                            _ => i += 1,
                        }
                    } else {
                        i += 1;
                    }
                }
                "SELECT" => {
                    // SELECT CASE expr
                    if let Some(Token::Keyword(kw)) = tokens.get(i + 1) {
                        if kw == "CASE" {
                            let mut j = i + 2;
                            let mut expr = Vec::new();
                            while j < tokens.len() && tokens[j] != Token::Newline {
                                expr.push(tokens[j].clone());
                                j += 1;
                            }
                            statements.push(Statement::SelectCase { expr });
                            i = j;
                        }
                    }
                }
                "CASE" => {
                    // CASE value1, value2, value3 or CASE ELSE
                    if let Some(Token::Keyword(kw)) = tokens.get(i + 1) {
                        if kw == "ELSE" {
                            statements.push(Statement::CaseElse);
                            i += 2;
                            continue;
                        }
                    }

                    // Parse comma-separated case values
                    let mut j = i + 1;
                    let mut values = Vec::new();
                    let mut current_value = Vec::new();

                    while j < tokens.len() && tokens[j] != Token::Newline {
                        if tokens[j] == Token::Comma {
                            if !current_value.is_empty() {
                                values.push(current_value.clone());
                                current_value.clear();
                            }
                        } else {
                            current_value.push(tokens[j].clone());
                        }
                        j += 1;
                    }

                    if !current_value.is_empty() {
                        values.push(current_value);
                    }

                    statements.push(Statement::Case { values });
                    i = j;
                }
                "GOTO" => {
                    if let Some(Token::Number(line)) = tokens.get(i + 1) {
                        statements.push(Statement::Goto {
                            line: line.parse().unwrap_or(0),
                        });
                        i += 2;
                    }
                }
                "GOSUB" => {
                    if let Some(Token::Number(line)) = tokens.get(i + 1) {
                        statements.push(Statement::Gosub {
                            line: line.parse().unwrap_or(0),
                        });
                        i += 2;
                    }
                }
                "RETURN" => {
                    statements.push(Statement::Return);
                    i += 1;
                }
                "DATA" => {
                    // Collect all values after DATA keyword (separated by commas)
                    let mut values = Vec::new();
                    let mut j = i + 1;
                    while j < tokens.len() && tokens[j] != Token::Newline {
                        if tokens[j] != Token::Comma {
                            values.push(tokens[j].clone());
                        }
                        j += 1;
                    }
                    statements.push(Statement::Data { values });
                    i = j;
                }
                "READ" => {
                    // Collect variable names (separated by commas)
                    let mut vars = Vec::new();
                    let mut j = i + 1;
                    while j < tokens.len() && tokens[j] != Token::Newline {
                        if let Token::Identifier(var) = &tokens[j] {
                            vars.push(var.clone());
                        }
                        j += 1;
                    }
                    statements.push(Statement::Read { vars });
                    i = j;
                }
                "RESTORE" => {
                    statements.push(Statement::Restore);
                    i += 1;
                }
                "CLS" => {
                    statements.push(Statement::Cls);
                    i += 1;
                }
                "LOCATE" => {
                    // LOCATE row, col
                    let mut j = i + 1;
                    let mut row_expr = Vec::new();

                    // Parse row expression until comma
                    while j < tokens.len()
                        && tokens[j] != Token::Comma
                        && tokens[j] != Token::Newline
                    {
                        row_expr.push(tokens[j].clone());
                        j += 1;
                    }

                    let mut col_expr = Vec::new();
                    if j < tokens.len() && tokens[j] == Token::Comma {
                        j += 1; // Skip comma
                                // Parse column expression
                        while j < tokens.len() && tokens[j] != Token::Newline {
                            col_expr.push(tokens[j].clone());
                            j += 1;
                        }
                    }

                    statements.push(Statement::Locate {
                        row: row_expr,
                        col: col_expr,
                    });
                    i = j;
                }
                "DIM" => {
                    // DIM A(10) or DIM A(10,10)
                    if let Some(Token::Identifier(var)) = tokens.get(i + 1) {
                        if tokens.get(i + 2) == Some(&Token::LeftParen) {
                            let mut j = i + 3;
                            let mut dims = Vec::new();

                            // Parse dimension sizes
                            while j < tokens.len() && tokens[j] != Token::RightParen {
                                if let Token::Number(n) = &tokens[j] {
                                    if let Ok(size) = n.parse::<usize>() {
                                        dims.push(size);
                                    }
                                }
                                j += 1;
                            }

                            statements.push(Statement::Dim {
                                var: var.clone(),
                                dims,
                            });
                            i = j + 1;
                        }
                    }
                }
                "OPTION" => {
                    // OPTION BASE 0 or OPTION BASE 1
                    if let Some(Token::Keyword(base_kw)) = tokens.get(i + 1) {
                        if base_kw == "BASE" {
                            if let Some(Token::Number(n)) = tokens.get(i + 2) {
                                if let Ok(base) = n.parse::<usize>() {
                                    if base == 0 || base == 1 {
                                        statements.push(Statement::OptionBase { base });
                                    }
                                }
                            }
                        }
                    }
                    i += 3;
                }
                "RESUME" => {
                    // RESUME or RESUME NEXT
                    let next = if let Some(Token::Keyword(kw)) = tokens.get(i + 1) {
                        kw == "NEXT"
                    } else {
                        false
                    };
                    statements.push(Statement::Resume { next });
                    i += if next { 2 } else { 1 };
                }
                "RANDOMIZE" => {
                    // RANDOMIZE or RANDOMIZE seed
                    let mut seed = None;
                    let mut j = i + 1;
                    let has_seed;
                    if j < tokens.len() && tokens[j] != Token::Newline {
                        let mut seed_expr = Vec::new();
                        while j < tokens.len() && tokens[j] != Token::Newline {
                            seed_expr.push(tokens[j].clone());
                            j += 1;
                        }
                        if !seed_expr.is_empty() {
                            seed = Some(seed_expr);
                            has_seed = true;
                        } else {
                            has_seed = false;
                        }
                    } else {
                        has_seed = false;
                    }
                    statements.push(Statement::Randomize { seed });
                    i = if has_seed { j } else { i + 1 };
                }
                "END" => {
                    // Check for END SELECT
                    if let Some(Token::Keyword(kw)) = tokens.get(i + 1) {
                        if kw == "SELECT" {
                            statements.push(Statement::EndSelect);
                            i += 2;
                            continue;
                        }
                    }
                    // Regular END statement
                    statements.push(Statement::End);
                    i += 1;
                }
                // Standalone FN call for side effects: `fn mci(...)` etc.
                "FN" => {
                    let mut expr = vec![tokens[i].clone()];
                    let mut j = i + 1;
                    while j < tokens.len() && tokens[j] != Token::Newline {
                        expr.push(tokens[j].clone());
                        j += 1;
                    }
                    statements.push(Statement::Expr { expr });
                    i = j;
                }
                _ => {}
            }
        } else if let Token::Identifier(var) = &tokens[i] {
            // Implied LET: X = expr or X(subscripts) = expr
            if tokens.get(i + 1) == Some(&Token::Equal) {
                // Simple variable assignment
                let mut expr = Vec::new();
                let mut j = i + 2;
                while j < tokens.len() && tokens[j] != Token::Newline {
                    expr.push(tokens[j].clone());
                    j += 1;
                }
                statements.push(Statement::Let {
                    var: var.clone(),
                    expr,
                });
                i = j;
                continue;
            } else if tokens.get(i + 1) == Some(&Token::LeftParen) {
                // Array assignment: X(subscripts) = expr
                let mut j = i + 2;
                let mut depth = 1;
                while j < tokens.len() && depth > 0 {
                    if tokens[j] == Token::LeftParen {
                        depth += 1;
                    } else if tokens[j] == Token::RightParen {
                        depth -= 1;
                    }
                    j += 1;
                }

                // j now points past the closing paren
                if j < tokens.len() && tokens[j] == Token::Equal {
                    let subscripts = tokens[i + 2..j - 1].to_vec();
                    let mut expr = Vec::new();
                    let mut k = j + 1;
                    while k < tokens.len() && tokens[k] != Token::Newline {
                        expr.push(tokens[k].clone());
                        k += 1;
                    }
                    statements.push(Statement::LetArray {
                        var: var.clone(),
                        subscripts,
                        expr,
                    });
                    i = k;
                    continue;
                }
            }
        }
        i += 1;
    }

    statements
}

struct Interpreter {
    variables: HashMap<String, Value>,
    program: HashMap<i32, Statement>,
    call_stack: Vec<usize>,
    for_loops: Vec<ForLoop>,
    while_loops: Vec<WhileLoop>,
    rng: rand::rngs::ThreadRng,
    data_values: Vec<Value>,
    data_pointer: usize,
    arrays: HashMap<String, Array>,
    option_base: usize,
    user_functions: HashMap<String, (String, Vec<Token>)>, // name -> (param, expr)
    external_functions: HashMap<String, FuncDef>,          // name -> (param, expr)
    exit_for_flag: bool,
    exit_while_flag: bool,
    file_handles: HashMap<i32, std::fs::File>,
    error_handler: Option<i32>,
    last_error: Option<String>,
    error_line: i32,
    resume_line: Option<usize>,
}


impl Interpreter {
    fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            program: HashMap::new(),
            call_stack: Vec::new(),
            for_loops: Vec::new(),
            while_loops: Vec::new(),
            rng: rand::thread_rng(),
            data_values: Vec::new(),
            data_pointer: 0,
            arrays: HashMap::new(),
            option_base: 0,
            user_functions: HashMap::new(),
            external_functions: HashMap::new(),
            exit_for_flag: false,
            exit_while_flag: false,
            file_handles: HashMap::new(),
            error_handler: None,
            last_error: None,
            error_line: 0,
            resume_line: None,
        }
    }

    fn store_line(&mut self, line_num: i32, stmt: &Statement) {
        self.program.insert(line_num, stmt.clone());
    }

    fn delete_line(&mut self, line_num: i32) {
        self.program.remove(&line_num);
    }

    fn list(&self) {
        let mut lines: Vec<_> = self.program.keys().collect();
        lines.sort();

        for line_num in lines {
            if let Some(stmt) = self.program.get(line_num) {
                println!("{} {:?}", line_num, stmt);
            }
        }
    }

    fn run(&mut self) {
        let mut lines: Vec<_> = self.program.keys().cloned().collect();
        lines.sort();

        // First pass: collect all DATA values
        self.data_values.clear();
        self.data_pointer = 0;
        for &line_num in &lines {
            if let Some(Statement::Data { values }) = self.program.get(&line_num) {
                for token in values {
                        match token {
                            Token::Number(n) => {
                                if let Ok(num) = n.parse::<f64>() {
                                    self.data_values.push(Value::Number(num));
                                }
                            }
                            Token::StringLiteral(s) => {
                                self.data_values.push(Value::String(s.clone()));
                            }
                            _ => {}
                        }
                    }
            }
        }

        let mut pc = 0; // program counter (index into lines vec)

        while pc < lines.len() {
            let line_num = lines[pc];

            // Check for RESUME - jump to the saved resume position
            if self.resume_line.is_some() {
                if let Some(resume_pc) = self.resume_line {
                    self.resume_line = None;
                    pc = resume_pc;
                    continue;
                }
            }

            if let Some(stmt) = self.program.get(&line_num).cloned() {
                match stmt {
                    Statement::Goto { line } => {
                        // Find the index of the target line
                        if let Some(pos) = lines.iter().position(|&l| l == line) {
                            pc = pos;
                            continue;
                        } else {
                            eprintln!("Error: Line {} not found", line);
                            break;
                        }
                    }
                    Statement::Gosub { line } => {
                        self.call_stack.push(pc + 1);
                        if let Some(pos) = lines.iter().position(|&l| l == line) {
                            pc = pos;
                            continue;
                        } else {
                            eprintln!("Error: Line {} not found", line);
                            break;
                        }
                    }
                    Statement::Return => {
                        if let Some(return_pc) = self.call_stack.pop() {
                            pc = return_pc;
                            continue;
                        } else {
                            eprintln!("Error: RETURN without GOSUB");
                            break;
                        }
                    }
                    Statement::OnGoto {
                        expr,
                        lines: target_lines,
                    } => {
                        let index_val = self.evaluate_expr(&expr);
                        if let Value::Number(n) = index_val {
                            let index = n.trunc() as usize;
                            if index >= 1 && index <= target_lines.len() {
                                let target = target_lines[index - 1];
                                if let Some(pos) = lines.iter().position(|&l| l == target) {
                                    pc = pos;
                                    continue;
                                }
                            }
                            // If index out of range, just continue to next line
                        }
                    }
                    Statement::OnGosub {
                        expr,
                        lines: target_lines,
                    } => {
                        let index_val = self.evaluate_expr(&expr);
                        if let Value::Number(n) = index_val {
                            let index = n.trunc() as usize;
                            if index >= 1 && index <= target_lines.len() {
                                let target = target_lines[index - 1];
                                self.call_stack.push(pc + 1);
                                if let Some(pos) = lines.iter().position(|&l| l == target) {
                                    pc = pos;
                                    continue;
                                }
                            }
                            // If index out of range, just continue to next line
                        }
                    }
                    Statement::If {
                        condition,
                        then_stmt,
                        else_stmt,
                        then_line,
                    } => {
                        if self.evaluate_condition(&condition) {
                            // Execute THEN part
                            if let Some(stmt) = then_stmt {
                                self.execute_statement(&stmt);
                            } else if let Some(line) = then_line {
                                if let Some(pos) = lines.iter().position(|&l| l == line) {
                                    pc = pos;
                                    continue;
                                }
                            }
                        } else {
                            // Execute ELSE part if it exists
                            if let Some(stmt) = else_stmt {
                                self.execute_statement(&stmt);
                            }
                        }
                    }
                    Statement::For {
                        var,
                        start,
                        end,
                        step,
                    } => {
                        // Evaluate and set the loop variable
                        let start_val = self.evaluate_expr(&start);
                        self.variables.insert(var.clone(), start_val);

                        // Evaluate end value and step
                        let end_val = if let Value::Number(n) = self.evaluate_expr(&end) {
                            n
                        } else {
                            0.0
                        };

                        let step_val = if let Some(s) = step {
                            if let Value::Number(n) = self.evaluate_expr(&s) {
                                n
                            } else {
                                1.0
                            }
                        } else {
                            1.0
                        };

                        // Push loop info onto stack (pc + 1 to start at next line)
                        self.for_loops.push(ForLoop {
                            var: var.clone(),
                            end_value: end_val,
                            step: step_val,
                            return_pc: pc + 1,
                        });
                    }
                    Statement::Next { var } => {
                        // Check if EXIT FOR was called
                        if self.exit_for_flag {
                            self.exit_for_flag = false;
                            if !self.for_loops.is_empty() {
                                self.for_loops.pop();
                            }
                            // Continue to next statement (pc will increment naturally)
                        } else if let Some(for_loop) = self.for_loops.last().cloned() {
                            // If NEXT has no variable, or if it matches the loop variable
                            if var.is_empty() || for_loop.var == var {
                                // Increment the loop variable
                                if let Some(Value::Number(current)) =
                                    self.variables.get(&for_loop.var)
                                {
                                    let new_val = current + for_loop.step;

                                    // Check if we should continue the loop
                                    let should_continue = if for_loop.step > 0.0 {
                                        new_val <= for_loop.end_value
                                    } else {
                                        new_val >= for_loop.end_value
                                    };

                                    if should_continue {
                                        self.variables
                                            .insert(for_loop.var.clone(), Value::Number(new_val));
                                        pc = for_loop.return_pc;
                                        continue;
                                    } else {
                                        // Loop finished, pop it
                                        self.for_loops.pop();
                                    }
                                }
                            }
                        }
                    }
                    Statement::ExitFor => {
                        self.exit_for_flag = true;
                        // The flag will be checked at the next NEXT statement
                    }
                    Statement::While { condition } => {
                        if self.evaluate_condition(&condition) {
                            // Push loop info (pc + 1 to start at next line)
                            self.while_loops.push(WhileLoop {
                                condition: condition.clone(),
                                start_pc: pc + 1,
                            });
                        } else {
                            // Skip to after WEND
                            let mut depth = 1;
                            let mut j = pc + 1;
                            while j < lines.len() && depth > 0 {
                                if let Some(stmt) = self.program.get(&lines[j]) {
                                    match stmt {
                                        Statement::While { .. } => depth += 1,
                                        Statement::Wend => depth -= 1,
                                        _ => {}
                                    }
                                }
                                j += 1;
                            }
                            pc = j - 1;
                        }
                    }
                    Statement::Wend => {
                        // Check if EXIT WHILE was called
                        if self.exit_while_flag {
                            self.exit_while_flag = false;
                            if !self.while_loops.is_empty() {
                                self.while_loops.pop();
                            }
                            // Continue to next statement
                        } else if let Some(while_loop) = self.while_loops.last().cloned() {
                            if self.evaluate_condition(&while_loop.condition) {
                                // Continue loop
                                pc = while_loop.start_pc;
                                continue;
                            } else {
                                // Exit loop
                                self.while_loops.pop();
                            }
                        }
                    }
                    Statement::ExitWhile => {
                        self.exit_while_flag = true;
                        // The flag will be checked at the next WEND statement
                    }
                    Statement::SelectCase { expr } => {
                        // Evaluate the SELECT expression
                        let select_value = self.evaluate_expr(&expr);

                        // Find matching CASE
                        let mut found_match = false;
                        let mut j = pc + 1;
                        while j < lines.len() {
                            if let Some(stmt) = self.program.get(&lines[j]).cloned() {
                                match stmt {
                                    Statement::Case { values } => {
                                        if !found_match {
                                            // Check if any value matches
                                            for case_val_expr in &values {
                                                let case_val = self.evaluate_expr(case_val_expr);
                                                let matches = match (&select_value, &case_val) {
                                                    (Value::Number(a), Value::Number(b)) => {
                                                        (a - b).abs() < f64::EPSILON
                                                    }
                                                    (Value::String(a), Value::String(b)) => a == b,
                                                    _ => false,
                                                };
                                                if matches {
                                                    found_match = true;
                                                    // Set pc to line after CASE, minus 1 because continue skips pc += 1
                                                    pc = j;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    Statement::CaseElse => {
                                        if !found_match {
                                            found_match = true;
                                            // Set pc to line after CASE ELSE, minus 1 because continue skips pc += 1
                                            pc = j;
                                        }
                                    }
                                    Statement::EndSelect => {
                                        if !found_match {
                                            pc = j; // Skip to END SELECT
                                        }
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                            j += 1;
                        }
                        // Don't continue here - let pc increment naturally to skip past the CASE statement
                    }
                    Statement::Case { .. } | Statement::CaseElse => {
                        // If we hit a CASE or CASE ELSE during normal execution,
                        // it means we're falling through from a previous case.
                        // Skip to END SELECT.
                        let mut depth = 1;
                        let mut j = pc + 1;
                        while j < lines.len() && depth > 0 {
                            if let Some(stmt) = self.program.get(&lines[j]) {
                                match stmt {
                                    Statement::SelectCase { .. } => depth += 1,
                                    Statement::EndSelect => depth -= 1,
                                    _ => {}
                                }
                            }
                            j += 1;
                        }
                        pc = j - 1;
                    }
                    Statement::EndSelect => {
                        // Just continue execution
                    }
                    Statement::End => {
                        break;
                    }
                    Statement::Resume { next } => {
                        // Set the resume position and continue
                        if next {
                            // RESUME NEXT - continue at next line after error
                            if let Some(err_pc) = lines.iter().position(|&l| l == self.error_line) {
                                self.resume_line = Some(err_pc + 1);
                            }
                        } else {
                            // RESUME - retry the same line that caused error
                            if let Some(err_pc) = lines.iter().position(|&l| l == self.error_line) {
                                self.resume_line = Some(err_pc);
                            }
                        }
                        // Clear the error
                        self.error_line = 0;
                        continue;
                    }
                    _ => {
                        // Wrap statement execution in error handler
                        if self.error_handler.is_some() {
                            // Execute with error handling
                            self.execute_statement_with_error_handling(
                                &stmt, line_num, &lines, &mut pc,
                            );
                        } else {
                            self.execute_statement(&stmt);
                        }
                    }
                }
            }
            pc += 1;
        }

        self.call_stack.clear();
        self.for_loops.clear();
        self.while_loops.clear();
    }

    fn execute_immediate(&mut self, statements: &[Statement]) {
        for stmt in statements {
            self.execute_statement(stmt);
        }
    }

    fn execute_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Compound(stmts) => {
                // Execute all statements in sequence
                for s in stmts {
                    self.execute_statement(s);
                }
            }
            Statement::Let { var, expr } => {
                let value = self.evaluate_expr(expr);
                match value {
                    Value::Struct(sv) => {
                        // Struct return from a C call — store as a struct-backed array so
                        // that field reads (e.g. lc(0)) are forwarded to dyncall.
                        self.arrays.insert(var.clone(), Array::from_struct_val(sv));
                    }
                    _ => {
                        self.variables.insert(var.clone(), value);
                    }
                }
            }
            Statement::Write { items } => {
                // WRITE outputs values with commas, strings in quotes
                for (idx, expr) in items.iter().enumerate() {
                    if idx > 0 {
                        print!(",");
                    }
                    let val = self.evaluate_expr(expr);
                    match val {
                        Value::String(s) => print!("\"{}\"", s),
                        Value::Struct(_) => {}
                        Value::Number(n) => {
                            if n.fract() == 0.0 && n.abs() < 1e10 {
                                print!("{}", n as i64);
                            } else {
                                print!("{}", n);
                            }
                        }
                    }
                }
                println!();
            }
            Statement::WriteFile { file_num, items } => {
                use std::io::Write;
                // Evaluate all items first to avoid borrowing issues
                let mut values = Vec::new();
                for expr in items {
                    values.push(self.evaluate_expr(expr));
                }

                // Now write to file
                if let Some(file) = self.file_handles.get_mut(file_num) {
                    for (idx, val) in values.iter().enumerate() {
                        if idx > 0 {
                            write!(file, ",").ok();
                        }
                        match val {
                            Value::String(s) => write!(file, "\"{}\"", s).ok(),
                            Value::Struct(_) => None,
                            Value::Number(n) => {
                                if n.fract() == 0.0 && n.abs() < 1e10 {
                                    write!(file, "{}", *n as i64).ok()
                                } else {
                                    write!(file, "{}", n).ok()
                                }
                            }
                        };
                    }
                    writeln!(file).ok();
                }
            }
            Statement::Print { items, no_newline } => {
                // Handle empty PRINT (just print newline)
                if items.is_empty() {
                    println!();
                    let _ = io::stdout().flush();
                    return;
                }

                let mut column = 0;
                const TAB_WIDTH: usize = 14;

                for item in items {
                    match item {
                        PrintItem::Expr(expr) => {
                            // Check if this is a TAB(n) or SPC(n) function call
                            if expr.len() >= 4 {
                                if let (
                                    Token::Function(fname),
                                    Token::LeftParen,
                                    _,
                                    Token::RightParen,
                                ) = (
                                    &expr[0],
                                    &expr[1],
                                    &expr[2],
                                    expr.get(3).unwrap_or(&Token::Newline),
                                ) {
                                    if fname == "TAB" {
                                        // Evaluate TAB argument to get target column
                                        let tab_expr = vec![expr[2].clone()];
                                        if let Value::Number(n) = self.evaluate_expr(&tab_expr) {
                                            let target = n as usize;
                                            if target > column {
                                                let spaces = target - column;
                                                print!("{}", " ".repeat(spaces));
                                                column = target;
                                            }
                                        }
                                        continue;
                                    } else if fname == "SPC" {
                                        // Evaluate SPC argument to get number of spaces
                                        let spc_expr = vec![expr[2].clone()];
                                        if let Value::Number(n) = self.evaluate_expr(&spc_expr) {
                                            let spaces = n as usize;
                                            print!("{}", " ".repeat(spaces));
                                            column += spaces;
                                        }
                                        continue;
                                    }
                                }
                            }

                            // Regular expression evaluation
                            let value = self.evaluate_expr(expr);
                            let text = match value {
                                Value::Number(n) => n.to_string(),
                                Value::String(s) => s,
                                Value::Struct(_) => String::new(),
                            };
                            print!("{}", text);
                            column += text.len();
                        }
                        PrintItem::Comma => {
                            // Move to next tab zone
                            let spaces_needed = TAB_WIDTH - (column % TAB_WIDTH);
                            if spaces_needed < TAB_WIDTH {
                                print!("{}", " ".repeat(spaces_needed));
                                column += spaces_needed;
                            }
                        }
                        PrintItem::Semicolon => {
                            // No spacing, just continue
                        }
                    }
                }

                if !*no_newline {
                    println!();
                    io::stdout().flush().unwrap();
                } else {
                    io::stdout().flush().unwrap();
                }
            }
            Statement::Input { prompt, vars } => {
                // Print prompt if provided
                if let Some(p) = prompt {
                    print!("{}", p);
                } else {
                    print!("? ");
                }
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                // Split input by commas for multiple variables
                let values: Vec<&str> = input.split(',').map(|s| s.trim()).collect();

                for (i, var) in vars.iter().enumerate() {
                    let val = values.get(i).unwrap_or(&"");
                    if let Ok(num) = val.parse::<f64>() {
                        self.variables.insert(var.clone(), Value::Number(num));
                    } else {
                        self.variables
                            .insert(var.clone(), Value::String(val.to_string()));
                    }
                }
            }
            Statement::Swap { var1, var2 } => {
                let val1 = self
                    .variables
                    .get(var1)
                    .cloned()
                    .unwrap_or(Value::Number(0.0));
                let val2 = self
                    .variables
                    .get(var2)
                    .cloned()
                    .unwrap_or(Value::Number(0.0));
                self.variables.insert(var1.clone(), val2);
                self.variables.insert(var2.clone(), val1);
            }
            Statement::Read { vars } => {
                for var in vars {
                    if self.data_pointer >= self.data_values.len() {
                        eprintln!("Error: Out of DATA");
                        break;
                    }
                    let value = self.data_values[self.data_pointer].clone();
                    self.variables.insert(var.clone(), value);
                    self.data_pointer += 1;
                }
            }
            Statement::Restore => {
                self.data_pointer = 0;
            }
            Statement::Data { .. } => {
                // DATA statements are processed during program initialization, skip during execution
            }
            Statement::Cls => {
                // Clear screen using ANSI escape codes
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().unwrap();
            }
            Statement::Locate { row, col } => {
                // Position cursor at row, col (1-based)
                use crossterm::cursor::MoveTo;
                use crossterm::execute;

                let row_val = if !row.is_empty() {
                    if let Value::Number(n) = self.evaluate_expr(row) {
                        (n as u16).saturating_sub(1) // Convert to 0-based
                    } else {
                        0
                    }
                } else {
                    0
                };

                let col_val = if !col.is_empty() {
                    if let Value::Number(n) = self.evaluate_expr(col) {
                        (n as u16).saturating_sub(1) // Convert to 0-based
                    } else {
                        0
                    }
                } else {
                    0
                };

                execute!(io::stdout(), MoveTo(col_val, row_val)).ok();
            }
            Statement::Dim { var, dims } => {
                if dims.is_empty() {
                    eprintln!("Error: DIM requires at least one dimension");
                } else if dims.len() > 60 {
                    eprintln!("Error: Maximum 60 dimensions supported");
                } else {
                    // Create bounds: lower = option_base, upper = specified dim value
                    let lower_bounds = vec![self.option_base; dims.len()];
                    let upper_bounds = dims.clone();
                    let array = Array::new(lower_bounds, upper_bounds);
                    self.arrays.insert(var.clone(), array);
                }
            }
            Statement::OptionBase { base } => {
                self.option_base = *base;
            }
            Statement::LetArray {
                var,
                subscripts,
                expr,
            } => {
                let subs = self.parse_subscripts(subscripts);
                let value = self.evaluate_expr(expr);
                self.set_array_element(var, &subs, value);
            }
            Statement::MidAssign {
                var,
                start_expr,
                length_expr,
                value_expr,
            } => {
                // Get the variable's current value
                let current_val = self
                    .variables
                    .get(var)
                    .cloned()
                    .unwrap_or(Value::String(String::new()));
                let s = match current_val {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    Value::Struct(_) => String::new(),
                };

                // Evaluate start position and length
                let start_val = self.evaluate_expr(start_expr);
                let start = match start_val {
                    Value::Number(n) => (n as usize).saturating_sub(1), // BASIC is 1-indexed
                    _ => 0,
                };

                let length_val = self.evaluate_expr(length_expr);
                let length = match length_val {
                    Value::Number(n) => n as usize,
                    _ => 0,
                };

                // Evaluate the value to insert
                let new_val = self.evaluate_expr(value_expr);
                let new_str = match new_val {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    Value::Struct(_) => String::new(),
                };

                // Replace the substring
                let mut chars: Vec<char> = s.chars().collect();
                let end = (start + length).min(chars.len());

                // Remove the old substring
                if start < chars.len() {
                    chars.drain(start..end);
                }

                // Insert the new substring at the start position
                let new_chars: Vec<char> = new_str.chars().collect();
                for (i, c) in new_chars.iter().enumerate() {
                    chars.insert(start + i, *c);
                }

                self.variables
                    .insert(var.clone(), Value::String(chars.iter().collect()));
            }
            Statement::Randomize { seed } => {
                if let Some(Value::Number(n)) = seed.as_ref().map(|e| self.evaluate_expr(e)) {
                    let iterations = (n.abs() as usize) % 1000;
                    for _ in 0..iterations {
                        self.rng.gen::<f64>();
                    }
                }
            }
            Statement::DefFn { name, param, expr } => {
                self.user_functions
                    .insert(name.clone(), (param.clone(), expr.clone()));
            }
            Statement::DefXfn { name, defstr } => {
                let fdef = DynCaller::define_function(defstr).unwrap();

                self.external_functions.insert(name.clone(), fdef);
            }
            Statement::ExitFor => {
                self.exit_for_flag = true;
            }
            Statement::ExitWhile => {
                self.exit_while_flag = true;
            }
            Statement::SelectCase { .. }
            | Statement::Case { .. }
            | Statement::CaseElse
            | Statement::EndSelect => {
                // These are handled in run() method during flow control
            }
            Statement::LineInput { prompt, var } => {
                if let Some(p) = prompt {
                    print!("{}", p);
                    io::stdout().flush().unwrap();
                }
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                // Don't trim or split - keep the entire line including commas
                self.variables.insert(
                    var.clone(),
                    Value::String(
                        input
                            .trim_end_matches('\n')
                            .trim_end_matches('\r')
                            .to_string(),
                    ),
                );
            }
            Statement::LineInputFile { file_num, var } => {
                use std::io::Read;
                if let Some(file) = self.file_handles.get_mut(file_num) {
                    let mut line_bytes = Vec::new();
                    // Read until newline
                    let mut buf = [0u8; 1];
                    loop {
                        match file.read(&mut buf) {
                            Ok(0) => break, // EOF
                            Ok(_) => {
                                if buf[0] == b'\n' {
                                    break;
                                }
                                line_bytes.push(buf[0]);
                            }
                            Err(_) => break,
                        }
                    }

                    // Convert bytes to string and remove \r if present
                    let line = String::from_utf8_lossy(&line_bytes).to_string();
                    let trimmed = line.trim_end_matches('\r').to_string();
                    self.variables.insert(var.clone(), Value::String(trimmed));
                }
            }
            Statement::Open {
                filename,
                mode,
                file_num,
            } => {
                let filename_val = self.evaluate_expr(filename);
                let filename_str = match filename_val {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    Value::Struct(_) => String::new(),
                };

                use std::fs::OpenOptions;
                let file_result = match mode.as_str() {
                    "INPUT" => OpenOptions::new().read(true).open(&filename_str),
                    "OUTPUT" => OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(&filename_str),
                    "APPEND" => OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&filename_str),
                    _ => {
                        eprintln!("Error: Unknown file mode '{}'", mode);
                        return;
                    }
                };

                match file_result {
                    Ok(file) => {
                        self.file_handles.insert(*file_num, file);
                    }
                    Err(e) => {
                        self.last_error = Some(format!("File open error: {}", e));
                        eprintln!("Error opening file '{}': {}", filename_str, e);
                    }
                }
            }
            Statement::Close { file_num } => {
                if self.file_handles.remove(file_num).is_none() {
                    eprintln!("Error: File #{} not open", file_num);
                }
            }
            Statement::PrintFile { file_num, items } => {
                use std::io::Write;
                // Pre-evaluate all items to avoid borrow checker issues
                let mut outputs: Vec<(String, bool, bool, bool, usize)> = Vec::new(); // (text, is_comma, is_semicolon, is_spc, spc_count)
                let mut col = 0;
                for item in items {
                    match item {
                        PrintItem::Expr(tokens) => {
                            // Check if this is a SPC(n) or TAB(n) function call
                            if tokens.len() >= 4 {
                                if let (
                                    Token::Function(fname),
                                    Token::LeftParen,
                                    _,
                                    Token::RightParen,
                                ) = (
                                    &tokens[0],
                                    &tokens[1],
                                    &tokens[2],
                                    tokens.get(3).unwrap_or(&Token::Newline),
                                ) {
                                    if fname == "SPC" {
                                        // Evaluate SPC argument to get number of spaces
                                        let spc_expr = vec![tokens[2].clone()];
                                        if let Value::Number(n) = self.evaluate_expr(&spc_expr) {
                                            let spaces = n as usize;
                                            outputs.push((
                                                String::new(),
                                                false,
                                                false,
                                                true,
                                                spaces,
                                            ));
                                            continue;
                                        }
                                    } else if fname == "TAB" {
                                        // For file output, treat TAB like SPC
                                        let tab_expr = vec![tokens[2].clone()];
                                        if let Value::Number(n) = self.evaluate_expr(&tab_expr) {
                                            let target = n as usize;
                                            outputs.push((
                                                String::new(),
                                                false,
                                                false,
                                                true,
                                                target,
                                            ));
                                            continue;
                                        }
                                    }
                                }
                            }

                            let val = self.evaluate_expr(tokens);
                            let output = match val {
                                Value::Number(n) => {
                                    if n.fract() == 0.0 {
                                        format!("{}", n as i32)
                                    } else {
                                        format!("{}", n)
                                    }
                                }
                                Value::String(s) => s,
                                Value::Struct(_) => String::new(),
                            };
                            outputs.push((output, false, false, false, 0));
                        }
                        PrintItem::Comma => {
                            outputs.push((String::new(), true, false, false, 0));
                        }
                        PrintItem::Semicolon => {
                            outputs.push((String::new(), false, true, false, 0));
                        }
                    }
                }

                // Now write to file
                if let Some(file) = self.file_handles.get_mut(file_num) {
                    for (output, is_comma, is_semicolon, is_spc, spc_count) in outputs {
                        if is_comma {
                            let spaces = 14 - (col % 14);
                            write!(file, "{}", " ".repeat(spaces)).ok();
                            col += spaces;
                        } else if is_spc {
                            write!(file, "{}", " ".repeat(spc_count)).ok();
                            col += spc_count;
                        } else if !is_semicolon {
                            write!(file, "{}", output).ok();
                            col += output.len();
                        }
                    }
                    // Add newline if not suppressed
                    if !items.is_empty() && !matches!(
                        items.last(),
                        Some(PrintItem::Comma) | Some(PrintItem::Semicolon)
                    ) {
                        writeln!(file).ok();
                    }
                }
            }
            Statement::InputFile { file_num, vars } => {
                use std::io::Read;
                if let Some(file) = self.file_handles.get_mut(file_num) {
                    let mut buffer = String::new();
                    if file.read_to_string(&mut buffer).is_ok() {
                        let parts: Vec<&str> = buffer.split(',').collect();
                        for (i, var) in vars.iter().enumerate() {
                            if let Some(part) = parts.get(i) {
                                let trimmed = part.trim();
                                if let Ok(n) = trimmed.parse::<f64>() {
                                    self.variables.insert(var.clone(), Value::Number(n));
                                } else {
                                    self.variables
                                        .insert(var.clone(), Value::String(trimmed.to_string()));
                                }
                            }
                        }
                    }
                }
            }
            Statement::OnError { line } => {
                if *line == 0 {
                    // ON ERROR GOTO 0 disables error handler
                    self.error_handler = None;
                } else {
                    self.error_handler = Some(*line);
                }
            }
            Statement::Resume { .. } => {
                // RESUME is handled in run() method, not here
                // This shouldn't be executed directly
            }
            Statement::Expr { expr } => {
                // Evaluate for side effects, discard return value
                let processed = self.process_functions_and_parentheses(expr);
                self.evaluate_expr(&processed);
            }
            _ => {}
        }
    }

    fn execute_statement_with_error_handling(
        &mut self,
        stmt: &Statement,
        line_num: i32,
        lines: &[i32],
        pc: &mut usize,
    ) {
        // Execute the statement
        self.execute_statement(stmt);

        // Check if an error occurred (marked by error_line < 0)
        if self.error_line < 0 {
            // Extract error code from error_line
            let err_code = -self.error_line;
            // Set the actual error line and jump to handler
            self.error_line = line_num;
            // Store error code in a variable for ERR function
            self.variables
                .insert("__ERR__".to_string(), Value::Number(err_code as f64));

            if let Some(handler_line) = self.error_handler {
                if let Some(handler_pc) = lines.iter().position(|&l| l == handler_line) {
                    *pc = handler_pc - 1; // -1 because pc will be incremented
                }
            }
        }
    }

    fn evaluate_condition(&mut self, tokens: &[Token]) -> bool {
        // Handle AND/OR/NOT logical operators
        // First check for OR (lowest precedence)
        let mut depth = 0;
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::LeftParen => depth += 1,
                Token::RightParen => depth -= 1,
                Token::Or if depth == 0 => {
                    // Split on OR
                    let left = self.evaluate_condition(&tokens[..i]);
                    let right = self.evaluate_condition(&tokens[i + 1..]);
                    return left || right;
                }
                _ => {}
            }
        }

        // Then check for AND
        depth = 0;
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::LeftParen => depth += 1,
                Token::RightParen => depth -= 1,
                Token::And if depth == 0 => {
                    // Split on AND
                    let left = self.evaluate_condition(&tokens[..i]);
                    let right = self.evaluate_condition(&tokens[i + 1..]);
                    return left && right;
                }
                _ => {}
            }
        }

        // Handle NOT (highest precedence)
        if !tokens.is_empty() && tokens[0] == Token::Not {
            return !self.evaluate_condition(&tokens[1..]);
        }

        // Handle parentheses
        if !tokens.is_empty()
            && tokens[0] == Token::LeftParen
            && tokens[tokens.len() - 1] == Token::RightParen
        {
            return self.evaluate_condition(&tokens[1..tokens.len() - 1]);
        }

        // Find the comparison operator
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut comparison = None;
        let mut found_comp = false;

        for token in tokens {
            if let Token::Comparison(op) = token {
                comparison = Some(op.clone());
                found_comp = true;
            } else if let Token::Equal = token {
                comparison = Some("=".to_string());
                found_comp = true;
            } else if !found_comp {
                left.push(token.clone());
            } else {
                right.push(token.clone());
            }
        }

        if let Some(op) = comparison {
            let left_val = self.evaluate_expr(&left);
            let right_val = self.evaluate_expr(&right);
            let left_compare = match left_val {
                Value::Number(n) => Value::String(n.to_string()),
                Value::String(s) => Value::String(s),
                Value::Struct(_) => Value::String(String::new()),
            };
            let right_compare = match right_val {
                Value::Number(n) => Value::String(n.to_string()),
                Value::String(s) => Value::String(s),
                Value::Struct(_) => Value::String(String::new()),
            };

            match left_compare {
                Value::String(ref s) if s == "27" => {
                    // Debug print
                    println!("Comparing: {:?} {} {:?}", left_compare, op, right_compare);
                }
                _ => {}
            }

            match (left_compare, right_compare) {
                (Value::String(l), Value::String(r)) => match op.as_str() {
                    "=" => l == r,
                    "<>" => l != r,
                    "<" => l < r,
                    ">" => l > r,
                    "<=" => l <= r,
                    ">=" => l >= r,
                    _ => false,
                },
                _ => false,
            }
        } else {
            false
        }
    }

    fn evaluate_expr(&mut self, tokens: &[Token]) -> Value {
        if tokens.is_empty() {
            return Value::Number(0.0);
        }

        // Check if it's a single string literal
        if tokens.len() == 1 {
            if let Token::StringLiteral(s) = &tokens[0] {
                return Value::String(s.clone());
            }
            if let Token::Struct(sv) = &tokens[0] {
                return Value::Struct(sv.clone());
            }
            if let Token::Identifier(id) = &tokens[0] {
                // Special variables
                if id == "ERR" {
                    // Return error code stored in __ERR__ variable
                    if let Some(err_val) = self.variables.get("__ERR__") {
                        return err_val.clone();
                    }
                    return Value::Number(0.0);
                }
                if id == "ERL" {
                    return Value::Number(self.error_line as f64);
                }
                if let Some(val) = self.variables.get(id) {
                    return val.clone();
                }
            }
        }

        // Handle negative numbers: if first token is - followed by a number
        let tokens = if tokens.len() >= 2 {
            if let (Token::Operator('-'), Token::Number(n)) = (&tokens[0], &tokens[1]) {
                if tokens.len() == 2 {
                    // Just a negative number
                    return Value::Number(-n.parse::<f64>().unwrap_or(0.0));
                } else {
                    // Negative number followed by more expression
                    let mut new_tokens = vec![Token::Number(format!("-{}", n))];
                    new_tokens.extend_from_slice(&tokens[2..]);
                    new_tokens
                }
            } else {
                tokens.to_vec()
            }
        } else {
            tokens.to_vec()
        };

        // Process functions and parentheses first
        let processed_tokens = self.process_functions_and_parentheses(&tokens);

        // After processing, check if we have strings to concatenate
        let has_string_results = processed_tokens
            .iter()
            .any(|t| matches!(t, Token::StringLiteral(_)))
            || processed_tokens.iter().any(|t| {
                if let Token::Identifier(id) = t {
                    if let Some(val) = self.variables.get(id) {
                        matches!(val, Value::String(_))
                    } else {
                        false
                    }
                } else {
                    false
                }
            });

        // If we have strings and + operators after processing, handle string concatenation
        if has_string_results
            && processed_tokens
                .iter()
                .any(|t| matches!(t, Token::Operator('+')))
        {
            return self.evaluate_string_expr(&tokens); // Re-process from original tokens
        }

        // Check if we got a single result after processing
        if processed_tokens.len() == 1 {
            if let Token::StringLiteral(s) = &processed_tokens[0] {
                return Value::String(s.clone());
            }
            if let Token::Struct(sv) = &processed_tokens[0] {
                return Value::Struct(sv.clone());
            }
        }

        // Convert tokens to values and operators
        let mut values = Vec::new();
        let mut operators = Vec::new();

        for token in &processed_tokens {
            match token {
                Token::Number(_) | Token::Identifier(_) => {
                    values.push(self.get_numeric_value(token));
                }
                Token::Function(func_name) => {
                    // Handle standalone function calls like RND without parentheses
                    if func_name == "RND" {
                        values.push(self.rng.gen::<f64>());
                    } else {
                        values.push(0.0);
                    }
                }
                Token::Operator(op) => {
                    operators.push(*op);
                }
                Token::Mod => {
                    operators.push('%'); // Use % internally for MOD
                }
                _ => {}
            }
        }

        if values.is_empty() {
            return Value::Number(0.0);
        }

        // First pass: handle ^ (exponentiation) - right associative
        let (values, operators) = self.apply_exponentiation(values, operators);

        // Second pass: handle * and /
        let (mut new_values, mut new_operators) = (vec![values[0]], Vec::new());

        for i in 0..operators.len() {
            // Make sure we have a right operand
            if i + 1 >= values.len() {
                break;
            }

            match operators[i] {
                '*' => {
                    let last = new_values.pop().unwrap();
                    new_values.push(last * values[i + 1]);
                }
                '/' => {
                    let last = new_values.pop().unwrap();
                    if values[i + 1] != 0.0 {
                        new_values.push(last / values[i + 1]);
                    } else {
                        // Set error code for division by zero (ERR = 11)
                        self.error_line = -11; // Negative to indicate error type
                        new_values.push(0.0);
                    }
                }
                '%' => {
                    // MOD operator
                    let last = new_values.pop().unwrap();
                    if values[i + 1] != 0.0 {
                        new_values.push(last % values[i + 1]);
                    } else {
                        eprintln!("Error: MOD by zero");
                        new_values.push(0.0);
                    }
                }
                op => {
                    new_values.push(values[i + 1]);
                    new_operators.push(op);
                }
            }
        }

        // Third pass: handle + and -
        let mut result = new_values[0];
        for i in 0..new_operators.len() {
            if i + 1 < new_values.len() {
                match new_operators[i] {
                    '+' => result += new_values[i + 1],
                    '-' => result -= new_values[i + 1],
                    _ => {}
                }
            }
        }

        Value::Number(result)
    }

    fn evaluate_string_expr(&mut self, tokens: &[Token]) -> Value {
        // Process functions first
        let tokens = self.process_functions_and_parentheses(tokens);

        // Collect string/value parts and operators
        let mut parts = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            match &tokens[i] {
                Token::StringLiteral(s) => {
                    parts.push(Value::String(s.clone()));
                }
                Token::Identifier(id) => {
                    if let Some(val) = self.variables.get(id) {
                        parts.push(val.clone());
                    } else {
                        parts.push(Value::Number(0.0));
                    }
                }
                Token::Number(n) => {
                    parts.push(Value::Number(n.parse().unwrap_or(0.0)));
                }
                Token::Operator('+') => {
                    // Skip operators, we'll concatenate all parts
                }
                _ => {}
            }
            i += 1;
        }

        if parts.is_empty() {
            return Value::String(String::new());
        }

        // Concatenate all parts
        let mut result = String::new();
        for part in parts {
            match part {
                Value::String(s) => result.push_str(&s),
                Value::Number(n) => result.push_str(&n.to_string()),
                Value::Struct(_) => {}
            }
        }

        Value::String(result)
    }

    fn apply_exponentiation(
        &self,
        values: Vec<f64>,
        operators: Vec<char>,
    ) -> (Vec<f64>, Vec<char>) {
        // Find all ^ operators - if none, return unchanged
        if !operators.contains(&'^') {
            return (values, operators);
        }

        // Process exponentiation from right to left (right-associative)
        // In values/operators arrays: values[i] operators[i] values[i+1] operators[i+1] values[i+2] ...
        let mut result_values = Vec::new();
        let mut result_operators = Vec::new();

        let mut i = 0;
        while i < values.len() {
            // Check if value[i] is followed by ^ operator
            if i < operators.len() && operators[i] == '^' {
                // Find consecutive ^ operations starting at i
                let start_val_idx = i;
                let mut op_idx = i;
                while op_idx < operators.len() && operators[op_idx] == '^' {
                    op_idx += 1;
                }
                // Now we have values[start_val_idx] ^ values[start_val_idx+1] ^ ... ^ values[op_idx]
                // with op_idx-start_val_idx consecutive ^ operators

                // Calculate right-associative exponentiation
                let mut result = values[op_idx];
                for j in (start_val_idx..op_idx).rev() {
                    result = values[j].powf(result);
                }

                result_values.push(result);

                // Skip past all the values and ^ operators we just processed
                i = op_idx + 1;

                // If there's a non-^ operator after the last value, add it
                if op_idx < operators.len() {
                    result_operators.push(operators[op_idx]);
                }
            } else {
                // This value is not followed by ^, just add it
                result_values.push(values[i]);
                if i < operators.len() {
                    result_operators.push(operators[i]);
                }
                i += 1;
            }
        }

        (result_values, result_operators)
    }

    fn process_functions_and_parentheses(&mut self, tokens: &[Token]) -> Vec<Token> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            // Check for FN calls: FN name(arg)
            if let Token::Keyword(kw) = &tokens[i] {
                if kw == "FN" && i + 1 < tokens.len() {
                    if let Token::Identifier(func_name) = &tokens[i + 1] {
                        if i + 2 < tokens.len() && tokens[i + 2] == Token::LeftParen {
                            // Find matching right paren
                            let mut depth = 1;
                            let mut j = i + 3;
                            while j < tokens.len() && depth > 0 {
                                if tokens[j] == Token::LeftParen {
                                    depth += 1;
                                } else if tokens[j] == Token::RightParen {
                                    depth -= 1;
                                }
                                j += 1;
                            }

                            // Parse all arguments (split by commas)
                            let arg_tokens = &tokens[i + 3..j - 1];
                            let mut args = Vec::new();
                            let mut current_arg = Vec::new();
                            let mut paren_depth = 0;

                            for token in arg_tokens {
                                match token {
                                    Token::LeftParen => {
                                        paren_depth += 1;
                                        current_arg.push(token.clone());
                                    }
                                    Token::RightParen => {
                                        paren_depth -= 1;
                                        current_arg.push(token.clone());
                                    }
                                    Token::Comma if paren_depth == 0 => {
                                        if !current_arg.is_empty() {
                                            args.push(current_arg.clone());
                                            current_arg.clear();
                                        }
                                    }
                                    _ => {
                                        current_arg.push(token.clone());
                                    }
                                }
                            }
                            if !current_arg.is_empty() {
                                args.push(current_arg);
                            }

                            // Call user-defined function (single arg only)
                            if let Some((param, expr)) = self.user_functions.get(func_name).cloned()
                            {
                                // Evaluate the single argument
                                let arg_value = if args.is_empty() {
                                    Value::Number(0.0)
                                } else {
                                    let arg_processed =
                                        self.process_functions_and_parentheses(&args[0]);
                                    self.evaluate_expr(&arg_processed)
                                };

                                // Save current value of parameter variable (if it exists)
                                let saved_value = self.variables.get(&param).cloned();

                                // Set parameter to argument value
                                self.variables.insert(param.clone(), arg_value);

                                // Evaluate function expression
                                let func_result = self.evaluate_expr(&expr);

                                // Restore parameter variable
                                if let Some(val) = saved_value {
                                    self.variables.insert(param, val);
                                } else {
                                    self.variables.remove(&param);
                                }

                                // Push result
                                match func_result {
                                    Value::Number(n) => result.push(Token::Number(n.to_string())),
                                    Value::String(s) => result.push(Token::StringLiteral(s)),
                                    Value::Struct(sv) => result.push(Token::Struct(sv)),
                                }
                            } else if let Some(fdef) =
                                self.external_functions.get(func_name).cloned()
                            {
                                // Evaluate all arguments for external function
                                let mut arg_values = Vec::new();
                                for arg in &args {
                                    let arg_processed = self.process_functions_and_parentheses(arg);
                                    let arg_value = self.evaluate_expr(&arg_processed);
                                    arg_values.push(arg_value);
                                }
                                self.handle_external_call(&fdef, &args, arg_values, &mut result);
                                // let mut invoke = fdef.prep();
                                // match arg_value {
                                //     Value::Number(num) => invoke.push_arg(&(num as i64)),
                                //     Value::String(str) => invoke.push_arg(&str),
                                // }
                                // let ret = invoke.call();
                                // match ret {
                                //     ArgVal::I32(n) => {
                                //         result.push(Token::Number(n.to_string()));
                                //     }
                                //     // ArgVal::RustString(s) => {
                                //     //     result.push(Token::StringLiteral(s));
                                //     // }
                                //     _ => {
                                //         result.push(Token::Number("0".to_string()));
                                //     }
                                // }
                                // println!("External FN {} returned {:?}", func_name, ret);
                            } else {
                                // Function not defined
                                result.push(Token::Number("0".to_string()));
                            }

                            i = j;
                            continue;
                        }
                    }
                }
                // Not an FN call, just push the keyword
                result.push(tokens[i].clone());
                i += 1;
            } else if let Token::Function(func_name) = &tokens[i] {
                // Check if it's a string function or special function that needs token processing
                let string_funcs = [
                    "LEN", "LEFT$", "RIGHT$", "MID$", "UCASE$", "LCASE$", "INSTR$", "CHR$", "ASC",
                    "STR$", "VAL", "SPACE$", "STRING$", "LTRIM$", "RTRIM$", "TRIM$", "HEX$",
                    "OCT$", "INKEY$",
                ];
                if string_funcs.contains(&func_name.as_str()) {
                    // Special case: INKEY$ can be called without parentheses
                    if func_name == "INKEY$"
                        && (i + 1 >= tokens.len() || tokens[i + 1] != Token::LeftParen)
                    {
                        // INKEY$ with no arguments
                        let func_result = self.evaluate_string_function(func_name, &[]);
                        match func_result {
                            Value::String(s) => result.push(Token::StringLiteral(s)),
                            Value::Number(n) => result.push(Token::Number(n.to_string())),
                            Value::Struct(sv) => result.push(Token::Struct(sv)),
                        }
                        i += 1;
                    } else if i + 1 < tokens.len() && tokens[i + 1] == Token::LeftParen {
                        // Handle string functions with parentheses
                        let mut depth = 1;
                        let mut j = i + 2;
                        while j < tokens.len() && depth > 0 {
                            if tokens[j] == Token::LeftParen {
                                depth += 1;
                            } else if tokens[j] == Token::RightParen {
                                depth -= 1;
                            }
                            j += 1;
                        }

                        // Parse arguments inside function
                        let args_tokens = &tokens[i + 2..j - 1];
                        let func_result = self.evaluate_string_function(func_name, args_tokens);

                        // Push result as appropriate token
                        match func_result {
                            Value::String(s) => result.push(Token::StringLiteral(s)),
                            Value::Number(n) => result.push(Token::Number(n.to_string())),
                            Value::Struct(sv) => result.push(Token::Struct(sv)),
                        }
                        i = j;
                    } else {
                        result.push(tokens[i].clone());
                        i += 1;
                    }
                } else {
                    // Numeric function
                    if i + 1 < tokens.len() && tokens[i + 1] == Token::LeftParen {
                        // Find matching right paren
                        let mut depth = 1;
                        let mut j = i + 2;
                        while j < tokens.len() && depth > 0 {
                            if tokens[j] == Token::LeftParen {
                                depth += 1;
                            } else if tokens[j] == Token::RightParen {
                                depth -= 1;
                            }
                            j += 1;
                        }

                        // Recursively evaluate the argument
                        let arg_tokens = &tokens[i + 2..j - 1];
                        let arg_processed = self.process_functions_and_parentheses(arg_tokens);
                        let arg_value = self.evaluate_expr(&arg_processed);

                        if let Value::Number(arg) = arg_value {
                            let func_result = self.evaluate_function(func_name, arg);
                            result.push(Token::Number(func_result.to_string()));
                        }
                        i = j;
                    } else {
                        result.push(tokens[i].clone());
                        i += 1;
                    }
                }
            } else if let Token::Identifier(id) = &tokens[i] {
                // Check if this is a user-defined function call (FN followed by name)
                // Actually, FN calls look like: Token::Keyword("FN"), Token::Identifier(name), Token::LeftParen...
                // But here we're in identifier context, so let's check if it's an array or regular var

                // Check if this is an array access (identifier followed by left paren, but not a function)
                let is_function = [
                    "ABS", "ATN", "COS", "EXP", "INT", "LOG", "RND", "SIN", "SQR", "TAN", "VAL",
                    "TAB", "LEN", "LEFT$", "RIGHT$", "MID$", "UCASE$", "LCASE$", "INSTR$", "CHR$",
                    "ASC", "STR$", "SPACE$", "STRING$",
                ]
                .contains(&id.as_str());
                if !is_function && i + 1 < tokens.len() && tokens[i + 1] == Token::LeftParen {
                    // Find matching right paren
                    let mut depth = 1;
                    let mut j = i + 2;
                    while j < tokens.len() && depth > 0 {
                        if tokens[j] == Token::LeftParen {
                            depth += 1;
                        } else if tokens[j] == Token::RightParen {
                            depth -= 1;
                        }
                        j += 1;
                    }

                    // Parse subscripts (comma-separated)
                    let subscript_tokens = &tokens[i + 2..j - 1];
                    let subscripts = self.parse_subscripts(subscript_tokens);

                    // Get array value
                    let array_value = self.get_array_element(id, &subscripts);
                    match array_value {
                        Value::Number(n) => result.push(Token::Number(n.to_string())),
                        Value::String(s) => result.push(Token::StringLiteral(s)),
                        Value::Struct(sv) => result.push(Token::Struct(sv)),
                    }
                    i = j;
                } else {
                    result.push(tokens[i].clone());
                    i += 1;
                }
            } else if tokens[i] == Token::LeftParen {
                // Find matching right paren
                let mut depth = 1;
                let mut j = i + 1;
                while j < tokens.len() && depth > 0 {
                    if tokens[j] == Token::LeftParen {
                        depth += 1;
                    } else if tokens[j] == Token::RightParen {
                        depth -= 1;
                    }
                    j += 1;
                }

                // Recursively evaluate the expression inside parentheses
                let inner = &tokens[i + 1..j - 1];
                let inner_processed = self.process_functions_and_parentheses(inner);
                let value = self.evaluate_expr(&inner_processed);
                match value {
                    Value::Number(n) => result.push(Token::Number(n.to_string())),
                    Value::String(s) => result.push(Token::StringLiteral(s)),
                    Value::Struct(sv) => result.push(Token::Struct(sv)),
                }
                i = j;
            } else {
                result.push(tokens[i].clone());
                i += 1;
            }
        }

        result
    }

    fn evaluate_function(&mut self, func_name: &str, arg: f64) -> f64 {
        match func_name {
            "ABS" => arg.abs(),
            "ATN" => arg.atan(),
            "COS" => arg.cos(),
            "EXP" => arg.exp(),
            "INT" => arg.trunc(),
            "FIX" => {
                // FIX truncates towards zero (same as INT for positive, different for negative)
                arg.trunc()
            }
            "CINT" => {
                // CINT rounds to nearest integer
                arg.round()
            }
            "LOG" => {
                if arg <= 0.0 {
                    eprintln!("Error: LOG of non-positive number ({})", arg);
                    0.0
                } else {
                    arg.ln()
                }
            }
            "RND" => {
                // Ignore the argument as per BASIC spec
                self.rng.gen::<f64>()
            }
            "SIN" => arg.sin(),
            "SQR" => {
                if arg < 0.0 {
                    eprintln!(
                        "Error: SQR of negative number ({}), using absolute value",
                        arg
                    );
                    arg.abs().sqrt()
                } else {
                    arg.sqrt()
                }
            }
            "TAN" => arg.tan(),
            "TAB" => {
                // TAB(n) should return spaces, but we can't track column position here
                // For now, just return the position as a marker - will be handled specially in PRINT
                arg
            }
            "SPC" => {
                // SPC(n) returns n spaces - handled specially in PRINT
                arg
            }
            "EOF" => {
                // EOF(file_num) - check if end of file reached
                let file_num = arg as i32;
                if self.file_handles.contains_key(&file_num) {
                    0.0 // File exists and not at EOF (simplified for now)
                } else {
                    -1.0 // File not open
                }
            }
            "SGN" => {
                // Return sign of number: -1 for negative, 0 for zero, 1 for positive
                if arg < 0.0 {
                    -1.0
                } else if arg > 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            _ => {
                eprintln!("Error: Unknown function {}", func_name);
                0.0
            }
        }
    }

    fn evaluate_string_function(&mut self, func_name: &str, args_tokens: &[Token]) -> Value {
        // Split arguments by commas
        let mut args = Vec::new();
        let mut current_arg = Vec::new();
        let mut paren_depth = 0;

        for token in args_tokens {
            match token {
                Token::LeftParen => {
                    paren_depth += 1;
                    current_arg.push(token.clone());
                }
                Token::RightParen => {
                    paren_depth -= 1;
                    current_arg.push(token.clone());
                }
                Token::Comma if paren_depth == 0 => {
                    if !current_arg.is_empty() {
                        args.push(current_arg.clone());
                        current_arg.clear();
                    }
                }
                _ => {
                    current_arg.push(token.clone());
                }
            }
        }
        if !current_arg.is_empty() {
            args.push(current_arg);
        }
        match func_name {
            "LEN" => {
                if args.is_empty() {
                    return Value::Number(0.0);
                }
                let val = self.evaluate_expr(&args[0]);
                match val {
                    Value::String(s) => Value::Number(s.len() as f64),
                    Value::Number(n) => Value::Number(n.to_string().len() as f64),
                    Value::Struct(_) => Value::Number(0.0),
                }
            }
            "LEFT$" => {
                if args.len() < 2 {
                    return Value::String(String::new());
                }
                let string_val = self.evaluate_expr(&args[0]);
                let count_val = self.evaluate_expr(&args[1]);

                let s = match string_val {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    Value::Struct(_) => String::new(),
                };
                let count = match count_val {
                    Value::Number(n) => n as usize,
                    _ => 0,
                };

                Value::String(s.chars().take(count).collect())
            }
            "RIGHT$" => {
                if args.len() < 2 {
                    return Value::String(String::new());
                }
                let string_val = self.evaluate_expr(&args[0]);
                let count_val = self.evaluate_expr(&args[1]);

                let s = match string_val {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    Value::Struct(_) => String::new(),
                };
                let count = match count_val {
                    Value::Number(n) => n as usize,
                    _ => 0,
                };

                let chars: Vec<char> = s.chars().collect();
                let start = if count > chars.len() {
                    0
                } else {
                    chars.len() - count
                };
                Value::String(chars[start..].iter().collect())
            }
            "MID$" => {
                if args.len() < 2 {
                    return Value::String(String::new());
                }
                let string_val = self.evaluate_expr(&args[0]);
                let start_val = self.evaluate_expr(&args[1]);

                let s = match string_val {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    Value::Struct(_) => String::new(),
                };
                let start = match start_val {
                    Value::Number(n) => (n as usize).saturating_sub(1), // BASIC is 1-indexed
                    _ => 0,
                };

                if args.len() >= 3 {
                    let length_val = self.evaluate_expr(&args[2]);
                    let length = match length_val {
                        Value::Number(n) => n as usize,
                        _ => s.len(),
                    };
                    let chars: Vec<char> = s.chars().collect();
                    let end = (start + length).min(chars.len());
                    Value::String(chars[start..end].iter().collect())
                } else {
                    // No length specified, take rest of string
                    let chars: Vec<char> = s.chars().collect();
                    Value::String(chars[start..].iter().collect())
                }
            }
            "UCASE$" => {
                if args.is_empty() {
                    return Value::String(String::new());
                }
                let val = self.evaluate_expr(&args[0]);
                match val {
                    Value::String(s) => Value::String(s.to_uppercase()),
                    Value::Number(n) => Value::String(n.to_string().to_uppercase()),
                    Value::Struct(_) => Value::String(String::new()),
                }
            }
            "LCASE$" => {
                if args.is_empty() {
                    return Value::String(String::new());
                }
                let val = self.evaluate_expr(&args[0]);
                match val {
                    Value::String(s) => Value::String(s.to_lowercase()),
                    Value::Number(n) => Value::String(n.to_string().to_lowercase()),
                    Value::Struct(_) => Value::String(String::new()),
                }
            }
            "INSTR$" => {
                if args.len() < 2 {
                    return Value::Number(0.0);
                }
                let haystack_val = self.evaluate_expr(&args[0]);
                let needle_val = self.evaluate_expr(&args[1]);

                let haystack = match haystack_val {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    Value::Struct(_) => String::new(),
                };
                let needle = match needle_val {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    Value::Struct(_) => String::new(),
                };

                if let Some(pos) = haystack.find(&needle) {
                    // Convert to 1-indexed position
                    Value::Number((pos + 1) as f64)
                } else {
                    Value::Number(0.0)
                }
            }
            "CHR$" => {
                if args.is_empty() {
                    return Value::String(String::new());
                }
                let val = self.evaluate_expr(&args[0]);
                let code = match val {
                    Value::Number(n) => n as u8,
                    Value::String(s) => {
                        if let Ok(n) = s.parse::<f64>() {
                            n as u8
                        } else {
                            0
                        }
                    }
                    Value::Struct(_) => 0,
                };
                Value::String((code as char).to_string())
            }
            "ASC" => {
                if args.is_empty() {
                    return Value::Number(0.0);
                }
                let val = self.evaluate_expr(&args[0]);
                let s = match val {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    Value::Struct(_) => String::new(),
                };
                // Get first character's ASCII code
                if let Some(first_char) = s.chars().next() {
                    Value::Number(first_char as u32 as f64)
                } else {
                    Value::Number(0.0)
                }
            }
            "STR$" => {
                if args.is_empty() {
                    return Value::String(String::new());
                }
                let val = self.evaluate_expr(&args[0]);
                match val {
                    Value::Number(n) => {
                        // Add leading space for positive numbers (classic BASIC behavior)
                        if n >= 0.0 {
                            Value::String(format!(" {}", n))
                        } else {
                            Value::String(n.to_string())
                        }
                    }
                    Value::String(s) => Value::String(s),
                    Value::Struct(_) => Value::String(String::new()),
                }
            }
            "VAL" => {
                // Convert string to number - returns a Number value
                if args.is_empty() {
                    return Value::Number(0.0);
                }
                let val = self.evaluate_expr(&args[0]);
                match val {
                    Value::String(s) => Value::Number(s.trim().parse::<f64>().unwrap_or(0.0)),
                    Value::Number(n) => Value::Number(n),
                    Value::Struct(_) => Value::Number(0.0),
                }
            }
            "HEX$" => {
                // Convert number to hexadecimal string
                if args.is_empty() {
                    return Value::String(String::new());
                }
                let val = self.evaluate_expr(&args[0]);
                match val {
                    Value::Number(n) => {
                        let int_val = n.trunc() as i64;
                        Value::String(format!("{:X}", int_val))
                    }
                    Value::String(_) | Value::Struct(_) => Value::String(String::new()),
                }
            }
            "OCT$" => {
                // Convert number to octal string
                if args.is_empty() {
                    return Value::String(String::new());
                }
                let val = self.evaluate_expr(&args[0]);
                match val {
                    Value::Number(n) => {
                        let int_val = n.trunc() as i64;
                        Value::String(format!("{:o}", int_val))
                    }
                    Value::String(_) | Value::Struct(_) => Value::String(String::new()),
                }
            }
            "SPACE$" => {
                // Generate n spaces
                if args.is_empty() {
                    return Value::String(String::new());
                }
                let val = self.evaluate_expr(&args[0]);
                let count = match val {
                    Value::Number(n) => n.max(0.0) as usize,
                    _ => 0,
                };
                Value::String(" ".repeat(count))
            }
            "STRING$" => {
                // Repeat a character n times: STRING$(n, char_code) or STRING$(n, "X")
                if args.len() < 2 {
                    return Value::String(String::new());
                }
                let count_val = self.evaluate_expr(&args[0]);
                let count = match count_val {
                    Value::Number(n) => n.max(0.0) as usize,
                    _ => 0,
                };

                let char_val = self.evaluate_expr(&args[1]);
                let ch = match char_val {
                    Value::Number(n) => n as u8 as char,
                    Value::String(s) => s.chars().next().unwrap_or(' '),
                    Value::Struct(_) => ' ',
                };

                Value::String(ch.to_string().repeat(count))
            }
            "LTRIM$" => {
                // Remove leading whitespace
                if args.is_empty() {
                    return Value::String(String::new());
                }
                let val = self.evaluate_expr(&args[0]);
                match val {
                    Value::String(s) => Value::String(s.trim_start().to_string()),
                    Value::Number(n) => Value::String(n.to_string().trim_start().to_string()),
                    Value::Struct(_) => Value::String(String::new()),
                }
            }
            "RTRIM$" => {
                // Remove trailing whitespace
                if args.is_empty() {
                    return Value::String(String::new());
                }
                let val = self.evaluate_expr(&args[0]);
                match val {
                    Value::String(s) => Value::String(s.trim_end().to_string()),
                    Value::Number(n) => Value::String(n.to_string().trim_end().to_string()),
                    Value::Struct(_) => Value::String(String::new()),
                }
            }
            "TRIM$" => {
                // Remove leading and trailing whitespace
                if args.is_empty() {
                    return Value::String(String::new());
                }
                let val = self.evaluate_expr(&args[0]);
                match val {
                    Value::String(s) => Value::String(s.trim().to_string()),
                    Value::Number(n) => Value::String(n.to_string().trim().to_string()),
                    Value::Struct(_) => Value::String(String::new()),
                }
            }
            "INKEY$" => {
                let mut keypressed: Value = Value::String(String::from(""));
                if let Ok(true) = crossterm::event::poll(Duration::from_millis(0)) {
                    if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                        if key.code == KeyCode::Char('c')
                            && key
                                .modifiers
                                .contains(crossterm::event::KeyModifiers::CONTROL)
                        {
                            // Handle Ctrl+C
                            println!("^C detected, exiting.");
                            std::process::exit(0);
                        } else {
                            keypressed = match key.code {
                                KeyCode::Esc => Value::String("\x1b".to_string()),
                                KeyCode::Enter => Value::String("\r".to_string()),
                                KeyCode::Backspace => Value::String("\x08".to_string()),
                                KeyCode::Tab => Value::String("\t".to_string()),
                                KeyCode::Char(c) => Value::String(c.to_string()),
                                _ => Value::String(String::from("")),
                            };
                        }
                    }
                }

                keypressed
            }
            _ => {
                eprintln!("Error: Unknown string function {}", func_name);
                Value::String(String::new())
            }
        }
    }

    fn get_numeric_value(&mut self, token: &Token) -> f64 {
        match token {
            Token::Number(n) => n.parse().unwrap_or(0.0),
            Token::Identifier(id) => {
                // Special case: RND without parentheses
                if id == "RND" {
                    return self.rng.gen::<f64>();
                }
                if let Some(Value::Number(n)) = self.variables.get(id) {
                    *n
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }

    fn parse_subscripts(&mut self, tokens: &[Token]) -> Vec<usize> {
        let mut subscripts = Vec::new();
        let mut current = Vec::new();

        for token in tokens {
            if token == &Token::Comma {
                if !current.is_empty() {
                    let value = self.evaluate_expr(&current);
                    if let Value::Number(n) = value {
                        subscripts.push(n as usize);
                    }
                    current.clear();
                }
            } else {
                current.push(token.clone());
            }
        }

        // Handle last subscript
        if !current.is_empty() {
            let value = self.evaluate_expr(&current);
            if let Value::Number(n) = value {
                subscripts.push(n as usize);
            }
        }

        subscripts
    }

    fn get_array_element(&mut self, name: &str, subscripts: &[usize]) -> Value {
        // Auto-initialize if needed with default dimensions (base to 10 for each dimension)
        if !self.arrays.contains_key(name) {
            let lower_bounds = vec![self.option_base; subscripts.len()];
            let upper_bounds = vec![10; subscripts.len()];
            let array = Array::new(lower_bounds, upper_bounds);
            self.arrays.insert(name.to_string(), array);
        }

        if let Some(array) = self.arrays.get(name) {
            // Struct-backed array: forward field reads to dyncall
            if let Some(sv) = &array.struct_val {
                if subscripts.len() == 1 {
                    return match sv.script_read(subscripts[0]) {
                        Ok(dyncall::ScriptVal::Number(n)) => Value::Number(n),
                        Ok(dyncall::ScriptVal::Str(s)) => Value::String(s),
                        Err(_) => {
                            eprintln!("Error: struct field {} out of bounds or unreadable", subscripts[0]);
                            Value::Number(0.0)
                        }
                    };
                }
                eprintln!("Error: struct arrays require a single subscript");
                return Value::Number(0.0);
            }
            if let Some(val) = array.get(subscripts) {
                return val.clone();
            } else {
                eprintln!("Error: Array subscript out of bounds or dimension mismatch");
            }
        }
        Value::Number(0.0)
    }

    fn set_array_element(&mut self, name: &str, subscripts: &[usize], value: Value) {
        // Auto-initialize if needed with default dimensions (base to 10 for each dimension)
        if !self.arrays.contains_key(name) {
            let lower_bounds = vec![self.option_base; subscripts.len()];
            let upper_bounds = vec![10; subscripts.len()];
            let array = Array::new(lower_bounds, upper_bounds);
            self.arrays.insert(name.to_string(), array);
        }

        if let Some(array) = self.arrays.get_mut(name) {
            if !array.set(subscripts, value) {
                eprintln!("Error: Array subscript out of bounds or dimension mismatch");
            }
        }
    }
}

struct Interactive {
    interpreter: Interpreter,
}

impl Interactive {
    fn new() -> Self {
        Interactive {
            interpreter: Interpreter::new(),
        }
    }

    fn run(&mut self) {
        println!("BASIC Interpreter - Interactive Mode");
        println!("Type 'EXIT' to quit");
        println!("Use line numbers (e.g., '10 PRINT X') to store code");
        println!("Type 'RUN' to execute stored code");
        println!("Type 'LIST' to view stored code\n");

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let line = input.trim();

            if line.to_uppercase() == "EXIT" {
                break;
            }

            if line.is_empty() {
                continue;
            }

            if line.to_uppercase() == "RUN" {
                self.interpreter.run();
                continue;
            }

            if line.to_uppercase() == "LIST" {
                self.interpreter.list();
                continue;
            }

            let tokens = tokenize(line);

            // Check if line starts with a number
            if let Some(Token::Number(line_num)) = tokens.first() {
                let line_num: i32 = line_num.parse().unwrap_or(0);
                let stmt_tokens = tokens[1..].to_vec();
                let statements = parse(&stmt_tokens);

                if statements.is_empty() {
                    // Empty line - delete the line number
                    self.interpreter.delete_line(line_num);
                } else if let Some(stmt) = statements.first() {
                    self.interpreter.store_line(line_num, stmt);
                }
            } else {
                // Immediate mode - execute right away
                let statements = parse(&tokens);

                if statements.is_empty() {
                    println!("Error: Invalid statement");
                    continue;
                }

                self.interpreter.execute_immediate(&statements);
            }
        }

        println!("Goodbye!");
    }
}

