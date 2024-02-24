use std::fs;
use std::error::Error;

pub fn grep_file(file_name: &str, grep: &str, sensitive: bool) -> Result<Vec<String>, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_name)?;
    let lines: Vec<&str> = file_content.lines().collect();
    let mut result: Vec<String> = Vec::new();
    for line in lines {
        let _line_string = line.to_string();
        if grep.len() == 0 {
            result.push(_line_string);
        }
        else {
            let mut push = false;
            if sensitive {
                push = line.to_lowercase().to_string().contains(&grep.to_lowercase());
            }
            else {
                push = line.to_string().contains(grep);
            }
            if push { result.push(_line_string); }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_exist() {
        match grep_file("test.txt", "", false) {
            Ok(_) => {},
            Err(e) => { panic!("{}", e) },
        }
    }

    #[test]
    #[should_panic]
    fn test_file_not_exist() {
        match grep_file("test_not_exist.txt", "", false) {
            Ok(_) => {},
            Err(e) => { panic!("{}", e) },
        }
    }

    #[test]
    fn test_file_empty() {
        match grep_file("test.txt", "", false) {
            Ok(result) => { assert_eq!(result.len(), 0); },
            Err(e) => { panic!("{}", e) },
        }
    }

    #[test]
    fn test_file_one_line() {
        match grep_file("test2.txt", "", false) {
            Ok(result) => { assert_eq!(result.len(), 1); },
            Err(e) => { panic!("{}", e) },
        }
    }

    #[test]
    fn test_file_eof() {
        match grep_file("test3.txt", "", false) {
            Ok(result) => { assert_eq!(result.len(), 3); },
            Err(e) => { panic!("{}", e) },
        }
    }

    #[test]
    fn test_file_grep_insensitive_1() {
        match grep_file("test4.txt", "miner", false) {
            Ok(result) => { assert_eq!(result.len(), 2); },
            Err(e) => { panic!("{}", e) },
        }
    }

    #[test]
    fn test_file_grep_insensitive_2() {
        match grep_file("test4.txt", "alva", false) {
            Ok(result) => { assert_eq!(result.len(), 2); },
            Err(e) => { panic!("{}", e) },
        }
    }

    #[test]
    fn test_file_grep_sensitive_1() {
        match grep_file("test4.txt", "miner", true) {
            Ok(result) => { assert_eq!(result.len(), 3); },
            Err(e) => { panic!("{}", e) },
        }
    }

    #[test]
    fn test_file_grep_sensitive_2() {
        match grep_file("test4.txt", "alva", true) {
            Ok(result) => { assert_eq!(result.len(), 3); },
            Err(e) => { panic!("{}", e) },
        }
    }
}