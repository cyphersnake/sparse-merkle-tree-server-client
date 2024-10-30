# Network Application with Cryptographic Primitive

## Task Description

Develop a network application that fulfills the following requirements:

1. **Network Communication**: Implement two processes that communicate through the network stack.

2. **Cryptographic Primitive**: Integrate a single cryptographic primitive (e.g., signature verification, cryptographic accumulator) that serves a practical purpose within the application. Use any publicly available crate; re-implementation is not necessary.

3. **Documentation**: Provide a brief explanation of how the cryptographic primitive is used in rollups, including one or two examples. Discuss performance and security considerations for production applications.

---

**Evaluation Criteria**:

- **Coding Practices**: Clean, maintainable, and efficient code.
- **Functionality**: The application works correctly and demonstrates proper use of the cryptographic primitive.
- **Discussion Preparedness**: Be ready to discuss library choices, network design, overall architecture, and production challenges.

---

## Architectural Log

### 1. **Network Communication**

#### Reasoning
The most interesting would be to make a solution through some p2p library, but within the framework of the task of communication between two nodes it will be overcomplicated. I will try to start with a regular TCP connection, with direct serialization of data by serde + bincode. This small custom protocol will most likely be sufficient for the task at hand.

If node communication becomes more complex, then perhaps migration to a protobuf-like specification to unify the binary description would be worthwhile, but let's start simple

#### Decision
Communication directly via TCP

### 2. **Cryptographic Primitive**

#### Reasoning

Here the most difficult thing is to stay within the framework of the primitive, since the intention is to do some kind of proof check of the secret+sol from a hash, for example. However, in order not to consider a full-fledged proof-system, let's take the most popular data structure merkle-tree.

I think this is a very popular choice for this task, so let's try to complicate it a bit: let the tree be so big that we won't store it in its entirety, but assume that we know the initial state and load changes dynamically through the network. Let's use sparse-merkle-tree.

Since I can use third-party code, I'll use my own! It's open-source under MIT and was written for Sirius tests, but it hasn't been made into a separate crate yet, so I'll copy it to this repository. This code was written for a test and I wouldn't say it meets my own standards, but it would be all the more interesting to discuss it if you'd like!

#### Decision
Use sparse-merkle-tree with constant size and initial state. Pass new data of a particular leaf through the network and return a proof of consistency with new\old roots.

### 3. **Documentation**

The main use-case of sparse-merklee-tree is of course interative proof of changes in a large memory buffer. 

Inside zk-rollup (or any other systems supporting transactionalism), we can use sparse-merkle-tree to prove the transition of memory using proof-system. 

Let's try to consider such a system. (I'm not tied to any particular implementation, just using my intuition of how it could be used). All actions described below take place on-circuit of the vm:

1. Opcode changes the memory state
2. We prove that the changed memory location belongs to a merklee-tree node with a certain index
3. We prove sparse-merkle-tree-proof, obtaining a new root state and guarantees of consistency (old root -> new root)

Repeat these three steps for each memory change (or memory change batches) and obtain a proof for the new memory root vm. In this case, the private input is the sparse-merkle-tree-proof and the public input is the old and new merkle-tree-root.

This approach makes available a merkle-tree hash function for large data buffers, since directly proving the whole tree on-circuit would be too expensive in terms of circuit size, especially for each change. In this case, we only need to prove one hash function for each level of the tree: `log(leaf_count)` once.

### Conclusion

As far as I can judge, I have answered all the questions posed and justified my technology choices. But I would be glad to have any tests and discussions of my reasoning!

## Implementation

Due to my time constraints, I have implemented the requested functionality as simply as possible. Namely:

- Took the `SparseMerkleTree` I mentioned earlier and changed the internal data type to `u64`.
- I implemented a single-threaded server based on the standard `std::net::TcpListener` and a client based on `std::net::TcpStream`.
- Startup configuration based on command line arguments and `clap` crate

Thus the solution structure is as follows:

- main crate: includes a sparse merkle tree, as well as a module with a request and response structure.
- tree keeper: a server whose API allows changing the state of a sparse-merkle-tree
- leaf changer: cli utility that connects to the server and sends a request for change

### Possible improvements

If I were asked what are the next features I would implement, I would move towards demonstrating how such a tree allows you to work with memory buffers.

I.e. instead of working directly with tree & leaf at API level, migrate to working with a large memory buffer and provide evidence of changes in this memory buffer to the client in response. You could use opcode as a request to change memory at a certain offset.

Also complicate the work with the network to take p2p and allow each participant of the network to change the state of files to each other on request, however, take into account the asynchronous nature of these changes.

## Usage

### Install

Use [rustup](https://rustup.rs/)

### Tree Keeper

The server stores the sparse-merkle-tree and allows you to change its state via API by passing which leaf and what value to change to

```console
# Run tree-keeper server
cargo run-tree-keeper

# Argument help is also available
cargo run-tree-keeper --help
```

### Leaf Changer

This utility connects to Tree Keeper and changes the state of a particular tree leaf

```console
# Run leaf-changer cli tool
cargo run-leaf-changer --leaf-index 10 --new-data 50

# Argument help is also available
cargo run-leaf-changer --help
```

### Development

```console
# Build all crates
cargo build --workspace

# Run all tests
cargo test
```

# Conclusion

The assignment was interesting, considering the breadth of freedom. I wish I could play around with ideas more due to my workload, because even with sparse-merkle-tree there's still so much more to do! Thanks for your interest in me, I'd be happy to discuss it on the call!
