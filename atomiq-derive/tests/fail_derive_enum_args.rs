#![cfg(not(clippy))]

use atomiq_derive::Atomizable;

#[derive(Atomizable)]
#[repr(u8)]
enum TestEnum {
    A,
    B(u8),
    C { value: u8 },
}

fn main() {}