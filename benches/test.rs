#![feature(test)]

extern crate test;

use swtch_regexp::automa::{Dfa, Nfa};
use test::Bencher;

#[bench]
fn nfa_test_bench(bench: &mut Bencher) {
    let nfa = Nfa::from_str("abb.+.a.").unwrap();
    bench.iter(|| {
        (0..100).for_each(|_| {
            nfa.test("aabbbba");
        })
    });
}

#[bench]
fn dfa_test_bench(bench: &mut Bencher) {
    let dfa = Dfa::from_str("abb.+.a.").unwrap();
    bench.iter(|| {
        (0..100).for_each(|_| {
            dfa.test("aabbbba");
        })
    });
}
