/// A builder that will be used to create a computational graph and the hint graph.
#[derive(Debug, Clone)]
pub struct Builder {
    graph: Vec<Node>,
}

/// A node in the computational graph.
#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub id: usize,
    pub inputs: (Option<usize>, Option<usize>), // (Some(a), Some(b)) are indices of nodes whose outputs are used as inputs for the current node.
    pub op: Option<OPERATION>,
    pub output: Option<u32>,
    pub hint: Option<usize>,
}

/// The operations that can be performed in the computational graph.
#[derive(Debug, Copy, Clone)]
pub enum OPERATION {
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
