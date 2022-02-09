pub fn get_symbols(frames: &Vec<Vec<u32>>) -> Vec<[String; 3]> {
    let mut symbolss: Vec<[String; 3]> = Vec::new();

    for (i, throws) in frames.iter().enumerate() {
        let mut symbols: [String; 3] = [String::new(), String::new(), String::new()];

        if i != 9 {
            handle_non_strike(&mut symbols, throws, 0);
        } else {
            if throws[0] == 10 {
                symbols[0] = String::from("X");
                if throws.get(1).is_some() {
                    if throws[1] == 10 {
                        symbols[1] = String::from("X");
                        if throws.get(2).is_some() {
                            if throws[2] == 10 {
                                symbols[2] = String::from("X");
                            } else {
                                symbols[2] = handle_digit(throws[2]);
                            }
                        }
                    } else {
                        handle_non_strike(&mut symbols, throws, 1);
                    }
                }
            } else {
                let scored_ten = handle_non_strike(&mut symbols, throws, 0);
                if throws.get(2).is_some() && scored_ten {
                    if throws[2] == 10 {
                        symbols[2] = String::from("X");
                    } else {
                        symbols[2] = handle_digit(throws[2]);
                    }
                }
            }
        }

        symbolss.push(symbols)
    }

    symbolss
}

fn handle_non_strike(frame_symbols: &mut [String; 3], throws: &Vec<u32>, i: usize) -> bool {
    frame_symbols[i] = handle_digit(throws[i]);
    if throws.get(i + 1).is_some() {
        if throws[i] + throws[i + 1] == 10 {
            frame_symbols[i + 1] = String::from("/");
            true
        } else {
            frame_symbols[i + 1] = handle_digit(throws[i + 1]);
            false
        }
    } else {
        throws[0] == 10
    }
}

fn handle_digit(digit: u32) -> String {
    match digit {
        0 => String::from("-"),
        10 => String::from("X"),
        _ => digit.to_string(),
    }
}
