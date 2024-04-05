

pub trait StringStrategy: Clone + Default {
    fn new() -> Self;
    fn prepare(&self, s: &str) -> String;
    fn len(&self, s: &str) -> usize;
    fn remove(&self, s: &str, index: usize) -> String;
    fn slice(&self, s: &str, start: usize, end: usize) -> String;
    fn suffix(&self, s: &str, start: usize) -> String;
    fn at(&self, s: &str, i: isize) -> Option<char>;
}

#[derive(Clone)]
pub struct UnicodeStringStrategy {}

impl Default for UnicodeStringStrategy {
    fn default() -> UnicodeStringStrategy {
        UnicodeStringStrategy {}
    }
}

impl StringStrategy for UnicodeStringStrategy {
    fn new() -> Self {
        Self {}
    }

    fn prepare(&self, s: &str) -> String {
        s.to_string()
    }

    fn len(&self, s: &str) -> usize {
        s.chars().count()
    }

    fn remove(&self, s: &str, index: usize) -> String {
        s.chars()
            .enumerate()
            .filter(|(ii, _)| ii != &index)
            .map(|(_, ch)| ch)
            .collect()
    }

    fn slice(&self, s: &str, start: usize, end: usize) -> String {
        s.chars().skip(start).take(end - start).collect()
    }

    fn suffix(&self, s: &str, start: usize) -> String {
        s.chars().skip(start).collect::<String>()
    }

    fn at(&self, s: &str, i: isize) -> Option<char> {
        if i < 0 || i >= s.len() as isize {
            return None;
        }

        s.chars().nth(i as usize)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicodei_strategy() {
        assert_eq!(UnicodeStringStrategy::new().prepare("ciccio"), "ciccio");
    }
}
