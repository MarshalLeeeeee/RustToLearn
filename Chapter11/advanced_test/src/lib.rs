pub mod test_module;

#[cfg(test)]
mod test_lib {
    #[test]
    fn test_add_for_succ() {
        println!("test_add_for_succ");
        let result = crate::test_module::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn test_add_for_fail() {
        println!("test_add_for_fail");
        let result = crate::test_module::add_wrong(2, 2);
        assert_eq!(result, 4);
    }
}
