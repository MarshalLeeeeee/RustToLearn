use minigrep;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_parse = minigrep::file_arg_parser::FileParse::parse()?;
    println!("{:?}", minigrep::file_greper::grep_file(
        &file_parse.name,
        &file_parse.grep,
        file_parse.sensitive,
    )?);
    Ok(())
}
