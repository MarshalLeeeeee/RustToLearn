
pub mod module {
    pub enum EnumSubModule { // this enum is public
        SubModule1,
        SubModule2,
    }

    mod sub_module1 { // this submodule is private
        pub fn f(s: String) {
            println!("sub_module1::f {}", s);
        }
    }
    mod sub_module2 { // this submodule is private
        pub fn f(s: String) {
            println!("sub_module2::f {}", s);
        }
    }

    pub fn f_absolute_path(e: EnumSubModule, mut s: String) { // this function is public
        s.push_str(" - absolute path");
        match e {
            EnumSubModule::SubModule1 => crate::module::module::sub_module1::f(s),
            EnumSubModule::SubModule2 => crate::module::module::sub_module2::f(s),
        }
    }
    pub fn f_relative_path(e: EnumSubModule, mut s: String) { // this function is public
        s.push_str(" - relative path");
        match e {
            EnumSubModule::SubModule1 => sub_module1::f(s),
            EnumSubModule::SubModule2 => sub_module2::f(s),
        }
    }
}

pub fn f_absolute_path(e: module::EnumSubModule, s: String) { // this function is public
    module::f_absolute_path(e, s);
}
pub fn f_relative_path(e: module::EnumSubModule, s: String) { // this function is public
    module::f_relative_path(e, s);
}