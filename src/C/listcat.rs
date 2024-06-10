#[macro_export]
macro_rules! listcat {
    // Base case: When one vector is passed, just clone it
    ($x:expr) => ($x.clone());

    // Recursive case: Process the first vector and recurse for the rest
    ($first:expr, $($rest:expr),+) => {{
        let mut combined = $first.clone(); // Start with a clone of the first vector
        // Use a block to encapsulate the expansion logic
        $(
            combined.extend($rest.clone()); // Extend the combined vector with clones of the rest
        )*
        combined
    }};
}