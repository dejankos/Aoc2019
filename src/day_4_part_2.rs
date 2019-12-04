use crate::day_4_part_1::I32Extension;

pub fn diff_passwords_improved(from: u32, to: u32) -> u32 {
    let mut count: u32 = 0;

    for i in from..to {
        let digits: Vec<u32> = i.to_digits();

        let mut same_digits: [u32; 10] = [1; 10];
        let mut increase = true;
        for (i, d) in digits.iter().enumerate() {
            if i + 1 < digits.len() && increase {
                if digits[i + 1] < *d {
                    increase = false;
                }

                if digits[i + 1] == *d {
                    same_digits[*d as usize] += 1;
                }
            }
        }

        if increase && has_two_same(same_digits) {
            count += 1;
        }
    }

    count
}

fn has_two_same(same_digits: [u32; 10]) -> bool {
    for i in 0..same_digits.len() {
        if same_digits[i] == 2 {
            return true;
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_diff_passwords() {
        assert_eq!(diff_passwords_improved(112233, 112234), 1);
        assert_eq!(diff_passwords_improved(123444, 123445), 0);
        assert_eq!(diff_passwords_improved(111122, 111123), 1);
    }
}