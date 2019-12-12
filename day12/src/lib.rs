pub mod moon;
pub mod system;

pub use moon::*;
pub use system::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Dimension {
    X,
    Y,
    Z,
    All,
}
