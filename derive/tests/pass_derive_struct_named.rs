use atomiq::prelude::*;
use atomiq_derive::Atomizable;

#[derive(Atomizable)]
struct TestStruct {
    test_value: i32,
}

fn main() {
    let test_struct = TestStruct { test_value: 42 };
    
    let atom: i32 = test_struct.pack();
    let test_struct: TestStruct = TestStruct::unpack(atom);
    
    assert_eq!(test_struct.test_value, 42);
    
    let atomic_struct: Atomic<TestStruct> = test_struct.atomize();
    
    atomic_struct.store(TestStruct { test_value: 43 }, Ordering::Relaxed);
    
    let loaded: TestStruct = atomic_struct.load(Ordering::Relaxed);
    
    assert_eq!(loaded.test_value, 43);
}