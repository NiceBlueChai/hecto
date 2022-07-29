use std::cmp;
use std::ops::Deref;

#[derive(Debug)]
pub struct Row {
    string: String,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: slice.to_string(),
        }
    }
}

impl Deref for Row {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.string.deref()
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String{
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }
}