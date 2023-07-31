use crate::lexer::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Ast {
    Load(String, String),
    Bpm(f32),

    Play(String, f32),
    PlayPattern(String),
    Newline,
}

pub type Patterns = HashMap<String, Vec<Ast>>;

fn parse(tokens: &Vec<Token>) -> Result<Vec<Ast>, Box<dyn std::error::Error>> {
    let mut ast: Vec<Ast> = Vec::new();
    let mut index = 0;

    while index < tokens.len() {
        match &tokens[index] {
            Token::Newline => {
                ast.push(Ast::Newline);
            },
            Token::Bpm => {
                index += 1;
                if let Token::Int(bpm) = &tokens[index] {
                    ast.push(Ast::Bpm(*bpm));
                } else {
                    return Err(format!("expected integer but got: {:?}", tokens[index]).into());
                }
            },
            Token::Load => {
                index += 1;
                if let (Token::Ident(name), Token::Ident(path)) = (&tokens[index], &tokens[index + 1]) {
                    index += 1;
                    ast.push(Ast::Load(name.clone(), path.clone()));
                } else {
                    return Err(format!("expected ident and path but got: {:?} {:?}", tokens[index - 1], tokens[index]).into());
                }
            },
            Token::Ident(ident) => {
                if index + 1 != tokens.len() {
                    if let Token::Int(tone) = &tokens[index + 1] {
                        index += 1;
                        ast.push(Ast::Play(ident.clone(), *tone));
                    } else {
                        ast.push(Ast::PlayPattern(ident.clone()));
                    }
                }
            },
            _ => {},
        }
        index += 1;
    }

    return Ok(ast);
}

pub fn make(tokens: &Vec<Token>) -> Result<Patterns, Box<dyn std::error::Error>> {
    let mut patterns: HashMap<String, Vec<Ast>> = HashMap::new();
    let mut index = 0;

    while index < tokens.len() {
        match &tokens[index] {
            Token::Define => {
                index += 1;
                let name = match &tokens[index] {
                    Token::Ident(ident) => ident.clone(),
                    _ => {
                        return Err(format!("expected ident but got: {:?}", tokens[index]).into());
                    },
                };
                index += 1;
                let mut body: Vec<Token> = Vec::new();
                'body_loop: loop {
                    match &tokens[index] {
                        Token::End => {
                            patterns.insert(name, parse(&body)?);
                            break 'body_loop;
                        },
                        _ => {
                            body.push(tokens[index].clone());
                        },
                    }
                    if index + 1 == tokens.len() {
                        return Err(format!("expected end but got: {:?}", tokens[index]).into());
                    }
                    index += 1;
                }
            },
            _ => {},
        }
        index += 1;
    }

    return Ok(patterns);
}

