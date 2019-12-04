pub trait I32Extension {
    fn to_digits(&self) -> Vec<u32>;
}

impl I32Extension for u32 {
    fn to_digits(&self) -> Vec<u32> {
        self.to_string()
            .chars()
            .map(|c| c.to_string().parse::<u32>().unwrap())
            .collect()
    }
}

pub fn diff_passwords(from: u32, to: u32) -> u32 {
    let mut count: u32 = 0;

    for i in from..to {
        let digits: Vec<u32> = i.to_digits();

        let mut two_same = false;
        let mut increase = true;
        for (i, d) in digits.iter().enumerate() {
            if i + 1 < digits.len() && increase {
                if digits[i + 1] < *d {
                    increase = false;
                }

                if digits[i + 1] == *d {
                    two_same = true;
                }
            }
        }

        if two_same && increase {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_diff_passwords() {
        assert_eq!(diff_passwords(111111, 111112), 1);
        assert_eq!(diff_passwords(223450, 223452), 0);
        assert_eq!(diff_passwords(171309, 643603), 1625);
    }
}