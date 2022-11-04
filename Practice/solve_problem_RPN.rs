fn main() {
    let input_str = read_input();
    let original_token = tokenize(&input_str);
}

fn read_input() -> String {
    let mut input_user = String::new();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Can not read this line");
    input_user
}

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}
pub fn tokenize(input_str: &String) -> Vec<InfixToken> {
    let mut tokens: Vec<InfixToken> = Vec::new();
    for element in input_str.split_whitespace() {
        if element == "+" {
            tokens.push(InfixToken::Operator(Operator::Add));
        } else if element == "-" {
            tokens.push(InfixToken::Operator(Operator::Sub));
        } else if element == "*" {
            tokens.push(InfixToken::Operator(Operator::Mul));
        } else if element == "/" {
            tokens.push(InfixToken::Operator(Operator::Div));
        } else if element == "(" {
            tokens.push(InfixToken::LeftParen);
        } else if element == ")" {
            tokens.push(InfixToken::RightParen);
        } else if element == "\n" {
            continue;
        } else {
            tokens.push(InfixToken::Operand(element.parse::<isize>().unwrap()))
        }
    }
    tokens
}

pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    if check_valid(tokens) {
        let mut stack = Vec::new();
    }
}
