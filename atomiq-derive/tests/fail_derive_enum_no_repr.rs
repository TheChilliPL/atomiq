#![cfg(not(clippy))]

use atomiq_derive::Atomizable;

#[derive(Atomizable)]
enum TestEnum {
    A,
    B,
    C,
}

fn main() {}