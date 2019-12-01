mod data_1_12_2019;

fn cal_fuel_req_for_all(modules: Vec<usize>) -> usize {
    modules.iter()
        .map(|m| calc_fuel_req_for_module(m.to_owned()))
        .sum()
}


fn calc_fuel_req_for_module(module_mass: usize) -> usize {
    if module_mass == 0 || module_mass < 6 {
        return 0;
    }
    let req = (module_mass / 3) - 2;

    req + calc_fuel_req_for_module(req)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calc_fuel_req_for_each_module() {
        assert_eq!(2, calc_fuel_req_for_module(14));
        assert_eq!(966, calc_fuel_req_for_module(1969));
        assert_eq!(50346, calc_fuel_req_for_module(100756));
    }

    #[test]
    fn should_calc_fuel_req_for_all_modules() {
        assert!(cal_fuel_req_for_all(data_1_12_2019::parse_input()) > 0)
    }
}

fn main() {
    println!("Answer to part two is = {}", cal_fuel_req_for_all(data_1_12_2019::parse_input()));
}