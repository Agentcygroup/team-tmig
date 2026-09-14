pub mod eig;
pub mod entropy;
pub mod matrix;
pub mod rank;
pub use eig::power_iteration;
pub use entropy::shannon_entropy;
pub use matrix::Matrix;
pub use rank::rank;
