use std::io::Write;

pub fn get_input(text: Option<String>) -> Result<String, std::io::Error> {
    let mut input = String::new();
    if text.is_some() {
        print!("{}", text.unwrap_or_default());
    }
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}
