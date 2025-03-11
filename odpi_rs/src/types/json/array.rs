use super::*;
use std::iter::FusedIterator;

pub struct Iter<'a> {
    array: &'a Array<'a>,
    index: usize,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(array: &'a Array) -> Iter<'a> {
        Iter { array, index: 0 }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Node<'a>;

    fn next(&mut self) -> Option<&'a Node<'a>> {
        if self.index < self.array.len() {
            let item = unsafe { self.array.get_unchecked(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.array.len() - self.index;
        (len, Some(len))
    }
}

impl<'a> FusedIterator for Iter<'a> {}
