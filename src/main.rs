/// A builder that will be used to create a computational graph and the hint graph.
#[derive(Debug, Clone)]
struct Builder {
    graph: Vec<Node>,
}

/// A node in the computational graph.
#[derive(Debug, Copy, Clone)]
struct Node {
    id: usize,
    inputs: (Option<usize>, Option<usize>), // (Some(a), Some(b)) are indices of nodes in the computation graph.  Outputs from those nodes are used as inputs for the current noe.
    op: Option<OPERATION>,
    output: Option<u32>,
    hint: Option<usize>,
}

/// The operations that can be performed in the computational graph.
#[derive(Debug, Copy, Clone)]
enum OPERATION {
    ADD,
    MUL,
}

impl Builder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Builder { graph: Vec::new() }
    }

    /// Initializes a node in the graph.
    pub fn init(&mut self) -> usize {
        let node_id = self.graph.len();
        let new_node = Node {
            id: node_id,
            inputs: (None, None),
            op: None,
            output: None,
            hint: None,
        };
        self.graph.push(new_node);
        node_id
    }

    /// Initializes a node in a graph, set to a constant value.
    pub fn constant(&mut self, val: u32) -> usize {
        let node_id = self.graph.len();
        let new_node = Node {
            id: node_id,
            inputs: (None, None),
            op: None,
            output: Some(val),
            hint: None,
        };
        self.graph.push(new_node);
        node_id
    }
    /// Creates a new node by adding two nodes in the graph, returning the index of the new node in the graph.
    pub fn add(&mut self, a: usize, b: usize) -> usize {
        let node_id = self.graph.len();
        let new_node = Node {
            id: node_id,
            inputs: (Some(a), Some(b)),
            op: Some(OPERATION::ADD),
            output: None,
            hint: None,
        };
        self.graph.push(new_node);
        node_id
    }

    /// Multiplies two nodes in the graph, returning the index of the new node in the graph.
    pub fn mul(&mut self, a: usize, b: usize) -> usize {
        let node_id = self.graph.len();
        let new_node = Node {
            id: node_id,
            inputs: (Some(a), Some(b)),
            op: Some(OPERATION::MUL),
            output: None,
            hint: None,
        };
        self.graph.push(new_node);
        node_id
    }

    /// Hint value is externally computed.  We "link" it to the node that it is dependent on.
    pub fn hint(&mut self, hint_value: u32, hint_node: usize) -> usize {
        let node_id = self.graph.len();
        let new_node = Node {
            id: node_id,
            inputs: (None, None),
            op: None,
            output: Some(hint_value),
            hint: Some(hint_node),
        };
        self.graph.push(new_node);
        node_id
    }

    /// Asserts that 2 nodes are equal for hinted values.
    /// This is a constraint that is checked after the graph is filled in.
    /// The first input should be the hint node, and the second input should be the node containing the output value that the hint node links to.
    pub fn assert_equal(&self, a: usize, b: usize) -> bool {
        let dependent_node_index = self.graph[a].hint.unwrap();
        let dependent_output = self.graph[dependent_node_index].output.unwrap();
        let hinted_output = self.graph[b].output.unwrap();
        assert_eq!(dependent_output, hinted_output);
        true
    }

    /// Fills in all the nodes of the graph based on some inputs.
    pub fn fill_nodes(&mut self, input_node: usize, input_val: u32) {
        // We fill in the input value for the variable input node.
        self.graph[input_node].output = Some(input_val);

        // We then iterate through the graph and fill in the values for the rest of the nodes.
        // We use the `inputs` tuple to obtain the indices of the input nodes.
        // Input and Constant nodes have a (None, None) tuple for inputs...no match block needed for that case. 
        for node in 0..self.graph.len() {
            if let (Some(a), Some(b)) = self.graph[node].inputs {
                let a_val = self.graph[a].output.unwrap();
                let b_val = self.graph[b].output.unwrap();
                match self.graph[node].op {
                    Some(OPERATION::ADD) => {
                        self.graph[node].output = Some(a_val + b_val);
                    }
                    Some(OPERATION::MUL) => {
                        self.graph[node].output = Some(a_val * b_val);
                    }
                    None => {} // This should never be a case, but it's here to satisfy the compiler.
                }
            }
        }
    }

    /// Given a graph that has `fill_nodes` already called on it
    /// checks that all the constraints hold.
    pub fn check_constraints(&self) -> bool {
        for node in 0..self.graph.len() {
            if let (Some(a), Some(b)) = self.graph[node].inputs {
                let a_val = self.graph[a].output.unwrap();
                let b_val = self.graph[b].output.unwrap();
                match self.graph[node].op {
                    Some(OPERATION::ADD) => {
                        assert_eq!(self.graph[node].output.unwrap(), a_val + b_val);
                    }
                    Some(OPERATION::MUL) => {
                        assert_eq!(self.graph[node].output.unwrap(), a_val * b_val);
                    }
                    None => {}
                }
            }
        }
        true
    }

    /// Getter function for obtaining a node from the graph.
    pub fn get_node(self, id: usize) -> Node {
        self.graph[id]
    }
}

fn main() {
    // I used the following code to inspect and debug the implementation of the computational graph.
    let mut builder = Builder::new();
    let x = builder.init();
    let seven = builder.constant(7);
    let x_plus_seven = builder.add(x, seven);
    let sqrt_x_plus_7 = builder.hint(4, x_plus_seven);
    let computed_sq = builder.mul(sqrt_x_plus_7, sqrt_x_plus_7);
    // We can inspect the whole graph.  Output values for all nodes except constants should be None.
    println!("computational graph before filling nodes: {:?}", builder);
    builder.fill_nodes(x, 9);
    // Every node should now have an output value corresponding to its calculation.
    println!("computational graph after filling nodes: {:?}", builder);
    // Alternatively, we can inspect individual nodes.
    // Note:  Cloning is only used for debugging purposes...quick way to avoid borrowing issues but obviously not efficient for large graphs, production code, etc.
    let x_plus_seven_node = builder.clone().get_node(x_plus_seven);
    println!("Node {}: {:?}", x_plus_seven_node.id, x_plus_seven_node);
    // We can check that the constraints hold.
    // Note:  Cloning is only used for debugging purposes.
    let answer = builder.clone().check_constraints();
    println!("Constraints hold: {:?}", answer);
    // We can check that the equivalence betwee the hint and the computed square holds.
    // Note:  Cloning is only used for debugging purposes.
    let hint_eq = builder.clone().assert_equal(sqrt_x_plus_7, computed_sq);
    println!("Hint equality holds: {:?}", hint_eq);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial() {
        let mut builder = Builder::new();
        let x = builder.init();
        let x_squared = builder.mul(x, x);
        let five = builder.constant(5);
        let x_squared_plus_5 = builder.add(x_squared, five);
        let y = builder.add(x_squared_plus_5, x);
        builder.fill_nodes(x, 6);
        builder.check_constraints();
        assert_eq!(builder.get_node(y).output.unwrap(), 47);
    }


    #[test]
    #[should_panic]
    fn test_polynomial_panic() {
        let mut builder = Builder::new();
        let x = builder.init();
        let x_squared = builder.mul(x, x);
        let five = builder.constant(5);
        let x_squared_plus_5 = builder.add(x_squared, five);
        let y = builder.add(x_squared_plus_5, x);
        builder.fill_nodes(x, 6);
        builder.check_constraints();
        assert_eq!(builder.get_node(y).output.unwrap(), 46);
    }

    #[test]
    fn test_hint() {
        let mut builder = Builder::new();
        let a = builder.init();
        let one = builder.constant(1);
        let b = builder.add(a, one);
        // Now we need to show that (a + 1) / 8 == 1
        let c = builder.hint(1, b);
        let eight = builder.constant(8);
        let c_times_8 = builder.mul(c, eight);
        // We can show that a + 1 / 8 == 1 by showing that c * 8 == b
        builder.fill_nodes(a, 7);
        builder.check_constraints();
        let _ = builder.assert_equal(c, c_times_8);
    }

    #[test]
    #[should_panic]
    fn test_hint_panic() {
        let mut builder = Builder::new();
        let a = builder.init();
        let one = builder.constant(1);
        let b = builder.add(a, one);
        let c = builder.hint(1, b);
        let eight = builder.constant(8);
        let c_times_8 = builder.mul(c, eight);
        // This should panic since it will attempt to assert that 7 == 8;
        builder.fill_nodes(a, 6);
        builder.check_constraints();
        let _ = builder.assert_equal(c, c_times_8);
    }

    #[test]
    fn test_sqrt() {
        let mut builder = Builder::new();
        let x = builder.init();
        let seven = builder.constant(7);
        let x_plus_seven = builder.add(x, seven);
        let sqrt_x_plus_7 = builder.hint(4, x_plus_seven);
        let computed_sq = builder.mul(sqrt_x_plus_7, sqrt_x_plus_7);
        builder.fill_nodes(x, 9);
        builder.check_constraints();
        let _ = builder.assert_equal(sqrt_x_plus_7, computed_sq);
    }

    #[test]
    #[should_panic]
    fn test_sqrt_panic() {
        let mut builder = Builder::new();
        let x = builder.init();
        let seven = builder.constant(7);
        let x_plus_seven = builder.add(x, seven);
        let sqrt_x_plus_7 = builder.hint(4, x_plus_seven);
        let computed_sq = builder.mul(sqrt_x_plus_7, sqrt_x_plus_7);
        builder.fill_nodes(x, 10);
        builder.check_constraints();
        let _ = builder.assert_equal(sqrt_x_plus_7, computed_sq);
    }


}
