pub mod my_module;

mod my_module2;
pub use my_module2::MyTrait2;
pub use test_macro_derive::MyTrait2;

pub use test_macro_derive::attr_macro;

pub use test_macro_derive::func_macro;
