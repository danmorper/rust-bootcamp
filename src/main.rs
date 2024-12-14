// Define the Node structure for the BST

// Option<T> allows you to represent the possibility of a value being present or absent. 
// Some(T): Contains a value of type T.
// None: Does not contain a value.

// Box<T> is a smart pointer type that represents a heap-allocated value of type T.
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn main() {
    // Create the left child node
    let left_child = Node {
        value: 9,
        left: None,
        right: None,
    };

    // Create the right child node
    let right_child = Node {
        value: 11,
        left: None,
        right: None,
    };

    // Create the root node with left and right children
    let root = Node {
        value: 10,
        left: Some(Box::new(left_child)),
        right: Some(Box::new(right_child)),
    };

    // Print the root node's value
    println!("Root node value: {}", root.value);

    // Print the root node's left child value
    match &root.left {
        Some(left) => println!("Root node left child: {}", left.value),
        None => println!("Root node left child: None"),
    }

    // Print the root node's right child value
    match &root.right {
        Some(right) => println!("Root node right child: {}", right.value),
        None => println!("Root node right child: None"),
    }
}