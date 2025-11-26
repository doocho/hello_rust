pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

struct LineRef<'a> {
    line: &'a str,
}

impl<'a> LineRef<'a> {
    fn new(line: &'a str) -> Self {
        Self { line }
    }

    fn as_str(&self) -> &str {
        self.line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longer_test() {
        let line = LineRef::new("hello world");
        assert_eq!(line.as_str(), line.line);
    }
}