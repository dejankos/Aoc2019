use std::process::id;

#[derive(Debug)]
pub struct DataStats {
    _0: u32,
    _1: u32,
    _2: u32,
}

pub fn decode_image(layers: Vec<&str>) -> Vec<u32> {
    let x = layers[0].len();
    let y = layers.len();

    let mut x_iter = 0;
    let mut decoded: Vec<u32> = Vec::new();

    while x_iter < x {
        let mut y_iter = 0;
        let mut last_color = 2;

        while y_iter < y {
            let s = layers[y_iter];
            let color = s.chars().nth(x_iter).unwrap().to_string().parse::<u32>().unwrap();

            match color {
                0 | 1=> {
                    if last_color == 2 {
                        last_color = color;
                    }
                }
                _ => {}
            }

            y_iter += 1;
        }
        decoded.push(last_color);
        x_iter += 1;
    }

    decoded
}

pub fn calc_pass(layers: Vec<&str>) -> u32 {
    let mut min_0 = DataStats { _0: u32::max_value(), _1: 0, _2: 0 };

    layers.iter()
        .for_each(|slice| {
            let ds = to_data_stats(slice);
            if ds._0 < min_0._0 {
                min_0 = ds;
            }
        });

    min_0._1 * min_0._2
}

pub fn print_image(image: Vec<u32>) {
    for (idx, val) in image.iter().enumerate() {
        if idx % 25 == 0 {
            println!(" ")
        }
        let mut p = '0';
        match val {
            0 => p = '\u{2592}',
            1 => p = '\u{2593}',
            2 => p = '\u{2591}',
            _ => {},
        }
        print!("{}", p);
    }
}

fn to_data_stats(slice: &str) -> DataStats {
    let mut _0 = 0;
    let mut _1 = 0;
    let mut _2 = 0;
    for s in slice.chars() {
        let i = s.to_string().parse::<u32>().unwrap();
        match i {
            0 => {
                _0 += 1;
            }
            1 => {
                _1 += 1;
            }
            2 => {
                _2 += 1;
            }
            _ => {}
        }
    }

    DataStats {
        _0,
        _1,
        _2,
    }
}


#[cfg(test)]
mod tests {
    use crate::day_8_data::{parse_input, RAW_DATA, RAW_DATA_TEST, RAW_DATA_TEST_2};

    use super::*;

    #[test]
    fn should_calc_pass() {
        let data = parse_input(2, 3, RAW_DATA_TEST);
        let pass = calc_pass(data);
        assert_eq!(1, pass);
    }

    #[test]
    fn should_calc_pass_1() {
        let data = parse_input(25, 6, RAW_DATA);
        let pass = calc_pass(data);
        assert_eq!(1452, pass)
    }

    #[test]
    fn should_decode_image() {
        let data = parse_input(2, 2, RAW_DATA_TEST_2);
        let decoded = decode_image(data);

        assert_eq!(vec![0, 1, 1, 0], decoded)
    }

    #[test]
    fn should_decode_image_2() {
        let data = parse_input(25, 6, RAW_DATA);
        let decoded = decode_image(data);

        print_image(decoded);
    }
}
