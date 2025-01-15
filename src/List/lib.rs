/// This module contains the List and Node structures, as well as the Memory manager.
pub mod list;
pub mod mem;
pub mod node;
pub mod C;


// Re-exporting the List type for easy access.
pub use list::List;
pub use mem::Memory;
pub use node::Node;

pub mod List;
