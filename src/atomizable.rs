use crate::atom::{Atom, BitAtom, IntAtom};
use crate::Atomic;

/// Trait for types that may be represented as atomic values.
pub trait Atomizable {
    /// The primitive representation of the type.
    type Atom: Atom;
    
    /// Packs the value into its primitive representation.
    fn pack(self) -> Self::Atom;
    
    /// Unpacks the value from its primitive representation.
    fn unpack(atom: Self::Atom) -> Self;
}

impl<T: Atom> Atomizable for T {
    type Atom = T;

    fn pack(self) -> Self::Atom {
        self
    }
    
    fn unpack(atom: Self::Atom) -> Self {
        atom
    }
}

/// Trait for types that may be represented as atomic bit values.
pub trait BitAtomizable: Atomizable<Atom: BitAtom> {}

impl<T: BitAtom> BitAtomizable for T {}

/// Trait for types that may be represented as atomic integer values.
pub trait IntAtomizable: Atomizable<Atom: IntAtom> {}

impl<T: IntAtom> IntAtomizable for T {}

/// Extension trait for converting values into atomic.
/// 
/// This trait is implemented for all types that implement `Atomizable`.
pub trait Atomize {
    /// The primitive representation of the type.
    type Atom: Atom;

    /// Converts the value into an atomic.
    fn atomize(self) -> Atomic<Self::Atom>;
}

impl<T: Atomizable> Atomize for T {
    type Atom = T::Atom;
    
    fn atomize(self) -> Atomic<Self::Atom> {
        Atomic::from(self.pack())
    }
}