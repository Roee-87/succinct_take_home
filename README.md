# Succinct Code Assessment

The library code can be found in src/lib.rs. I used main.rs to debug and inspect Node and Builder structs. Tests are included in main.rs in the tests module.

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

Operations are restricted to either addition or multiplication. This is reprsesnted as an enum in the op field.

```Rust
enum OPERATION {
    ADD,
    MUL,
}
```

Inputs correspond to other nodes whose outputs are used as inputs for the current node's assigned calculation (add or multiply). The tuple corresponds to the index values of the input nodes. Constant value nodes and inputs nodes don't have any inputs, so their inputs tuple will always be `(None, None)`.

Output values are computed based on the op enum along with the input values obtained from the output values of the input nodes.

A node can be designated as a hint node. The hint value will point to a node whose value will be hinted at. The output for the hint node will always be externally computed.

## Constructing the computational graph

Here's an example of the following function can be represented with a computational graph:

$$ x^{2} + x + 5 $$

```Rust
let mut builder = Builder::new();
let x = builder.init();
let x_squared = builder.mul(x, x);
let five = builder.constant(5);
let x_squared_plus_5 = builder.add(x_squared, five);
let y = builder.add(x_squared_plus_5, x);
```

`let mut = Builder::new()` initializes a new Builder object.

`let x = bulder.init()` initializes a new variable input node and adds the node to the `builder.graph` vector. The `x` variable is assigned the index value of the new node (in this case it is a usize `0`).

`let five = builder.constant(5);` adds an constant value node whose output `5u32`. The node is added to `builder.graph` with index value `1`.

`let x_squared_plus_5 = builder.add(x_squared, five)` adds an output node whose output is $$ x^{2} + 5 $$. The node is added to `builder.graph` with index value `2`.

`let y = builder.add(x_squared_plus_5, x)` adds an output node whose output is $$ x^{2} + x + 5 $$. The node is added to `builder.graph` with index value `3`.

The graph is now complete and ready to be filled using an input value for the `x` variable node.

## Design approach for filling in values for the computational graph.

Output values for all nodes except constant value nodes are set to None when they are initialized. After the graph has been built, an input value is entered for the input variable node using the `self.fill_nodes()` method.

Continuing from the example above:

```Rust
builder.fill_nodes(x, 6);
```

Will enter a value `6u32` into the `x` variable node. We then traverse the computational graph and calculate the outputs of each node using input values and the corresponding operation. The indices in the `inputs` field tuple correspond to input nodes. The output values from those nodes are used as inputs for the current nodes computation. Using pattern matching on the `op` field of the Node struct, we execute the correct calculation and update the output field of with the result of the calculation.

Constraint checking is preformed using an almost identical approach after a computation graph has been filled.

## Hints

Hint values are added as nodes to the graph vector. Hint nodes contain an output that cannot be directly computed inside of the computation graph. These nodes link to an output that can be calculated inside of the computational graph. The following example can be used prove we know a value when summed with seven will have a real square root:

$$\sqrt{x + 7}$$

```Rust
    let mut builder = Builder::new();
    let x = builder.init();
    let seven = builder.constant(7);
    let x_plus_seven = builder.add(x, seven);
    let sqrt_x_plus_7 = builder.hint(4, x_plus_seven);
    let computed_sq = builder.mul(sqrt_x_plus_7, sqrt_x_plus_7);
    builder.fill_nodes(x, 9);
    let hint_eq = builder.assert_equal(sqrt_x_plus_7, computed_sq);
    println!("Hint equality holds: {:?}", hint_eq);
```

The square root of the summed values will equal `4`. Since we cannot directly compute the square root, we compute the square of `4` inside the computational graph and link the output to the sum of `x + 7`. Establishing equivalence between `x+7` and $$ 4^{2} $$ demonstrates that we know a valid value `x` that upholds the constraint.

## Run the code

Running

```bash
cargo run
```

runs the main function, which I used for debugging and inspecting the Node and graph structs.

Running

```bash
cargo test
```

runs the test cases for the three scenarios outlined in the assessment document.

## Notes

The coding task took approximately two hours and the README write up took about 30 minutes.
