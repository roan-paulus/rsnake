fn main() -> crossterm::Result<()> {
    match snake::run() {
        Ok(message) => println!("{message}"),
        Err(e) => eprintln!("{e}"),
    };

    Ok(())
}
