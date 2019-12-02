pub mod day_2_part_1;
pub mod day_2_data;

type IntTuple = (usize, usize);

const MATCH: usize = 19690720;


fn find_noun_verb() -> IntTuple {
    let input = day_2_data::parse_input();

    for noun in 0..99 {
        for verb in 0..99 {
            let mut cloned = input.clone();
            cloned[1] = noun;
            cloned[2] = verb;

            if day_2_part_1::intcode_computer(&mut cloned)[0] == MATCH {
                return (noun, verb);
            }
        }
    }

    panic!("Couldn't find match!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_noun_verb() {
        find_noun_verb(); // just don't panic
    }
}

fn main() {
    println!("Answer to part two is = {:?}", find_noun_verb());
}
