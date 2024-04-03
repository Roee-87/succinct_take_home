# Succinct Code Assessment

## Design approach

The computational graph is defined as a vector of nodes.

```Rust
struct Builder {
    graph: Vec<Node>
}
```

A node is defined as follows:

```Rust
struct Node {
    id: usize,
    inputs: (Option<usize>, Option<usize>),
    op: Option<OPERATION>,
    output: Option<u32>,
    hint: Option<usize>,
}
```

The node ID correlates to the nodes index in the graph vector. Instead of using a global counter variable, I simply used the length of the vector as the id value for a new node. Instead of returning a new node when adding a new node to the graph, I only return its index value. If a node needs to be accessed, I provided a getter function.
