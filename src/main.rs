// importing neccessary packages
use colored::*;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // i need to know what is collect here.

    // collecting arguments as a vector
    let arguments: Vec<String> = env::args().collect();

    // generating error if arguments length is less than 3, that mean either pattern or file path or both is not provided.
    if arguments.len() < 3 {
        println!(
            "please provide valid arguments, either pattern or file path or both are missing!"
        );
    }
    // if argument length is equal to 3 than it is one file and  we will go forward and run program
    else if arguments.len() >= 3 {
        // collected pattern from provided argument
        let pattern = &arguments[1];

        // initialising regex instance.
        let re = Regex::new(&pattern).unwrap();

        // running for loop for each file one by one
        for i in 2..arguments.len() {
            // so this is keep track of total number of count of matched patterns for each file.
            let mut match_count: i32 = 0;

            // we will assign refrence of each filepath argument to file path by it's index.
            let file_path = arguments.get(i).expect("incorrect file path is provided");

            // we will open the given file and handle error if file don not exists.
            let file = File::open(file_path).expect("file do not exists at given path");

            // we will read the given file with bufreader.
            let reader = BufReader::new(file);

            // displaying initial details and file number, i.e. detail of which file we are displaying currently.
            println!("\n--------------------------------------------------");
            println!("File no. {}", i - 1);
            println!("--------------------------------------------------");
            println!("Lines with matched patterns: ");
            println!("--------------------------------------------------\n");

            // running for loop on each line of given file after reading with bufreader, matching pattern here and also handling errors as well.
            for (line_index, line) in reader.lines().enumerate() {
                // handling error with Ok, Err
                match line {
                    Ok(content) => {
                        // i need to know why we pass ref of content.
                        // using regex object to match pattern with each line and displaying result with highlighting that pattern.
                        if re.is_match(&content) {
                            let highlight = re.replace_all(&content, |caps: &regex::Captures| {
                                caps[0].red().bold().to_string()
                            });
                            // index start from zero so while displaying line number of matched line i am using line_index + 1 to show actual line number of that line in that file.
                            println!("{}. {}", (line_index + 1), highlight);
                            match_count += 1;
                        }
                    }
                    // handling error in case error occured above, which i need to know exactly what.
                    Err(e) => {
                        eprintln!("error occured while reading file - {}, {}", file_path, e);
                        break;
                    }
                }
            }

            // displaying ending details with total number of pattern matches and file number in which we are matching pattern.
            println!("\n--------------------------------------------------");
            println!(
                "Total number of matched pattern in file - {}: {}",
                i - 1,
                match_count
            );
            println!("--------------------------------------------------\n");
        }
    }
}
