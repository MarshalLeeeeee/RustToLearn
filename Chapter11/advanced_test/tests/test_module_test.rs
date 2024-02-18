use advanced_test;

#[test]
fn test_add_for_succ_in_dir() {
    println!("test_add_for_succ_in_dir");
    let result = advanced_test::test_module::add(2, 2);
    assert_eq!(result, 4);
}

#[test]
#[ignore]
fn test_add_for_fail_in_dir() {
    println!("test_add_for_fail_in_dir");
    let result = advanced_test::test_module::add_wrong(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn test_eq_for_succ_in_dir() {
    println!("test_eq_for_succ_in_dir");
    assert!(advanced_test::test_module::eq(2, 2));
}

#[test]
#[ignore]
fn test_eq_for_fail_in_dir() {
    println!("test_eq_for_fail_in_dir");
    assert!(advanced_test::test_module::eq_wrong(2, 2));
}