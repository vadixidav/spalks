extern crate slab;
extern crate specs;

use slab::Slab;
use specs::{world::Index, World};

/// This trait is implemented by functors that resolve to a truth value about `M` components on `N` entities.
pub trait Functor {
    fn answer(&self, &World, &[Index]) -> bool;

    // TODO: This needs to report which `Resource` and `Component` it touches for cache invalidation purposes.
}

pub struct Ontology {
    functors: Slab<Box<Functor>>,
}

impl Ontology {
    /// Find all entities that satisfy a functor.
    ///
    /// Asks for a functor to be satisfied given several parameter entities and an
    /// argument positon to find all entities that satisfy the functor. This returns
    /// a lazily evaluated iterator so that just one entity can be retrieved that satisfies
    /// the condition if that is desired. If there is only one entity, that can save some
    /// execution time.
    fn existential(&self, world: &World, functor: usize, position: usize, params: &[Index]) {
        unimplemented!()
    }

    /// Determine truth of a functor given specific params.
    fn answer(&self, world: &World, functor: usize, params: &[Index]) -> bool {
        unimplemented!()
    }
}
