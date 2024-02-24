use std::env;

fn env_var(s: &str) {
    print!("{s}");
    print!(" = ");
    if let Ok(env_var) = env::var(s) {
        println!("{:?}", env_var);
    }
    else {
        println!("Not present");
    }
}

fn test_env_var() {
    env_var("TEST_ENV_VAR");
    env_var("TEST_ENV_VAR_2");
}

fn main() {
    test_env_var();
}
