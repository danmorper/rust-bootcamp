#[derive(Debug)] // To be able to print the struct with println!

// Define the Node struct (like a class in OOP)
struct Node {
    value: i32, // Integer value.
    left: Option<Box<Node>>, // Left child node. Option<Box<Node>> is a recursive data structure.
    right: Option<Box<Node>>, // Right child node. 
    // Option is an enum with two variants: Some(T) and None.
    // Box is somethong to store data. I don't really understand it yet.
}

// Implementation block, to add methods to the Node struct.
impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
    
    // Insert a value into the BST.
    // The function is recursive, because it calls itself.
    // mut self means that the function can modify the struct.
    fn insert(&mut self, value: i32) {
        if value < self.value {
            // match is like an if-else statement. In this case, it checks if self.left is Some or None.
            match self.left {
                Some(ref mut left_node) => left_node.insert(value),
                // ref allows to call self.left as left_node. mut allows to modify left_node.
                None => self.left = Some(Box::new(Node::new(value))),
                // If self.left is None, create a new Node and assign it to self.left.
            }
        } else if value > self.value {
            match self.right {
                Some(ref mut right_node) => right_node.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    // mut is not used because the function does not modify the struct.
    // & is used to pass self by reference. What it means you can not modify the struct.
    fn search(&self, value: i32) -> bool {
        if self.value == value {
            // Return true if the value is found. It can be written as "return true;".
            true
        } else if value < self.value {
            match &self.left {
                Some(left_node) => left_node.search(value),
                None => false,
            }
        } else {
            match &self.right {
                Some(right_node) => right_node.search(value),
                None => false,
            }
        }
    }

    // It orders the nodes in the BST in ascending order. result is a mutable reference to a vector.
    // Remember that left_node < current_node < right_node.
    fn inorder_traversal(&self, result: &mut Vec<i32>) {
        // If the left child exists, call the function recursively.
        if let Some(ref left_node) = self.left {
            left_node.inorder_traversal(result);
        }
        // Add the value of the current node to the vector.
        result.push(self.value);
        // If the right child exists, call the function recursively.
        if let Some(ref right_node) = self.right {
            right_node.inorder_traversal(result);
        }
    }

    fn height(&self) -> i32 {
        let left_height = match &self.left {
            Some(left_node) => left_node.height(),
            None => 0,
        };
        let right_height = match &self.right {
            Some(right_node) => right_node.height(),
            None => 0,
        };
        // Return the height of the current node.
        // std::cmp::max returns the maximum of two values.
        1 + std::cmp::max(left_height, right_height)
    }
}

struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, value: i32) {
        match self.root {
            Some(ref mut root_node) => root_node.insert(value),
            None => self.root = Some(Box::new(Node::new(value))),
        }
    }

    fn search(&self, value: i32) -> bool {
        match &self.root {
            Some(root_node) => root_node.search(value),
            None => false,
        }
    }

    fn inorder_traversal(&self) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(ref root_node) = self.root {
            root_node.inorder_traversal(&mut result);
        }
        result
    }

    fn height(&self) -> i32 {
        match &self.root {
            Some(root_node) => root_node.height(),
            None => 0,
        }
    }
}

fn main() {
    let mut bst = BinarySearchTree::new();

    // Example: Construct the BST from a vector of integers
    let values = vec![10, 5, 20, 3, 7, 15, 25];
    for value in values {
        bst.insert(value);
    }

    // Search function
    println!("Search for 7: {}", bst.search(7)); // true
    println!("Search for 30: {}", bst.search(30)); // false

    // In-order traversal
    println!("In-order traversal: {:?}", bst.inorder_traversal()); // [3, 5, 7, 10, 15, 20, 25]

    // Height of the tree
    println!("Height of the tree: {}", bst.height()); // 3
}
