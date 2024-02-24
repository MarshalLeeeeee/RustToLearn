use std::env;
use std::error::Error;

pub struct FileParse {
    pub name: String,
    pub grep: String,
    pub sensitive: bool,
}

impl FileParse {
    pub fn parse() -> Result<FileParse, Box<dyn Error>> {
        let args = env::args().collect();
        Self::_do_parse_cmd(args)
    }
    
    fn _do_parse_cmd(args: Vec<String>) -> Result<FileParse, Box<dyn Error>> {
        let name = args.get(1).ok_or("File name missing")?;
        let grep = args.get(2).ok_or("File grep missing")?;
        Ok(FileParse {
            name: name.clone(),
            grep: grep.clone(),
            sensitive: Self::_do_parse_env_var(),
        })
    }

    fn _do_parse_env_var() -> bool {
        let env_var = env::var("FILE_GREP_SENSITIVE").unwrap_or("0".to_string());
        env_var != "0"
    }

    pub fn print_self(&self) {
        println!("name: {}, grep: {}, sensitive: {}", self.name, self.grep, self.sensitive);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    // test for exactly two args
    fn test1() {
        let args = vec![String::from(""), String::from("test.txt"), String::from("test")];
        match FileParse::_do_parse_cmd(args) {
            Ok(file_parse) => {
                assert_eq!(file_parse.name, "test.txt");
                assert_eq!(file_parse.grep, "test");
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
    
    #[test]
    // test for more than two args
    fn test2() {
        let args = vec![String::from(""), String::from("test.txt"), String::from("test"), String::from("ignores")];
        match FileParse::_do_parse_cmd(args) {
            Ok(file_parse) => {
                assert_eq!(file_parse.name, "test.txt");
                assert_eq!(file_parse.grep, "test");
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
    
    #[test]
    #[should_panic(expected = "File grep missing")]
    // test for more than one arg
    fn test3() {
        let args = vec![String::from(""), String::from("test.txt")];
        match FileParse::_do_parse_cmd(args) {
            Ok(file_parse) => {
                assert_eq!(file_parse.name, "test.txt");
                assert_eq!(file_parse.grep, "");
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
    
    #[test]
    #[should_panic(expected = "File name missing")]
    // test for more than no arg
    fn test4() {
        let args = vec![String::from("")];
        match FileParse::_do_parse_cmd(args) {
            Ok(file_parse) => {
                assert_eq!(file_parse.name, "test.txt");
                assert_eq!(file_parse.grep, "");
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
