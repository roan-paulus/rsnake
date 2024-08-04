fn main() -> crossterm::Result<()> {
    match snake::run() {
        Ok(_) => println!("Exitting..."),
        Err(e) => eprintln!("{e}"),
    };

    Ok(())
}
