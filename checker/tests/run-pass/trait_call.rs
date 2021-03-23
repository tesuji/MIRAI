// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

// Checks that calls via traits can be resolved if call site has enough type information

// MIRAI_FLAGS --diag=verify

use mirai_annotations::*;

pub trait Tr {
    fn bar(&self) -> i32;
}

struct Bar {
    i: i32,
}

impl Tr for Bar {
    fn bar(&self) -> i32 {
        self.i
    }
}

struct Foo {
    bx: Box<dyn Tr>,
}

pub fn t1() {
    let bar = Bar { i: 1 };
    let foo = Foo {
        bx: Box::new(bar) as Box<dyn Tr>,
    };
    let bi = foo.bx.bar();
    verify!(bi == 1);
}

pub fn t2(t: &dyn Tr) {
    let bi = t.bar(); //~ the called function did not resolve to an implementation with a MIR body
    verify!(bi == 3); // ignored because the previous unresolved call makes every subsequent thing moot
}

pub fn main() {}