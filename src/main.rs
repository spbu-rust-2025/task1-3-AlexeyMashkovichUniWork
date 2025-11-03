use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let file_open_result = File::open(buffer);
    let mut file;
    match file_open_result {
        Err(_) => {
            println!("failure");
            return Ok(());
        }
        Ok(result) => {
            file = result;
        }
    };
    let mut buf: Vec<_> = Vec::new();
    match file.read(&mut buf) {
        Err(_) => {
            println!("failure");
        }
        Ok(_) => {
            println!("success");
        }
    }

    Ok(())
}
