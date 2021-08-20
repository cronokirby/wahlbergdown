use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    // Discard program name
    args.next();
    let path = args.next().expect("Please give me a file path to run");
    let src = fs::read_to_string(path)?;
    print!("{}", src);
    Ok(())
}
