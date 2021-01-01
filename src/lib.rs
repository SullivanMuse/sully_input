use std::ops::RangeInclusive;

#[derive(Clone, Copy, PartialEq)]
pub struct Input<'a> {
    string: &'a str,
    index: usize,
    line: usize,
}

impl<'a> Input<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            string,
            index: 0,
            line: 1,
        }
    }

    fn curr(&self) -> &'a str {
        self.string.get(self.index..).unwrap_or("")
    }

    pub fn exact<E: Exact>(&self, exact: E) -> Option<(Self, ())> {
        let curr = self.curr();
        let rest = exact.exact(curr)?;
        let delta = curr.len() - rest.len();
        let interim = &curr[..delta];
        let newlines = interim.chars().fold(0, |acc, c| if c == '\n' {
            acc + 1
        } else {
            acc
        });
        let input = Self {
            string: self.string,
            index: self.index + delta,
            line: self.line + newlines,
        };
        Some((input, ()))
    }
}

impl<'a> std::fmt::Debug for Input<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slice = self.curr();
        let slice = &slice[..slice.len().min(15)];
        write!(f, "Input({:?} ({}) {:?})", self.index, self.line, slice)
    }
}

#[test]
fn test_debug_input() {
    let string = "word\nword\nword";
    let index = 5;
    let line = 2;
    let input = Input { string, index, line };
    assert_eq!(format!("{:?}", &input), "Input(5 (2) \"word\\nword\")".to_string());
}

#[derive(Clone, Copy, PartialEq)]
pub struct Span<'a> {
    string: &'a str,
    start: usize,
    end: usize,
    line: usize,
}

impl<'a> Span<'a> {
    pub fn slice(&self) -> &'a str {
        self.string.get(self.start..self.end).expect("Bad span.")
    }

    pub fn column(&self) -> usize {
        let string = &self.string[..self.start];
        let index = string.rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.start - index + 1
    }
    
    pub fn error(&self, message: &str) -> String {
        let start = self.string
            .get(..self.start)
            .unwrap()
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);
        let end = self.string
            .get(self.end..)
            .unwrap()
            .find('\n')
            .unwrap_or(self.string.len());
        format!(
            "[Error {line}:{column}] {message}\n{content}\n{leading}{carets}",
            line    = self.line,
            column  = self.column(),
            message = message,
            content = &self.string[start..end],
            leading = " ".repeat(self.column() - 1),
            carets  = "^".repeat(self.end - self.start),
        )
    }
}

impl<'a> std::fmt::Debug for Span<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slice = self.slice();
        write!(f, "Span({:?} ({}) {:?})", self.start..self.end, self.line, slice)
    }
}

#[test]
fn test_debug_span() {
    let string = "word\nword\nword";
    let start = 5;
    let end = 9;
    let line = 2;
    let span = Span { string, start, end, line };
    assert_eq!(format!("{:?}", span), "Span(5..9 (2) \"word\")".to_string());
}

pub trait Exact {
    fn exact<'a>(&self, input: &'a str) -> Option<&'a str>;
}

/// Parse a prefix matching the char exactly
impl Exact for char {
    fn exact<'a>(&self, input: &'a str) -> Option<&'a str> {
        input.strip_prefix(*self)
    }
}

/// Parse a prefix matching the &str exactly
impl Exact for &str {
    fn exact<'a>(&self, input: &'a str) -> Option<&'a str> {
        input.strip_prefix(self)
    }
}

/// Parse *any* of the characters in the inclusive range
impl Exact for RangeInclusive<char> {
    fn exact<'a>(&self, input: &'a str) -> Option<&'a str> {
        let c = input.chars().next()?;
        if self.contains(&c) {
            Some(&input[c.len_utf8()..])
        } else {
            None
        }
    }
}

/// Parse a single character matching the predicate
impl<F> Exact for F
where
    F: Fn(char) -> bool
{
    fn exact<'a>(&self, input: &'a str) -> Option<&'a str> {
        input.strip_prefix(self)
    }
}

#[test]
fn test_exact() {
    let input = Input::new("1234");
    let mut out = input;
    out.index += 1;
    assert_eq!(input.exact('0'..='9'), Some((out, ())));

    let input = Input::new("Hello");
    let mut out = input;
    out.index += 1;
    assert_eq!(input.exact('H'), Some((out, ())));

    let input = Input::new("Hello");
    let mut out = input;
    out.index += 5;
    assert_eq!(input.exact("Hello"), Some((out, ())));
}
