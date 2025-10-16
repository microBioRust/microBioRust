pub mod gbk;         // expose gbk.rs
pub use gbk::genbank; // expose `genbank!` macro or function if defined in gbk.rs
pub mod embl;
pub use embl:embl;
