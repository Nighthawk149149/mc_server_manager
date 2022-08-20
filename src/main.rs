use std::io;
use std::io::Write;

// Clears the screen and sets the cursor to the top left corner
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

// Prints the menu and returns the choice as a String result
fn prompt_and_get_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    // Takes input from the user and returns the choice as an integer
    let input = loop {
        match prompt_and_get_input("1. Open server folder\n2. Create new server\n3. Exit the program\n\nEnter a valid option: ") {
                Ok(input) => {
                    match input.trim().parse::<u32>() {
                        Ok(input) => break input,
                        Err(error) => {
                            clear_screen();
                            println!("Please try again: {}", error);
                            continue;
                        }
                    }
                },
                Err(error) => {
                    clear_screen();
                    println!("Please try again: {}", error);
                    continue;
                }
            }
    };

    println!("Input: {}", input);
}
