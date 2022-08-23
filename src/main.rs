use std::fs;
use std::io;
use std::io::Write;

fn main() {
    // Check if mcsm_data folder exists
    if fs::metadata("mcsm_data").is_err() {
        fs::create_dir("mcsm_data").expect("Couldn't create mcsm_data folder");
    }

    // Takes input from the user and returns the choice as an integer
    clear_screen();
    println!("[ MCSM | Home ]");
    let input = loop {
        // Get input
        match prompt_and_get_input("1. Open server folder\n2. Create new server\n3. Exit the program\n\nEnter a valid option: ") {
            Ok(input) => {
                // Get int from input
                match input.trim().parse::<u8>() {
                    Ok(input) => {
                        if (1..=3).contains(&input) {
                            break input;
                        } else {
                            clear_screen();
                            println!("Please try again: invalid input");
                            continue;
                        }
                    },
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

    // Take action based on the input
    match input {
        // Open server folder
        1 => {
            clear_screen();
            println!("[ MCSM | Servers ]");
            // List the contents of the current folder and save it
            let contents = fs::read_dir("mcsm_data")
                .expect("Couldn't read mcsm_data folder")
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        e.path()
                            .file_name()
                            .and_then(|n| n.to_str().map(|n| n.to_string()))
                    })
                })
                .collect::<Vec<String>>();
            contents.iter().for_each(|f| println!("> {}", f));
        }
        // This will never be reached
        _ => (),
    }
}

// Clears the screen and sets the cursor to the top left corner
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

// Prints the menu and returns the choice as a String result
fn prompt_and_get_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush().expect("Couldn't flush stdout");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}
