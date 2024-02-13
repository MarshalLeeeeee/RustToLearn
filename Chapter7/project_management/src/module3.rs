mod sub_module3; // do not make the whole submodule public
pub use sub_module3::f; // but make certain identifier public by pub use