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

/*- Make a vec of GLT:S -*/
#[allow(non_snake_case)]
fn make_GLT(inp:&String) -> Vec<GLT<>> {
    return inp.trim().split("").filter(|&x| !x.is_empty())
    .map(|s| {
        if s.parse::<u8>().is_ok() {
            GLT::Num(s.parse::<u8>().unwrap())
        } else {
            GLT::Str(s)
        }
    }).collect();
}

fn main() {

    /*- Possible words that will be outputted at the end -*/
    let mut possible:Vec<&str> = vec![];
    let mut final_words:Vec<&str> = vec![];

    /*- The "wordle" words -*/
    let words:Vec<String> = read_file(&"/Users/artur/Desktop/valid-wordle-words.txt");


    /*- All the yellow words, col 1 - X -*/
    let mut letters_yellow:Vec<Vec<String>> = Vec::new();

    /*- Loop through the columns and ask for input -*/
    for i in 0..5 {
        println!("Enter the yellow bricks in column {}", i+1);

        /*- Saves to this String -*/
        let mut input:String = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        /*- Add it to "column" <i> -*/
        letters_yellow.push(split(
            input
                .trim()
                .to_string()
        ));
    }

    println!("{:?}", letters_yellow);

    /*- Ask the user for green chars -*/
    println!("Enter the green chars - leave 0 for blank:");
    let mut green_input:String = String::new();
    stdin()
        .read_line(&mut green_input)
        .expect("Failed to read input!");

    /*- Split the input into a vector of GLT:s (either str or u8)  -*/
    let green_input_vec:Vec<GLT> = make_GLT(&green_input);

    /*- Ask the user for gray chars -*/
    println!("Enter the gray chars (No letter can be same as a yellow):");
    let mut gray_input:String = String::new();
    stdin()
        .read_line(&mut gray_input)
        .expect("Failed to read input!");

    gray_input = gray_input.trim().to_string();

    /*- The characters that are valid -*/
    let letters_gray:Vec<String> = split(gray_input);
    let letters_green:Vec<GLT> = green_input_vec;

    /*- Iterate over every word -*/
    for word in &words {

        /*- All wordle words are always 5 letters long -*/
        if word.len() != 5 { return };

        /*- Split the word into an array and loop through every character -*/
        let word_vec:Vec<String> = split(word.to_string());
        let mut contains_all_letters:bool<> = true;

        /*- Check if the word contains all letters of the input -*/
        for (column_index, letter_list) in letters_yellow.iter().enumerate() {

            /*- Iterate over all chars -*/
            for letter in letter_list {
                
                if !word_vec.contains(letter) {
                    contains_all_letters = false;
                    break;
                }else{
                    /*- Check if all letters are not in the same spot as the
                        yellow ones, because yellow means right but wrong spot -*/
                    if &word_vec[column_index] == letter {
                        contains_all_letters = false;
                        break;
                    };
                }
            }
        }if contains_all_letters {

            let mut gray_match:bool = false;

            /*- Check if it doesn't match any gray ones -*/
            for gray in &letters_gray {
                if word_vec.contains(gray) {
                    gray_match = true;
                    break;
                };
            }

            /*- If it does, add it to the possible words -*/
            if !gray_match { possible.push(word) };
        }else{ continue };
    }

    /*******/
    /*******/
    /*******/
    
    /*- Loop through all final words -*/
    for i in 0..possible.len() {

        /*- Loop through all confirmed letters -*/
        let mut word_is_possible:bool = true;

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
