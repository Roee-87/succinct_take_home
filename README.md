# Succinct Code Assessment

## Design approach for computation graph and nodes

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

The node ID correlates to the nodes index in the graph vector. Instead of using a global counter variable, I simply used the length of the vector as the id value for a new node. Instead of returning a new node when adding a new node to the graph, I only return its index value. If a node needs to be accessed, its index value can be inputted into a getter function.

Inputs correspond to other nodes whose outputs are used as inputs for the current node's assigned calculation (add or multiply). The tuple corresponds to the index values of the input nodes. Constant value nodes and inputs nodes don't have any inputs, so their inputs tuple will always be `(None, None)`.

Output values are computed based on the op enum along with the input values obtained from the output values of the input nodes.

A node can be designated as a hint node. The hint value will point to a node whose value will be hinted at. The output for the hint node will always be externally computed.

## Constructing the computational graph

Here's an example of the following function can be represented with a computational graph:

$$ x^{2} + x + 5 $$

```Rust
let mut builder = Builder::new();
let x = builder.init();
let seven = builder.constant(7);
let x_plus_seven = builder.add(x, seven);
let sqrt_x_plus_7 = builder.hint(4, x_plus_seven);
let computed_sq = builder.mul(sqrt_x_plus_7, sqrt_x_plus_7);
```

`let mut = Builder::new()` initializes a new Builder object.

`let x = bulder.init()` initializes a new variable input node and adds the node to the `builder.graph` vector. the `x` variable is assigned the index value of the new node (in this case it is a usize `0`).

`let seven = builder.constant(7)` adds a constant node with a value of `7u32` to `builder.graph`. `seven` is assigned the index value of the new constant value node ( index value `1`).

`let x_plus_seven = builder.add(x, seven)` adds an output node whose output is the sum of `builder.graph[x].output` and `builder.graph[seven].output`.

`self.add(x, seven)

## Design approach for filling in values for the computational graph.

Output values for all nodes except constant value nodes are set to None when they are initialized. After the graph has been built, an input value is entered for the
