
// TODO: Track calculation time
#![feature(core_panic)]

extern crate core;

use core::panicking::panic;
use std::fmt::format;
use std::io;
use ansi_term::Color::*;
use std::fs::{File, write};
use std::io::{Write, BufReader, BufRead, Error};

fn main() {

    // get starting number for prime calculation
    println!("{}",
        Blue.bold().paint("Please enter starting number for calculation:"),
    );

    let _calc_min: f64 = get_usr_input_f64();

    // get ending number for prime calculation
    println!("{}",
             Blue.bold().paint("Please enter ending number for calculation:"),
    );

    let _calc_max: f64 = get_usr_input_f64();

    // run prime calculation
    prime_calc(_calc_min, _calc_max);
}

fn prime_calc(_calc_min: f64, _calc_max: f64) {

    // TODO: check if _calc_min is round -> decrease by 1
    // TODO: check if _calc_max is round -> increase by 1
    // TODO: Write prime numbers to file

    create_output_file();

    // initializing first number being checked
    let mut _current_calc_number: f64 = _calc_min;

    // actual prime number calculation loop
    while _current_calc_number < _calc_max {

        // initialize divider to perform calculations
        let mut _current_dividing_number: f64 = 3.0;

        // run until you divide through _current_calc_number - 1
        'division_check: while _current_calc_number > _current_dividing_number {

            // divide current number with current divider
            let _current_division_result: f64 = _current_calc_number / _current_dividing_number;

            // if number has no decimal points it was divisible by the divider and thus cannot be a prime number
            if _current_division_result.fract() == 0.0 {
                // println!("{} is roundly divisable by {}. Not a prime number!", _current_calc_number, _current_dividing_number);
                break 'division_check;
            }

            // if number has decimal points it isn't roundly divisible by the divider and thus additional calculations are needed
            else if _current_division_result.fract() != 0.0 {
                // println!("{} is NOT roundly divisable by {}. Checking next divider!", _current_calc_number, _current_dividing_number);
                _current_dividing_number += 2.0;
            }
        }

        if _current_dividing_number >= _current_calc_number {
            println!("{} {}",
                _current_calc_number,
                Cyan.bold().paint("is a prime number!")
            );

            write_to_file(_current_calc_number);
        }

        _current_calc_number += 2.0;

    }
}

fn create_output_file() {
    let file_name = "prime_list.txt";
    let output = File::create(file_name);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error);
        },
    };
}

fn write_to_file(input: f64) {
    //TODO: figure out how to write to a file (new line each time)

}

// return user input from console as f64 (to reduce code duplication)
fn get_usr_input_f64() -> f64 {

    let _usr_input: f64 = loop {

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input: f64 = input.trim().parse().unwrap();

        return input
    };
}