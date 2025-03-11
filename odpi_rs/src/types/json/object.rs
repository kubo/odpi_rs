use super::*;
use std::iter::FusedIterator;
use std::str;

pub struct Iter<'a> {
    object: &'a Object<'a>,
    index: usize,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(object: &'a Object) -> Iter<'a> {
        Iter { object, index: 0 }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a Node<'a>);

    fn next(&mut self) -> Option<(&'a str, &'a Node<'a>)> {
        if self.index < self.object.len() {
            unsafe {
                let key = if let Ok(key) = str::from_utf8(self.object.key_as_bytes(self.index)) {
                    key
                } else {
                    // skip this key
                    self.index += 1;
                    return self.next();
                };
                let value = self.object.value_unchecked(self.index);
                self.index += 1;
                Some((key, value))
            }
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.object.len() - self.index;
        (len, Some(len))
    }
}

impl<'a> FusedIterator for Iter<'a> {}
