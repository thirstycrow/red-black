# Red-Black Tree

The purpose of this library is to provide an algorithm framework that allows
users to create memory efficient red-black tree.

The algorithm is implemented on top of the `Node` and `NodePtr` traits, instead
of concrete structs. Users can define their own memory layout with techniques
such as bit field or shorter memory address to reduce the per node memory
footprint. Parent pointers are not necessary in this implementation to reduce
memory consumption. Instead, a temporary tree which keeps the parent
relationship is maintained on the call stack while traversing the tree nodes.
As a result, this is not an in-place implementation.
