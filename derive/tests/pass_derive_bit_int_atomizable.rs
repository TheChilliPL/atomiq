use atomiq::{Atomic, Atomizable, Atomize, Ordering};
use atomiq_derive::{Atomizable, BitAtomizable, IntAtomizable};

#[derive(Atomizable, BitAtomizable, IntAtomizable)]
struct TestStruct(u8);

fn main() {
    let test_struct = TestStruct(42);
    
    let atom: u8 = test_struct.pack();
    let test_struct: TestStruct = TestStruct::unpack(atom);
    
    assert_eq!(test_struct.0, 42);
    
    let atomic_struct: Atomic<TestStruct> = test_struct.atomize();
    
    let prev = atomic_struct.fetch_and(TestStruct(3), Ordering::Relaxed);
    
    assert_eq!(prev.0, 42);
    assert_eq!(atomic_struct.load(Ordering::Relaxed).0, 2);
    
    let prev = atomic_struct.fetch_add(TestStruct(1), Ordering::Relaxed);
    
    assert_eq!(prev.0, 2);
    assert_eq!(atomic_struct.load(Ordering::Relaxed).0, 3);
}