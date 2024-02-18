pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_wrong(a: i32, b: i32) -> i32 {
    a + b + 1
}

pub fn eq(a: i32, b: i32) -> bool {
    a == b
}

pub fn eq_wrong(a: i32, b: i32) -> bool {
    a != b
}

#[cfg(test)]
mod test_test_module {
    use super::*;

    #[test]
    fn test_add_for_succ_in_module() {
        println!("test_add_for_succ_in_module");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn test_add_for_fail_in_module() {
        println!("test_add_for_fail_in_module");
        let result = add_wrong(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_eq_for_succ_in_module() {
        println!("test_eq_for_succ_in_module");
        assert!(eq(2, 2));
    }
    
    #[test]
    #[ignore]
    fn test_eq_for_fail_in_module() {
        println!("test_eq_for_fail_in_module");
        assert!(eq_wrong(2, 2));
    }
}