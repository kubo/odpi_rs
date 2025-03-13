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
