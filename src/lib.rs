use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::mem;

#[derive(Debug, Clone, Copy)]
pub struct AsciiCaselessString<T = String>(T);
impl<T: AsRef<str>> AsciiCaselessString<T> {
    pub fn new(inner: T) -> Self {
        AsciiCaselessString(inner)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    pub fn as_inner_ref(&self) -> &T {
        &self.0
    }

    pub fn as_inner_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}
impl<T: AsRef<str>> PartialEq for AsciiCaselessString<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref().eq_ignore_ascii_case(other.0.as_ref())
    }
}
impl<T: AsRef<str>> Eq for AsciiCaselessString<T> {}
impl<T: AsRef<str>> PartialOrd for AsciiCaselessString<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let iter0 = self.0.as_ref().chars().map(|c| c.to_ascii_lowercase());
        let iter1 = other.0.as_ref().chars().map(|c| c.to_ascii_lowercase());
        iter0.partial_cmp(iter1)
    }
}
impl<T: AsRef<str>> Ord for AsciiCaselessString<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let iter0 = self.0.as_ref().chars().map(|c| c.to_ascii_lowercase());
        let iter1 = other.0.as_ref().chars().map(|c| c.to_ascii_lowercase());
        iter0.cmp(iter1)
    }
}
impl<T: AsRef<str>> Hash for AsciiCaselessString<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0
            .as_ref()
            .chars()
            .for_each(|c| c.to_ascii_lowercase().hash(hasher));
    }
}
impl<T: AsRef<str>> AsRef<T> for AsciiCaselessString<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
impl<T: AsRef<str>> AsMut<T> for AsciiCaselessString<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
impl<T: AsRef<str>> Borrow<AsciiCaselessStr> for AsciiCaselessString<T> {
    fn borrow(&self) -> &AsciiCaselessStr {
        AsciiCaselessStr::new(self.0.as_ref())
    }
}
impl<T: AsRef<str>> From<T> for AsciiCaselessString<T> {
    fn from(f: T) -> Self {
        Self::new(f)
    }
}

#[derive(Debug)]
pub struct AsciiCaselessStr(str);
impl AsciiCaselessStr {
    pub fn new(inner: &str) -> &Self {
        unsafe { mem::transmute(inner) }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl PartialEq for AsciiCaselessStr {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(other.0.as_ref())
    }
}
impl Eq for AsciiCaselessStr {}
impl PartialOrd for AsciiCaselessStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let iter0 = self.0.chars().map(|c| c.to_ascii_lowercase());
        let iter1 = other.0.chars().map(|c| c.to_ascii_lowercase());
        iter0.partial_cmp(iter1)
    }
}
impl Ord for AsciiCaselessStr {
    fn cmp(&self, other: &Self) -> Ordering {
        let iter0 = self.0.chars().map(|c| c.to_ascii_lowercase());
        let iter1 = other.0.chars().map(|c| c.to_ascii_lowercase());
        iter0.cmp(iter1)
    }
}
impl Hash for AsciiCaselessStr {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0
            .chars()
            .for_each(|c| c.to_ascii_lowercase().hash(hasher));
    }
}
impl AsRef<str> for AsciiCaselessStr {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl<'a> From<&'a str> for &'a AsciiCaselessStr {
    fn from(f: &'a str) -> Self {
        AsciiCaselessStr::new(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn it_works() {
        assert_eq!(
            AsciiCaselessString::new("foo"),
            AsciiCaselessString::new("FOO")
        );
    }

    #[test]
    fn hashset_works() {
        let mut set = HashSet::new();
        set.insert(AsciiCaselessString::new("FoO".to_owned()));
        assert!(set.contains(AsciiCaselessStr::new("foo")));
    }
}
