use crate::days::day_base::DayCommon;
use crate::days::day_base::DayTrait;
use regex::Regex;

pub struct Day {}

impl DayTrait for Day {
    fn get_id_str() -> String {
        "day_02".to_string()
    }
    fn get_result_str() -> String {
        let mut possibility_sum: i32 = 0;
        let mut power_sum: i32 = 0;
        DayCommon::for_each_line_in_input_file::<Day, _>(|s| {
            let (id, is_possible) = Day::process_part_1(s);
            if is_possible {
                possibility_sum += id;
            }
            power_sum += Day::process_part_2(s);
        });

        // Process string
        return format!("- Possibilities {}\n- Power {}", possibility_sum, power_sum);
    }
}

impl Day {
    const PKG_ASSOCIATIONS: [(&'static str, i32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

    fn process_part_1(str_value: &str) -> (i32, bool) {
        if str_value == "" {
            return (0, false);
        }
        let id_regex = Regex::new(r"Game ([0-9]*)").expect("Expect regex to parse.");
        let pkg_regex = Regex::new(r"([0-9]*) ([A-Za-z]*)").expect("Expect regex to parse.");
        let (game_str, setlist_str) = str_value
            .split_once(':')
            .expect("Expect to have at least one dot.");
        let id = id_regex.captures(game_str).expect("No captures?")[1]
            .parse::<i32>()
            .expect("Expected parsing to work");
        let sets = setlist_str.split(';');

        let mut is_game_possible: bool = true;
        for set in sets {
            let pkg_strs = set.split(',');
            for pkg_str in pkg_strs {
                let pkg_caps = pkg_regex
                    .captures(pkg_str.trim())
                    .expect("Need at least some captures");
                let pkg_amount = pkg_caps[1].parse::<i32>().expect("Need parsing to work");
                let pkg_id: String = String::from(&pkg_caps[2]);
                let pkg_index = Day::PKG_ASSOCIATIONS
                    .iter()
                    .position(|&(id, _)| id == &pkg_id)
                    .expect("Should be in there");
                if pkg_amount > Day::PKG_ASSOCIATIONS[pkg_index].1 {
                    is_game_possible = false;
                }
            }
        }
        return (id, is_game_possible);
    }

    fn process_part_2(str_value: &str) -> i32 {
        if str_value == "" {
            return 0;
        }
        let pkg_regex = Regex::new(r"([0-9]*) ([A-Za-z]*)").expect("Expect regex to parse.");
        let (_, setlist_str) = str_value
            .split_once(':')
            .expect("Expect to have at least one dot.");
        let sets = setlist_str.split(';');

        let mut pkg_min_amounts = Vec::<(String, i32)>::new();
        for (pkg_str, _) in Day::PKG_ASSOCIATIONS {
            pkg_min_amounts.push((String::from(pkg_str), 0));
        }

        for set in sets {
            let pkg_strs = set.split(',');
            for pkg_str in pkg_strs {
                let pkg_caps = pkg_regex
                    .captures(pkg_str.trim())
                    .expect("Need at least some captures");
                let pkg_amount = pkg_caps[1].parse::<i32>().expect("Need parsing to work");
                let pkg_id: String = String::from(&pkg_caps[2]);
                let pkg_index = pkg_min_amounts
                    .iter()
                    .position(|(id, _)| id == &pkg_id)
                    .expect("Should be in there");
                if pkg_amount > pkg_min_amounts[pkg_index].1 {
                    pkg_min_amounts[pkg_index].1 = pkg_amount;
                }
            }
        }

        let mut power_total = 1;
        for (_, power) in pkg_min_amounts {
            power_total *= power;
        }

        return power_total;
    }
}

#[cfg(test)]
mod day_02_test {
    // Needed because otherwise the module doesn't know about
    // the day struct even though it's in the same f***ing file.
    use crate::days::day_02::day_impl::Day;
    use crate::days::day_base::DayCommon;
    #[test]
    fn test_with_base_str() {
        let test_str = concat!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n"
        );
        let mut possibility_sum: i32 = 0;
        DayCommon::for_each_line_in_input_str(test_str, |s| {
            let (id, is_possible) = Day::process_part_1(s);
            if is_possible {
                possibility_sum += id;
            }
        });
        assert_eq!(possibility_sum, 8);
    }
    #[test]
    fn test_with_base_str_str_interp() {
        let test_str = concat!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n"
        );
        let mut power_sum: i32 = 0;
        DayCommon::for_each_line_in_input_str(test_str, |s| {
            power_sum += Day::process_part_2(s);
        });
        assert_eq!(power_sum, 2286);
    }
}
