// odpi_rs - a thin wrapper over Oracle Database Programming Interface for C
//
// URL: https://github.com/kubo/odpi_rs
//
//-----------------------------------------------------------------------------
// Copyright (c) 2025 Kubo Takehiro <kubo@jiubao.org>. All rights reserved.
// This program is free software: you can modify it and/or redistribute it
// under the terms of:
//
// (i)  the Universal Permissive License v 1.0 or at your option, any
//      later version (http://oss.oracle.com/licenses/upl); and/or
//
// (ii) the Apache License v 2.0. (http://www.apache.org/licenses/LICENSE-2.0)
//-----------------------------------------------------------------------------
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
