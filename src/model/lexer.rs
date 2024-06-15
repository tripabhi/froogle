use crate::snowball;

pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    pub fn chop_while<P>(&mut self, predicate: P) -> &'a [char]
    where
        P: Fn(&char) -> bool,
    {
        let mut idx = 0;
        while idx < self.content.len() && predicate(&self.content[idx]) {
            idx += 1;
        }
        self.chop(idx)
    }

    pub fn chop(&mut self, idx: usize) -> &'a [char] {
        let token = &self.content[0..idx];
        self.content = &self.content[idx..];
        token
    }

    pub fn trim_left(&mut self) {
        let _ = self.chop_while(|c| c.is_whitespace());
    }

    pub fn next_token(&mut self) -> Option<String> {
        self.trim_left();
        if self.content.is_empty() {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(
                self.chop_while(|c| c.is_numeric() || c.eq(&'.'))
                    .iter()
                    .collect(),
            );
        }

        if self.content[0].is_alphabetic() {
            let token = self
                .chop_while(|c| c.is_alphanumeric())
                .iter()
                .map(|c| c.to_ascii_lowercase())
                .collect::<String>();

            // Word Stemming https://en.wikipedia.org/wiki/Stemming
            return Some(stem_token(&token));
        }
        let _ = self.chop_while(|c| !c.is_alphanumeric() && !c.is_whitespace());
        self.next_token()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[inline]
pub fn stem_token(token: &str) -> String {
    let mut s_env = snowball::SnowballEnv::create(token);
    snowball::algorithms::english_stemmer::stem(&mut s_env);

    s_env.get_current().to_ascii_lowercase().to_string()
}
