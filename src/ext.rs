pub trait StrExt {
    fn rsplit_at(&self, i: usize) -> (&str, &str);
    fn rsplit_at_mut(&mut self, i: usize) -> (&mut str, &mut str);
}

impl StrExt for str {
    fn rsplit_at(&self, i: usize) -> (&str, &str) {
        self.split_at(self.len() - i)
    }

    fn rsplit_at_mut(&mut self, i: usize) -> (&mut str, &mut str) {
        self.split_at_mut(self.len() - i)
    }
}

#[cfg(test)]
mod test {
    use super::StrExt;

    #[test]
    fn rsplit_at() {
        let expects = [("abc", ""), ("ab", "c"), ("a", "bc"), ("", "abc")];

        for i in 0..3 {
            let got = "abc".rsplit_at(i);
            let expect = expects[i];

            assert_eq!(got, expect)
        }
    }

    #[test]
    fn rsplit_at_mut() {
        let expects = [("abc", ""), ("ab", "c"), ("a", "bc"), ("", "abc")];
        let mut raw = "abc".to_string();

        for i in 0..3 {
            let got = {
                let (a_mut, b_mut) = raw.rsplit_at_mut(i);
                (&*a_mut, &*b_mut)
            };
            let expect = expects[i];

            assert_eq!(got, expect)
        }
    }
}
