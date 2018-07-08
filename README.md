# spalks
A combination of Chalk and Specs intended to provide abstractions for integrating the Specs ECS with solver engine logic support

## Plan

This is an intial plan of how things will work.

What is desired is for an auxiliary solver system to be able to query things about entities in the ECS. Specifically, this means that it should be possible to build logical statements using the presence of Components and their associated data. Resolving operations involving n-components and their associated data should be possible.

So, if we require that two entities have a `Component` `struct X(i32)` and that the `i32` are equal, it should be possible to write a simple closure like `|X(a), X(b)| a == b` and pass that to a method which will have two implied template arguments of the `Component` type (`X` in this case) due to Rust's inferencing capability. Then this method will implicitly know the two `Component` types needed on two entities to fulfill the condition. For the condition to be fulfilled, it will need to both have the `Component` and also fulfill the logic condition. This could be done for not just one and two components, but `N` components, so that there can be functors with `N` number of arguments, each of which is an entity that fulfills the component conditions specified.

It should be possible to also just check for the existence of a component, and so another method should exist to check that. This should also be possible for `N` components.

Sometimes a more complex logical statement will need to be built which considers the data from multiple components of multiple entities simultaneously. To support this scenario, another variation of the above closure should exist which looks something like `|(X(a), Y(b)), Z(c)|`. This uses tuples to group together multiple components that need to be had by each entitiy for it to satisfy a condition. This form likely will require macros as a generation step for implementing the same trait for many tuple lengths since variadic generics/generic tuples doesn't seem to be in the Rust pipeline anytime soon. This will require at least 64 impls (8x8).

All of these functors need to be able to retrieve `Resource` from the `World` for global resources. The `Resource` does not exist on an entity and therefore doesn't need a functor parameter, but it does need to be passed into the closure somehow so it can be used. The way to solve this is to also implement the trait for variants of the closure parameters which begin with `Resource` or to pass the `Resource` as a variadic tuple at the first argument. The second solution seems good, but requires even more impls (8x8x8 = 512).

All functors in the solver take entities as arguments. Functors which draw from the component system, like the ones above, are leaf functors, and they are the source of truth data. However, higher-order functors can be build on those to form a directed graph which sources data from many leaf functors and the ECS and reduces data into further truth values. It should be possible for any functor to be assigned a truth value, which implies that the functor must be true. `spalks` should have at least one mode where it is first checked if the functor is already `Unknown` or not. If it is `Known` (`false` or `true`) already, then the logical statement is either redundant or contradictory.

Questions that can be proposed to the solver should include trying to find all entities that satisfy a condition or ask about the truth of a statement. This could include things like:
- Is there an object which is lit up in this particular room
- Give me all objects which are lit up in this particuar room
- Does this room contain any arachnids.
- Did the player win the game?
- Has the player been in this room?

All of the state which needs to be remembered to answer questions like `has this happened` must be stored in the ECS in some fashion.

Data should not propogate up from the `Component` level. After the `Component` level, all intermediate information should be truth values.

In the case that `chalk` is not used, intermediate results should be cached. This should be easy since the functor combined with an array of the entity IDs should be sufficient to use as a key in a hash table for lookup. An LRU cache would be a sufficient starting implementation, but it might be better to eventually also use execution time as a heuristic for the eviction strategy. If such an endeavor is undertaken, it may be difficult to deal with `Component` mutability. If a `Component` mutates, that would invalidate cached data, and so an informed decision about what to invalidate in the cache could be determined by also caching which cached answers might be invalidated by a `Component` mutation. To utilize this advantage, it is necessary to get a list of all the `Component` modifications in the `World`. If `chalk` is used, then it will cache things automatically, but that caching would probably lazily invalidate the cache entries, and it might not invalidate them as efficiently since it doesn't know that only leaf functors can be invalidated and when they specifically can be invalidated.

It should be fast to find entities which need to satisfy a condition which includes a rare `Component`. This is because we can iterate over all instances of the `Component` to find a match, and if there are few then it will be `O(n)` in time complexity across the number of entities with the `Component`. If the component is common, one should take great care not to invoke the query for every entity or for many entites, as that might make the time complexity `O(m*n)` where `m` is the number of entities satisfying the condition of the position they sit in a functor and `n` is the number of entities for which you had to run the query for. If possible, one should only make queries which evaluate every or many entities once per cycle of the ECS to avoid scalability issues.
