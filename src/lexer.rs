#[derive(Debug)]
pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn slice(&mut self, n: usize) -> &'a [char] {
        if n > self.content.len() {
            return &['\0'];
        }
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        return token;
    }

    fn slice_while<P: FnMut(&char) -> bool>(&mut self, mut predicate: P) -> &'a [char] {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            if self.content[n] == '\\' {
                n += 1;
            }
            n += 1;
        }
        return self.slice(n);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.trim_left();

        if self.content.len() == 0 {
            return None;
        }

        match self.content[0] {
            '"' => {
                self.slice(1);
                let slice = self.slice_while(|x| x != &'"');
                self.slice(1);
                return Some(slice);
            }
            '\'' => {
                self.slice(1);
                let slice = self.slice_while(|x| x != &'\'');
                self.slice(1);
                return Some(slice);
            }
            _ => (),
        }

        if self.content[0].is_numeric() {
            return Some(self.slice_while(|x| x.is_numeric()));
        }

        if self.content[0].is_alphabetic() {
            return Some(self.slice_while(|x| x.is_alphanumeric()));
        }

        Some(self.slice(1))
    }
}

/*
fn main() {
    let command = String::from(r#"echo "Hello, \"something here\" World"\\"#)
        .chars()
        .collect::<Vec<_>>();

    let lexer = Lexer::new(&command);
    for token in lexer.into_iter() {
        println!("{}", token.iter().collect::<String>())
    }
}
*/
