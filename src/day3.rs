use std::{fs, path::PathBuf};

fn load_input_file(filename: &str) -> String {
    fs::read_to_string(PathBuf::from(filename)).expect("Could not read file")
}

fn most_common_at_pos(readings: &Vec<u32>, reading_len: usize, pos: usize) -> u32 {
    let mut counts: Vec<u32> = vec![0; reading_len];
    for reading in readings {
        for idx in 0..reading_len {
            let shift = reading_len - idx - 1;
            let flag = 1 << shift;
            counts[idx] += (reading & flag) >> shift;
        }
    }
    if counts[pos] > (&readings.len() / 2) as u32 {
        1
    } else if counts[pos] < (&readings.len() / 2) as u32 {
        0
    } else {
        // Tiebreaker, 1 always wins
        if (&readings.len() % 2) == 0 {
            1
        } else {
            0
        }
    }
}

pub fn solve_1() -> u32 {
    let file = load_input_file("src/input/day3.txt");
    let input: Vec<u32> = file
        .lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();
    let reading_len = file.lines().collect::<Vec<&str>>()[0].len();

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for idx in 0..reading_len {
        if most_common_at_pos(&input, reading_len, idx) == 1 {
            gamma += u32::pow(2, (reading_len - idx - 1) as u32);
        } else {
            epsilon += u32::pow(2, (reading_len - idx - 1) as u32);
        }
    }
    gamma * epsilon
}

pub fn solve_2() -> u32 {
    let file = load_input_file("src/input/day3.txt");
    let input: Vec<u32> = file
        .lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();
    let reading_len = file.lines().collect::<Vec<&str>>()[0].len();

    let mut o2_rating: u32 = 0;
    let mut co2_rating: u32 = 0;

    // Find O2 rating
    //
    // Start with all readings being valid
    let mut valid_o2_readings: Vec<bool> = vec![true; input.len()];

    for idx in 0..reading_len {
        if o2_rating > 0 {
            break;
        }
        // Start with all input readings and filter out readings we've marked as invalid
        let mut filtered_readings = vec![];
        for (i, &val) in input.iter().enumerate() {
            if valid_o2_readings[i] {
                filtered_readings.push(val);
            }
        }

        // Determine the most common bit at position idx from filtered readings
        let most_common_bit = most_common_at_pos(&filtered_readings, reading_len, idx);

        let mut remaining_readings = filtered_readings.len();

        // Mark invalid any readings that don't have most common bit at position idx
        for (i, &val) in input.iter().enumerate() {
            if valid_o2_readings[i] {
                if remaining_readings == 1 {
                    o2_rating = val;
                    break;
                }

                let shift = reading_len - idx - 1;
                let flag = 1 << shift;
                if (val & flag) >> shift != most_common_bit {
                    remaining_readings -= 1;
                    valid_o2_readings[i] = false;
                }
            }
        }
    }

    // Find CO2 rating
    //
    // Start with all readings being valid
    let mut valid_co2_readings: Vec<bool> = vec![true; input.len()];

    for idx in 0..reading_len {
        if co2_rating > 0 {
            break;
        }

        // Start with all input readings and filter out readings we've marked as invalid
        let mut filtered_readings = vec![];
        for (i, &val) in input.iter().enumerate() {
            if valid_co2_readings[i] {
                filtered_readings.push(val);
            }
        }

        // Determine the least common bit at position idx from filtered readings
        let most_common_bit = most_common_at_pos(&filtered_readings, reading_len, idx);
        let least_common_bit = if most_common_bit == 1 { 0 } else { 1 };
        let mut remaining_readings = filtered_readings.len();
        // Mark invalid any readings that don't have least common bit at position idx
        for (i, &val) in input.iter().enumerate() {
            if valid_co2_readings[i] {
                if remaining_readings == 1 {
                    co2_rating = val;
                    break;
                }

                let shift = reading_len - idx - 1;
                let flag = 1 << shift;
                if (val & flag) >> shift != least_common_bit {
                    remaining_readings -= 1;
                    valid_co2_readings[i] = false;
                }
            }
        }
    }

    o2_rating * co2_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_common_at_pos_works() {
        let vecs: Vec<u32> = vec![0b0011, 0b0011, 0b1001, 0b0101];

        assert_eq!(most_common_at_pos(&vecs, 4, 0), 0);
        assert_eq!(most_common_at_pos(&vecs, 4, 1), 0);
        assert_eq!(most_common_at_pos(&vecs, 4, 2), 1);
        assert_eq!(most_common_at_pos(&vecs, 4, 3), 1);
    }

    #[test]
    fn most_common_at_pos_with_odd_size_vec() {
        let vecs: Vec<u32> = vec![0b1101, 0b0111, 0b0011];

        assert_eq!(most_common_at_pos(&vecs, 4, 0), 0);
        assert_eq!(most_common_at_pos(&vecs, 4, 1), 1);
        assert_eq!(most_common_at_pos(&vecs, 4, 2), 1);
        assert_eq!(most_common_at_pos(&vecs, 4, 3), 1);
    }
}
