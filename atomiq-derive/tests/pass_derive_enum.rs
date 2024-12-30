use atomiq::{Atomic, Atomizable, Atomize, Ordering};
use atomiq_derive::Atomizable;

#[derive(Atomizable, Debug, PartialEq)]
#[repr(i32)]
enum TestEnum {
    A,
    B,
    C,
}

fn main() {
    let test_enum = TestEnum::B;

    let atom: i32 = test_enum.pack();
    let test_struct: TestEnum = TestEnum::unpack(atom);

    assert_eq!(test_struct, TestEnum::B);

    let atomic_struct: Atomic<TestEnum> = test_struct.atomize();

    atomic_struct.store(TestEnum::C, Ordering::Relaxed);

    let loaded: TestEnum = atomic_struct.load(Ordering::Relaxed);

    assert_eq!(loaded, TestEnum::C);
}