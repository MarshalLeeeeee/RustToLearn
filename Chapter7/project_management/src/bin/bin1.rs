// pub mod module; // module is not in bin/, therefore we cannot find module
// mod module;
use project_management::module;

fn test_module() {
    crate::module::f_absolute_path(crate::module::module::EnumSubModule::SubModule1, String::from("module function"));
    crate::module::f_absolute_path(crate::module::module::EnumSubModule::SubModule2, String::from("module function"));
    crate::module::f_relative_path(crate::module::module::EnumSubModule::SubModule1, String::from("module function"));
    crate::module::f_relative_path(crate::module::module::EnumSubModule::SubModule2, String::from("module function"));
    module::module::f_absolute_path(crate::module::module::EnumSubModule::SubModule1, String::from("module::module function"));
    module::module::f_absolute_path(crate::module::module::EnumSubModule::SubModule2, String::from("module::module function"));
    module::module::f_relative_path(crate::module::module::EnumSubModule::SubModule1, String::from("module::module function"));
    module::module::f_relative_path(crate::module::module::EnumSubModule::SubModule2, String::from("module::module function"));
}

fn main() {
    println!("bin1");
    test_module();
}
