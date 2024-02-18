pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn add_wrong(left: i32, right: i32) -> i32 {
    left + right + 1
}

pub fn gt(left: i32, right: i32) -> bool {
    left > right
}

// functions under the scope of tests are automatically tested via ```cargo test```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_test() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn neg_test() { // more test_fns
        let result = add(-2, -2);
        assert_eq!(result, -4);
    }

    #[test]
    fn gt_test() {
        assert!(gt(2, 1));
        assert!(!gt(1, 2));
    }

    #[test]
    fn fail_test() {
        let result = add_wrong(2, 2);
        assert_eq!(result, 4, "The result should be {}", 4); // assertion with msg
    }

    #[test]
    #[should_panic(expected = "[Mismatch]")] // use #[should_panic] to expect a fail with substring match
    fn should_fail_test() {
        let result = add_wrong(2, 2);
        assert_eq!(result, 4, "[Mismatch] The result should be {}", 4); // assertion with msg
    }

    #[test]
    fn result_fail_test() -> Result<(), String> {
        if add_wrong(2, 2) == 4 {
            Ok(())
        }
        else {
            Err("[Deliberate fail]: Result_test".to_string())
        }
    }

    #[test]
    fn result_should_fail_test() -> Result<(), String> {
        let result = {
            if add_wrong(2, 2) == 4 {
                Ok(())
            }
            else {
                Err("[Deliberate fail]: Result_test".to_string())
            }
        };
        assert!(result.is_err()); // manual assert result to match Err to implement #[should_panic]
        Ok(())
    }
}
