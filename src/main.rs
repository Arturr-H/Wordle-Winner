use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::io::stdin;
// use std::collections::HashMap;

/*- Read files and get data back using only one function -*/
fn read_file(_filename: &str) -> Vec<String> {

    /*- Open the file from the input path -*/
    let file:File = File::open(_filename)
        .expect("Path not found!");
    
    /*- Save the file into a buffer for efficiency -*/
    let mut buffer:BufReader<std::fs::File> = BufReader::new(file);

    /*- Where the file data later should be saved -*/
    let mut file_content:String = String::new();

    /*- Get the file content as a string and store it in <file_content> -*/
    buffer.read_to_string(&mut file_content)
        .expect("Failed to read file!");

    /*- Split the file content into a vector of strings -*/
    let file_content_vec:Vec<String> = file_content.split("\n")
        .map(|s| s.to_string())
        .collect();

    /*- Return the vector -*/
    return file_content_vec;
}

/*- For quickly splitting strings -*/
fn split(stri:String) -> Vec<String> {

    let mut stri_vec:Vec<String> = stri.split("")
        .map(|s| s.to_string())
        .collect();

    stri_vec.pop();
    stri_vec.remove(0);

    /*- Return the vector -*/
    return stri_vec;
}

/*- Need this because we need a vector of both u8 and strings -*/
#[derive(Debug)]
enum GLT<'tpe> { /*- GLT = Green letter type -*/
    Num(u8),
    Str(&'tpe str)
}

fn main() {

    /*- Possible words that will be outputted at the end -*/
    let mut possible:Vec<&str> = vec![];
    let mut final_words:Vec<&str> = vec![];

    /*- The "wordle" words -*/
    let words:Vec<String> = read_file(&"/Users/artur/Desktop/valid-wordle-words.txt");

    /*- Ask the user for yellow chars -*/
    println!("Enter the yellow chars:");
    let mut yellow_input:String = String::new();
    stdin()
        .read_line(&mut yellow_input)
        .expect("Failed to read input!");

    yellow_input = yellow_input.trim().to_string();

    /*- Ask the user for green chars -*/
    println!("Enter the green chars - leave 0 for blank:");
    let mut green_input:String = String::new();
    stdin()
        .read_line(&mut green_input)
        .expect("Failed to read input!");

    /*- Split the input into a vector of GLT:s (either str or u8)  -*/
    let green_input_vec:Vec<GLT> = green_input.trim().split("").filter(|&x| !x.is_empty())
        .map(|s| {
            if s.parse::<u8>().is_ok() {
                GLT::Num(s.parse::<u8>().unwrap())
            } else {
                GLT::Str(s)
            }
        })
        .collect();

    /*- The characters that are valid -*/
    let letters_yellow:Vec<String> = split(yellow_input);
    let letters_green:Vec<GLT> = green_input_vec;

    /*- Iterate over every word -*/
    for word in &words {

        /*- All wordle words are always 5 letters long -*/
        if word.len() != 5 { return };

        /*- Split the word into an array and loop through every character -*/
        let word_vec:Vec<String> = split(word.to_string());
        let mut contains_all_letters:bool<> = true;

        /*- Check if the word contains all letters of the input -*/
        for letter in &letters_yellow {
            if !word_vec.contains(&letter) {
                contains_all_letters = false;
                break;
            }
        }if contains_all_letters {
            /*- If it does, add it to the possible words -*/
            possible.push(word);
        }else{ continue };
    }

    /*******/
    /*******/
    /*******/
    
    /*- Loop through all final words -*/
    for i in 0..possible.len() {

        /*- Loop through all confirmed letters -*/
        let mut word_is_possible:bool = false;

        /*- Loop through the words chars and extract the index alongside it -*/
        for (i, letter) in split(possible[i].to_string()).iter().enumerate() {

            /*- Match the enum states and check wether its a 0 (no char specified) or a str -*/
            match letters_green[i] {
                GLT::Num(0) => { continue },
                GLT::Str(s) => {
                    
                    /*- If any letter didn't match (word is now invalid) -*/
                    if letter != &s.to_lowercase() {
                        word_is_possible = false;
                        break;
                    }else {
                        word_is_possible = true;
                    }
                }
                _ => { panic!("Oopppsss, somethign went wrong...") },
            }
        }

        /*- If the word is possible, add it to the possible words -*/
        if word_is_possible == true {
            final_words.push(possible[i]);
        }
    };

    /*- Print the possible words -*/
    for word in &final_words {
        println!("-> {}", word);
    }

    println!("Possible words: {}", final_words.len());
}
