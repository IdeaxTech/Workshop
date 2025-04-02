use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // キーワード
    KeywordFn,
    KeywordLet,
    KeywordReturn,
    KeywordInt,

    // 名前（識別子）
    Identifier(String),

    // 数値
    UnsignedInteger(u64),

    // 文字列
    StringLiteral(String),

    // 記号
    Plus,       // +
    Minus,      // -
    Asterisk,   // *
    Slash,      // /
    Equal,      // =
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    Semicolon,  // ;
    Colon,      // :

    // ファイル終端
    EOF,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            position: 0,
        }
    }

    // 次の文字を見る（消費しない）
    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    // 次の文字を取得して進める
    fn next_char(&mut self) -> Option<char> {
        let ch = self.input.next();
        if ch.is_some() {
            self.position += 1;
        }
        ch
    }

    // 次のトークンを取得
    pub fn next_token(&mut self) -> Result<Token, String> {
        todo!(
            r#"トークンを取得するロジックを実装してください．
ヒント: self.next_char()を次の文字に進める．self.peek()で進まずに次の文字を確認する．
どんな文字だったかによって処理を分岐する．
        "#
        );
    }

    // すべてのトークンを取得
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            if token == Token::EOF {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        Ok(tokens)
    }
}

// テスト用のメイン関数
fn main() {
    let input = "fn main() { 
        let x = 42;
        /* これはコメントです */
        let message = 'Hello, world!'''; 
        return x + 10; 
    }";

    let mut lexer = Lexer::new(input);

    match lexer.tokenize() {
        Ok(tokens) => {
            println!("トークン一覧:");
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            println!("エラー: {}", e);
        }
    }
}
