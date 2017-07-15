

pub struct CharReader<T: Iterator<Item = char>> {
    it: T,
    peeked: Option<(Option<char>, Option<char>, Option<char>)>,
}

impl<T: Iterator<Item = char>> CharReader<T> {
    pub fn new(it: T) -> CharReader<T> {
        CharReader { it, peeked: None }
    }


    pub fn entry(&mut self) -> (Option<char>, Option<char>, Option<char>, Option<char>) {
        match self.peeked {
            None => {
                let a = self.it.next();
                let b = self.it.next();
                let c = self.it.next();

                let d = self.it.next();
                self.peeked = Some((b, c, d));
                (a, b, c, d)
            }
            Some((a, b, c)) => {
                let d = self.it.next();
                self.peeked = Some((b, c, d));
                (a, b, c, d)
            }
        }
    }
}

impl<T: Iterator<Item = char>> Iterator for CharReader<T> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let (next, _, _, _) = self.entry();
        next
    }
}
