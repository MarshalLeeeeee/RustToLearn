// make crate::module public
// however since the module has been set public in lib.rs, we can directly use the module
// pub mod module;
use project_management::module; // we can directly use module or still use crate::module
// even if we do not use module2, we can still use project_management::module2
// use is just to make a shortcut for the path and make it under crate
// use project_management::module2; 
// same for module3

fn test_module() {
    // call module::f
    crate::module::f_absolute_path(crate::module::module::EnumSubModule::SubModule1, String::from("module function"));
    crate::module::f_absolute_path(crate::module::module::EnumSubModule::SubModule2, String::from("module function"));
    crate::module::f_relative_path(crate::module::module::EnumSubModule::SubModule1, String::from("module function"));
    crate::module::f_relative_path(crate::module::module::EnumSubModule::SubModule2, String::from("module function"));
    // call module::module::f
    module::module::f_absolute_path(crate::module::module::EnumSubModule::SubModule1, String::from("module::module function"));
    module::module::f_absolute_path(crate::module::module::EnumSubModule::SubModule2, String::from("module::module function"));
    module::module::f_relative_path(crate::module::module::EnumSubModule::SubModule1, String::from("module::module function"));
    module::module::f_relative_path(crate::module::module::EnumSubModule::SubModule2, String::from("module::module function"));
}

fn test_nested_module() {
    project_management::module3::f();
}

fn test_struct_enum() {
    // Error: field y is private
    // let module2_struct = project_management::module2::module2_struct {
    //     x: 1,
    //     y: 2,
    // };
    let module2_struct = project_management::module2::module2_struct::init(1, 2);
    // Error: field y is private
    // println!("module2_struct x {}, y {}", module2_struct.x, module2_struct.y);
    // This is ok as we call the public method of struct
    module2_struct.print_self();

    // It is okay to define from an enum's variant name
    let module2_enum_x = project_management::module2::module2_enum::X(1); module2_enum_x.print_self();
    let module2_enum_y = project_management::module2::module2_enum::Y(2); module2_enum_y.print_self();
}

fn test_lib() {
    project_management::lib_func(); // the name of lib module is what cargo.toml defined in [package]
}

fn main() {
    println!("main");
    test_module();
    test_nested_module();
    test_struct_enum();
    test_lib();
}
