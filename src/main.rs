// rust-101-examples
//
// Written in 2021 by Akshay Gupta <kitallis@hey.com>
//
// To the extent possible under law, the author(s) have dedicated all copyright and related and
// neighboring rights to this software to the public domain worldwide. This software is distributed
// without any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication along with this software. If
// not, see <http://creativecommons.org/publicdomain/zero/1.0/>.

#![allow(unused_imports)]

pub mod borrowing;
pub mod ownership;
pub mod strings;

/// Document Rust concepts in a literate programming style.
pub fn main() {
    // ownership
    ownership::moves();
    ownership::cloning();
    ownership::copy_traits();
    ownership::returns();

    // borrowing
    borrowing::shared();
    borrowing::mutable_until_borrowed();
    borrowing::borrow_lifetime();

    // strings
    strings::slice();
}
