use my_graph_lib::*;

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
