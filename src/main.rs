struct Lexer<'a> {
    char_view: &'a str,
}

struct Token<'a> {
    char_view: &'a str,
}

impl<'a> Token<'a> {
    fn new(char_view: &'a str) -> Self {
        Token { char_view }
    }

    fn dump(&self) {
        let view = self.char_view;
        println!("{view}");
    }
}

impl<'a> Lexer<'a> {
    fn new(char_view: &'a str) -> Self {
        Lexer { char_view }
    }

    // maybe curry the predicate
    fn next(
        &mut self,
        predicate_begin: fn(char) -> bool,
        predicate_end: fn(char) -> bool,
    ) -> Option<Token<'a>> {
        if self.char_view.len() == 0 {
            return None;
        }

        let mut begin: Option<usize> = None;
        // find begin
        for (i, c) in self.char_view.chars().enumerate() {
            match predicate_begin(c) {
                false => continue,
                _ => {
                    begin = Some(i);
                    break;
                }
            }
        }

        let Some(begin) = begin else {
            return None;
        };

        let mut end: Option<usize> = None;
        // find end
        for (i, c) in self.char_view[begin..].chars().enumerate() {
            match predicate_end(c) {
                false => continue,
                _ => {
                    end = Some(i + begin);
                    break;
                }
            }
        }

        let end = end.unwrap_or(self.char_view.len());

        let ret = Some(Token::new(&self.char_view[begin..end]));
        self.char_view = &self.char_view[end..];
        ret
    }

    fn dump(&self) {
        let view = &self.char_view;
        println!("{view}");
    }
}

fn predicate_begin(c: char) -> bool {
    c != ' '
}
fn predicate_end(c: char) -> bool {
    c == ' '
}

fn main() {
    let mem_owner: String = "I  need to be lexed!".to_string();
    let mut lexer = Lexer::new(&mem_owner.as_str());
    lexer.dump();

    while let Some(first_token) = lexer.next(predicate_begin, predicate_end) {
        first_token.dump();
    }

    // should be empty
    lexer.dump();
}
