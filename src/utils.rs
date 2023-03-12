use std::str::Chars;

pub trait GetWhile {
    fn get_while<F>(&mut self, f: F) -> String
    where
        F: Fn(char) -> bool;
}

impl<'a> GetWhile for Chars<'a> {
    fn get_while<F>(&mut self, f: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();

        while let Some(char) = self.next() {
            if f(char) {
                result.push(char);
            } else {
                break;
            }
        }

        result
    }
}