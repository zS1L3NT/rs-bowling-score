pub fn get_subtotal(frames: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut subtotals: Vec<u32> = Vec::new();

    for (i, frame) in frames.iter().enumerate() {
        let frame_raw: u32 = frame.iter().sum();

        let previous_frame = match i {
            0 => 0,
            i => subtotals[i - 1],
        };

        let current_frame = if i != 9 {
            if frame_raw == 10 {
                if frame.len() == 1 {
                    10 + get_next_two_throws(&frames, i)
                } else {
                    10 + get_next_throw(&frames, i)
                }
            } else {
                frame_raw
            }
        } else {
            frame_raw
        };

        subtotals.push(previous_frame + current_frame);
    }

    subtotals
}

fn get_next_throw(frames: &Vec<Vec<u32>>, i: usize) -> u32 {
    let mut score: u32 = 0;

    if frames.get(i + 1).is_some() {
        score += frames[i + 1][0]
    }

    score
}

fn get_next_two_throws(frames: &Vec<Vec<u32>>, i: usize) -> u32 {
    let mut score: u32 = 0;

    if frames.get(i + 1).is_some() {
        score += frames[i + 1][0];
        if frames[i + 1].get(1).is_some() {
            score += frames[i + 1][1]
        } else if frames[i + 1].get(2).is_some() {
            score += frames[i + 1][2]
        } else if frames.get(i + 2).is_some() {
            score += frames[i + 2][0]
        }
    }

    score
}
