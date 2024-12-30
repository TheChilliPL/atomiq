#![cfg(not(clippy))]

use atomiq_derive::Atomizable;

#[derive(Atomizable)]
struct TestStruct;

fn main() {}