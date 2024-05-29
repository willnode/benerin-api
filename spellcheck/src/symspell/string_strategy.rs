use deepsize::DeepSizeOf;


#[derive(Clone, PartialEq, Eq, DeepSizeOf)]
pub struct UnicodeStringStrategy {}

impl Default for UnicodeStringStrategy {
    fn default() -> UnicodeStringStrategy {
        UnicodeStringStrategy {}
    }
}

impl UnicodeStringStrategy {
    pub fn new() -> Self {
        Self {}
    }

    pub fn prepare(&self, s: &str) -> String {
        s.to_string()
    }

    pub fn len(&self, s: &str) -> usize {
        s.chars().count()
    }

    pub fn remove(&self, s: &str, index: usize) -> String {
        s.chars()
            .enumerate()
            .filter(|(ii, _)| ii != &index)
            .map(|(_, ch)| ch)
            .collect()
    }

    pub fn slice(&self, s: &str, start: usize, end: usize) -> String {
        s.chars().skip(start).take(end - start).collect()
    }

    pub fn suffix(&self, s: &str, start: usize) -> String {
        s.chars().skip(start).collect::<String>()
    }

    pub fn at(&self, s: &str, i: isize) -> Option<char> {
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
