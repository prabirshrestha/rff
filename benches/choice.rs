#![feature(test)]
extern crate rff;
extern crate test;

use test::Bencher;
use rff::Choice;

#[bench]
fn create_choice(b: &mut Bencher) {
    b.iter(|| Choice::new("amor", "app/models/order"))
}