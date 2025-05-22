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
        println!("please provide valid arguments, pattern or file path is missing!");
    }
    // if argument length is equal to 3 than it is one file and  we will go forward and run program
    else if arguments.len() >= 3 {
        // collected pattern from provided argument
        let pattern = &arguments[1];

        // initialising regex instance.
        let re = Regex::new(&pattern).unwrap();

        for i in 2..arguments.len() {
            // so this is keep track of total number of count of matched patterns for each file.
            let mut match_count: i32 = 0;

            // we will assign referece to file path
            let file_path = arguments.get(i).expect("incorrect file path is provided");

            // we will open the given file and handle error if file don not exists.
            let file = File::open(file_path).expect("file do not exists at given path");

            // we will read the given file with bufreader.
            let reader = BufReader::new(file);

            println!("\n--------------------------------------------------");
            println!("File no. {}", i - 1);
            println!("--------------------------------------------------");
            println!("Lines with matched patterns: ");
            println!("--------------------------------------------------\n");

            for (line_index, line) in reader.lines().enumerate() {
                match line {
                    Ok(content) => {
                        // i need to know why we pass ref of content.
                        if re.is_match(&content) {
                            let highlight = re.replace_all(&content, |caps: &regex::Captures| {
                                caps[0].red().bold().to_string()
                            });
                            // here i am doing this line_index + 1 to to_string, so i need to know whether is it correct and what is the things happen under the hood.
                            println!("{}. {}", (line_index + 1), highlight);
                            match_count += 1;
                        }
                    }
                    Err(e) => {
                        eprintln!("error occured while reading file - {}, {}", file_path, e);
                        break;
                    }
                }
            }

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
