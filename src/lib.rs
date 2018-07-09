extern crate slab;
extern crate specs;

use slab::Slab;
use specs::{world::Index, World};

mod trilean;
pub use trilean::Trilean;

/// This trait is implemented by functors that resolve to a truth value about `M` components on `N` entities.
pub trait Functor {
    fn answer(&self, &World, &[Index]) -> Trilean;

    // TODO: This needs to report which `Resource` and `Component` it touches for cache invalidation purposes.
}

pub struct Ontology {
    functors: Slab<Box<Functor>>,
}

impl Ontology {
    /// Find all entities that satisfy a functor with one parameter.
    ///
    /// If more entites are involved, build a functor which captures them and .
    pub fn existential<F: Functor>(&self, world: &World, functor: F) {
        unimplemented!()
    }

    /// Determine truth of a functor given specific params.
    pub fn answer(&self, world: &World, functor: usize, params: &[Index]) -> Trilean {
        unimplemented!()
    }

    /// Register a new functor with the `Ontology` and get back a handle.
    pub fn register(&mut self, f: Box<Functor>) -> usize {
        self.functors.insert(f)
    }
}
