// main.rs
use actix_web::{App, HttpServer, Responder, web, HttpResponse, get, post};
use actix_files as fs;
use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
enum EmojiToken {
    Number(f64),
    Add,
    Subtract,
    Times,
    Division,
    If,
    Then,
    Else,
    EndIf,
    Variable(char),
    Assign,
    LoopStart,
    LoopEnd,
    LoopRange(f64, f64),
    FunctionStart(char),    // New variant for function start, with the function name as a char
    FunctionEnd,            // New variant for function end
    FunctionCall(char),
}

struct PeekableChars<'a> {
    chars: Chars<'a>,
    peeked: Option<char>,
}

impl<'a> PeekableChars<'a> {
    fn new(s: &'a str) -> Self {
        PeekableChars {
            chars: s.chars(),
            peeked: None,
        }
    }

    fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.peeked.take() {
            Some(ch)
        } else {
            self.chars.next()
        }
    }

    fn peek(&mut self) -> Option<&char> {
        if self.peeked.is_none() {
            self.peeked = self.chars.next();
        }
        self.peeked.as_ref()
    }
}

trait Tokenize {
    fn tokenize(&mut self) -> Vec<EmojiToken>;
    fn parse_number(&mut self) -> Option<EmojiToken>;
}

impl Tokenize for PeekableChars<'_> {
    fn parse_number(&mut self) -> Option<EmojiToken> {
        if let Some(ch) = self.peek() {
            match *ch {
                '0'..='9' | '🗨' | '.' => {
                    let mut number = String::new();
                    let mut is_decimal = ch == &'🗨';
                    if !is_decimal {
                        number.push(self.next().unwrap());
                    }

                    while let Some(next_ch) = self.peek() {
                        match *next_ch {
                            '0'..='9' | '.' => {
                                number.push(self.next().unwrap());
                            }
                            '🗨' => {
                                is_decimal = !is_decimal;
                                self.next().unwrap();
                            }
                            _ => break,
                        }
                    }

                    if is_decimal {
                        number.insert(0, '0');
                    }

                    Some(EmojiToken::Number(number.parse().unwrap()))
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn tokenize(&mut self) -> Vec<EmojiToken> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.next() {
            let token = match ch {
                '🙂' => EmojiToken::Add,
                '😢' => EmojiToken::Subtract,
                '😂' => EmojiToken::Times,
                '🤔' => EmojiToken::Division,
                '😕' => EmojiToken::If,
                '😄' => EmojiToken::Then,
                '😞' => EmojiToken::Else,
                '🔤' => EmojiToken::Assign,
                '🔁' => EmojiToken::LoopStart,
                '🔚' => EmojiToken::LoopEnd,
                '🏁' => EmojiToken::FunctionStart(self.next().unwrap()), // Assume the next char is the function name
                '🚩' => EmojiToken::FunctionEnd,
                '📞' => EmojiToken::FunctionCall(self.next().unwrap()), // Assume the next char is the function name
                // other cases...
                '[' => {
                    let start_num = self.parse_number();
                    if let Some(EmojiToken::Number(start)) = start_num {
                        if let Some('🔁') = self.next() {
                            let end_num = self.parse_number();
                            if let Some(EmojiToken::Number(end)) = end_num {
                                if let Some(']') = self.next() {
                                    EmojiToken::LoopRange(start, end)
                                } else {
                                    panic!("Missing closing ']' for loop range");
                                }
                            } else {
                                panic!("Invalid end number in loop range");
                            }
                        } else {
                            panic!("Missing '🔁' in loop range");
                        }
                    } else {
                        panic!("Invalid start number in loop range");
                    }
                }
                '0'..='9' | '🗨' | '.' => {
                    let mut number = String::new();
                    let mut is_decimal = ch == '🗨';
                    if !is_decimal {
                        number.push(ch);
                    }

                    while let Some(next_ch) = self.peek() {
                        match *next_ch {
                            '0'..='9' | '.' => {
                                number.push(self.next().unwrap());
                            }
                            '🗨' => {
                                is_decimal = !is_decimal;
                                self.next().unwrap();
                            }
                            _ => break,
                        }
                    }

                    if is_decimal {
                        number.insert(0, '0');
                    }

                    EmojiToken::Number(number.parse().unwrap())
                }
                _ => EmojiToken::Variable(ch),
            };

            tokens.push(token);
        }

        tokens
    }
}

// main.rs (continued)
struct EmojiInterpreter {
    tokens: Vec<EmojiToken>,
    position: usize,
    variables: HashMap<char, f64>,
    functions: HashMap<char, Vec<EmojiToken>>,
}

impl EmojiInterpreter {
    fn new(tokens: Vec<EmojiToken>) -> Self {
        EmojiInterpreter {
            tokens,
            position: 0,
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn next_token(&mut self) -> Option<(usize, &EmojiToken)> {
        if self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            let current_position = self.position;
            self.position += 1;
            Some((current_position, token))
        } else {
            None
        }
    }

    fn evaluate(&mut self) -> f64 {
        let mut result = 0.0;
        let mut operation: fn(f64, f64) -> f64 = |a, b| a + b;
        let mut if_condition = false;
        let mut if_block = false;
        let mut else_block = false;
        let mut assign_variable = None;
        let mut new_assignment = None;
        let mut loop_start = None;
        let mut loop_count = 0;

        let mut in_loop = false;

        let mut function_tokens: Option<Vec<EmojiToken>> = None;
        let mut function_name: Option<char> = None;
        let mut function_call = None;  // new variable to store the function name

        while let Some((current_position, token)) = self.next_token() {
            if let Some(name) = function_name {
                if let EmojiToken::FunctionEnd = token {
                    self.functions.insert(name, function_tokens.clone().unwrap());
                    function_name = None;
                    continue;
                }
                function_tokens.as_mut().unwrap().push(token.clone());
                continue;
            }
            match token {
                EmojiToken::Number(num) => {
                    if let Some(var) = assign_variable {
                        if (if_block && !if_condition) || (else_block && if_condition) {
                            continue;
                        }
                        new_assignment = Some((var, *num));
                        assign_variable = None;
                    } else {
                        if (if_block && !if_condition) || (else_block && if_condition) {
                            continue;
                        }
                        result = operation(result, *num);
                    }
                }
                EmojiToken::Add => {
                    operation = |a, b| a + b;
                }
                EmojiToken::Subtract => {
                    operation = |a, b| a - b;
                }
                EmojiToken::Times => {
                    operation = |a, b| a * b;
                }
                EmojiToken::Division => {
                    operation = |a, b| a / b;
                }
                EmojiToken::If => {
                    if_condition = result != 0.0;
                    if_block = true;
                }
                EmojiToken::Then => {
                    if !if_condition {
                        if_block = false;
                        else_block = true;
                    }
                }
                EmojiToken::Else => {
                    if_block = !if_block;
                    else_block = !else_block;
                }
                EmojiToken::EndIf => {
                    if_block = false;
                    else_block = false;
                }
                EmojiToken::Variable(ch) => {
                    if let Some(var) = assign_variable {
                        if (if_block && !if_condition) || (else_block && if_condition) {
                            continue;
                        }
                        if let Some(value) = self.variables.get(&var) {
                            result = operation(result, *value);
                        }
                        assign_variable = None;
                    } else {
                        assign_variable = Some(*ch);
                    }
                }
                EmojiToken::Assign => {
                    if assign_variable.is_none() {
                        continue;
                    }
                }
                EmojiToken::LoopStart => {
                    if loop_start.is_none() {
                        loop_start = Some(current_position);
                    }
                    if result != 0.0 {
                        loop_count = result as usize;
                        result = 0.0;
                    }
                    in_loop = true;
                }
                EmojiToken::LoopRange(start, end) => {
                    if !in_loop {
                        continue;
                    }
                    if loop_start.is_none() {
                        loop_start = Some(current_position);
                    }
                    if loop_count == 0 {
                        result = *start;
                        loop_count = ((*end - *start) as usize) + 1;
                    } else {
                        loop_count = 0;
                        loop_start = None;
                        in_loop = false;
                    }
                }
                EmojiToken::LoopEnd => {
                    println!("Result at loop iterations {}", loop_count);
                    if loop_count > 0 {
                        loop_count -= 1;
                        if let Some(start) = loop_start {
                            self.position = start;
                        }
                    } else {
                        loop_start = None;
                        in_loop = false;
                    }
                }

                EmojiToken::FunctionStart(name) => {
                    function_name = Some(*name);
                    function_tokens = Some(Vec::new());
                    continue;
                }
                EmojiToken::FunctionCall(name) => {
                    function_call = Some(*name);
                    continue;
                }
                _ => {}
            }
        }
        if let Some((var, value)) = new_assignment {
            self.variables.insert(var, value);
        }
        // Handle the function call
        if let Some(name) = function_call {
            // Look up the function and execute it
            let function_tokens = self.functions.get(&name)
                .expect(&format!("Undefined function '{}'", name))
                .clone();

            let mut function_interpreter = EmojiInterpreter {
                tokens: function_tokens,
                position: 0,
                variables: self.variables.clone(),
                functions: self.functions.clone(),
            };
            result = function_interpreter.evaluate();
        }
        result
    }
}

#[derive(Serialize, Deserialize)]
pub struct Script {
    code: String,
}

#[post("/run")]
async fn run_code(script: web::Json<Script>) -> impl Responder {
    let script = script.into_inner().code;
    let mut peekable_chars = PeekableChars::new(&script);
    let tokens = peekable_chars.tokenize();

    let mut interpreter = EmojiInterpreter::new(tokens);
    let result = interpreter.evaluate();

    web::Json(result.to_string())
}

#[get("/")]
async fn index() -> impl Responder {
    let api_url = env::var("API_URL").expect("API_URL must be set");

    let mut file = std::fs::read_to_string("src/index.html").expect("Could not read index.html");
    file = file.replace("API_URL_PLACEHOLDER", &api_url);

    HttpResponse::Ok()
        .body(file)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(run_code)
            .service(fs::Files::new("/", "./").index_file("index.html"))  // Serve static files
    })
    .bind("0.0.0.0:443")?
    .run()
    .await
}

// main.rs (continued)
// fn main() {
//     let input = "2️⃣🗨.🗨0️⃣🗨🤔2️⃣";
//     let mut peekable_chars = PeekableChars::new(input);
//     let tokens = peekable_chars.tokenize();

//     let mut interpreter = EmojiInterpreter::new(tokens);
//     let result = interpreter.evaluate();
//     println!("Result: {}", result);

//     ////////////////////////////////////////////////////////////////////////
//     let input2 = "2️⃣🗨.🗨5️⃣🗨😕😄🙂1️⃣😞🙂2️⃣";
//     let mut peekable_chars2 = PeekableChars::new(input2);
//     let tokens2 = peekable_chars2.tokenize();

//     let mut interpreter2 = EmojiInterpreter::new(tokens2);
//     let result2 = interpreter2.evaluate();
//     println!("Result: {}", result2);

//     ////////////////////////////////////////////////////////////////////////
//     let input3 = "🔤2️⃣🙂2️⃣";
//     let mut peekable_chars3 = PeekableChars::new(input3);
//     let tokens3 = peekable_chars3.tokenize();

//     let mut interpreter3 = EmojiInterpreter::new(tokens3);
//     let result3 = interpreter3.evaluate();
//     println!("Result: {}", result3);

//     ////////////////////////////////////////////////////////////////////////
//     let input4 = "9️⃣🔁🔚";
//     let mut peekable_chars4 = PeekableChars::new(input4);
//     let tokens4 = peekable_chars4.tokenize();

//     let mut interpreter4 = EmojiInterpreter::new(tokens4);
//     let result4 = interpreter4.evaluate();
//     println!("Result: {}", result4);

//     ////////////////////////////////////////////////////////////////////////
//     let input5 = "[0🔁8]🔚";
//     let mut peekable_chars5 = PeekableChars::new(input5);
//     let tokens5 = peekable_chars5.tokenize();

//     let mut interpreter5 = EmojiInterpreter::new(tokens5);
//     let result5 = interpreter5.evaluate();
//     println!("Result: {}", result5);

//     ////////////////////////////////////////////////////////////////////////
//     let input6 = "🏁f2️⃣😂2️⃣🚩📞f";
//     let mut peekable_chars6 = PeekableChars::new(input6);
//     let tokens6 = peekable_chars6.tokenize();

//     let mut interpreter6 = EmojiInterpreter::new(tokens6);
//     let result6 = interpreter6.evaluate();
//     println!("Result: {}", result6);
// }

