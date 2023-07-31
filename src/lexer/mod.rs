use std::fs;

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Int(f32),

    Bpm,
    Load,
    Define,
    End,
    Newline,
}


pub fn lexer(filename: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let mut tokens: Vec<Token> = Vec::new();
    let source = fs::read_to_string(filename)?;

    let mut token = String::new();
    let mut index = 0;
    let bytes = source.bytes().collect::<Vec<u8>>();
    while index < bytes.len() {
        let character = &String::from_utf8(vec![bytes[index]])?;
        match token.as_str() {
            "bpm" => { tokens.push(Token::Bpm); token = String::new(); },
            "load" => { tokens.push(Token::Load); token = String::new(); },
            "define" => { tokens.push(Token::Define); token = String::new(); },
            "end" => { tokens.push(Token::End); token = String::new(); },
            _ => {
                if character == " " || character == "\n" {
                    if let Ok(integer) = token.parse::<f32>() {
                        tokens.push(Token::Int(integer));
                    } else {
                        if token != "" {
                            tokens.push(Token::Ident(token.clone()));
                        }
                        if character == "\n" {
                            tokens.push(Token::Newline);
                        }
                    }
                    token = String::new();
                    index += 1;
                    continue;
                }
            },
        }
        if character != " " && character != "\n" {
            token = token + character;
        }
        index += 1;
    }

    return Ok(tokens);
}


