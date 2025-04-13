use clap::{Arg, ArgAction, Command}; // Import ArgAction for boolean flags

fn main() {
    // Get command line arguments
    let matches = Command::new("echor")
        .version("1.2.3")
        .author("abhilov gupta <abhilovgupta01@gmail.com>")
        .about("rust echo")
        .arg(
            Arg::new("text") // First argument: entered as argument
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..), // At least one argument required
        )
        .arg(
            Arg::new("omit_newline") // Second argument: created as an option
                .help("Do not print newline")
                .short('n') // Short flag
                .action(ArgAction::SetTrue), // Correct way to handle boolean flags
        )
        .get_matches(); // If nothing matches, it will display an error.
    
    // Get text argument
    let text: Vec<String> = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    // Check if omit_newline flag is present
    let omit_newline = matches.get_flag("omit_newline"); // Returns `true` if flag is present
    
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
