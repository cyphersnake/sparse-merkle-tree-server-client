/// The module provides the Sparse Merkle Tree data structure
///
/// # Simple Example
/// ```
/// let mut tr = poly_project::Tree::default();
/// println!("Default root is {}", tr.get_root());
///
/// let pr1 = tr.update_leaf(3, 1);
/// assert!(pr1.verify());
///
/// println!("Updated root is {}", tr.get_root());
/// ```
///
/// # Source
/// The original code was taken from
/// [Sirius](https://github.com/snarkify/sirius/blob/361-feat-protogalaxy-verify-circuit-8/examples/merkle/merkle_tree_gadget/off_circuit.rs).
///
/// Modified to work with `u64`, instead of `ff::PrimeField` elements
mod sparse_merkle_tree;

pub use sparse_merkle_tree::Tree;
