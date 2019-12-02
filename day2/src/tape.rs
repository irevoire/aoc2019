//! All the code of below was imported from another project where I did wrote a
//! braifuck interpretor

use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Tape {
    vec: Vec<i32>,
}

impl Tape {
    /// Create a new empty Tape
    pub fn new() -> Self {
        Tape { vec: Vec::new() }
    }
}

/// Generate a new Tape from a vector with no negative values
impl std::iter::FromIterator<i32> for Tape {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let mut t = Self::new();
        let mut idx = 0;

        for val in iter {
            t[idx] = val;
            idx += 1;
        }
        t
    }
}

/// You can index into the tape as if it was an array.
/// There is two major differences:
/// - You don’t need to increase the size of the array or choose a size at start.
///   You can indexes whatever you want and the tape will grow to this size.
/// - You can use negative indexes: Since I was too bored to implements some
///   real negative indexes or something, when you use a positive indexe it will
///   use the even number and the odd number for negatives indexes.
///   Here is a scheme to understand how you number are placed in the tape:
///   .---+----+---+----+---+----+---+----.
///   | 0 | -1 | 1 | -2 | 2 | -3 | 3 | -4 | indexes you provide
///   '---+----+---+----+---+----+---+----'
///     0   1    2   3    4   5    6   7    indexes in the internal vector
///
impl Index<i32> for Tape {
    type Output = i32;
    fn index(&self, mut i: i32) -> &Self::Output {
        if i >= 0 {
            i = i * 2;
        } else {
            i = i * -2 - 1;
        }
        let i = i as usize;

        // if we don’t have the mutability we can’t allocate
        // the missings cells

        if i >= self.vec.len() {
            return &0;
        }
        &self.vec[i]
    }
}

impl IndexMut<i32> for Tape {
    fn index_mut<'a>(&'a mut self, mut i: i32) -> &'a mut Self::Output {
        if i >= 0 {
            i = i * 2;
        } else {
            i = i * -2 - 1;
        }
        let i = i as usize;

        if i >= self.vec.len() {
            self.vec.append(&mut vec![0; i - self.vec.len() + 1]);
        }
        &mut self.vec[i]
    }
}
