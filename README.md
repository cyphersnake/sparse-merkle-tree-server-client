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

### Conclusion

As far as I can judge, I have answered all the questions posed and justified my technology choices. But I would be glad to have any tests and discussions of my reasoning!
