#[macro_use]
extern crate prettytable;

use std::io::stdout;
use std::io::{stdin, Write};

use prettytable::{Row, Table};

use get_subtotal::get_subtotal;
use get_symbols::get_symbols;

mod get_subtotal;
mod get_symbols;

fn main() {
    let mut frames: Vec<Vec<u32>> = Vec::new();

    for frame_num in 1..11 {
        if frames.get(frame_num - 1).is_none() {
            frames.push(Vec::new());
        }

        let mut throw_num: u32 = 1;

        println!();
        loop {
            let frame = &mut frames[frame_num - 1];

            let input = request_input(&*format!(
                "Enter score for frame {0} throw {1}: ",
                frame_num, throw_num
            ));

            match input.trim().parse::<u32>() {
                Ok(throw) => {
                    if throw > 10 {
                        println!("Throw cannot exceed 10 pins");
                        continue;
                    }

                    match throw_num {
                        1 => {
                            frame.push(throw);
                            if throw == 10 && frame_num != 10 {
                                break;
                            } else {
                                throw_num += 1;
                            }
                        }
                        2 => {
                            frame.push(throw);

                            let first_two_throws = frame[0] + throw;
                            if frame_num == 10 && frame[0] == 10 {
                                throw_num += 1;
                            } else {
                                if first_two_throws > 10 {
                                    println!("First and second throw cannot exceed 10");
                                    frame.pop();
                                    continue;
                                } else if frame_num == 10 && first_two_throws == 10 {
                                    throw_num += 1;
                                } else {
                                    break;
                                }
                            }
                        }
                        3 => {
                            frame.push(throw);
                            break;
                        }
                        _ => {}
                    }

                    draw_table(&frames);
                }
                Err(_) => println!("Throw must be a number!"),
            }
        }

        draw_table(&frames);
    }
}

fn draw_table(frames: &Vec<Vec<u32>>) {
    let mut table = Table::new();

    let mut header = Row::new(Vec::new());
    let mut symbolss = Row::new(Vec::new());
    let mut subtotals = Row::new(Vec::new());

    for i in 0..frames.len() {
        header.add_cell(cell!(format!("[{}]", i + 1)));
    }

    for symbols in get_symbols(frames) {
        let mut table = Table::new();

        if symbols[1] == "" {
            table.add_row(row![symbols[0]]);
        } else if symbols[2] == "" {
            table.add_row(row![symbols[0], symbols[1]]);
        } else {
            table.add_row(row![symbols[0], symbols[1], symbols[2]]);
        }

        symbolss.add_cell(cell!(table));
    }

    for subtotal in get_subtotal(frames) {
        subtotals.add_cell(cell!(subtotal));
    }

    table.add_row(header);
    table.add_row(symbolss);
    table.add_row(subtotals);

    table.printstd();
}

fn request_input(message: &str) -> String {
    print!("{}", message);
    stdout().flush().expect("Failed to flush input");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    input
}
