use crate::days::day_base::DayCommon;
use crate::days::day_base::DayTrait;

pub struct ScratchCard {
    amount: i32,
    accounted_for: bool,
}

pub struct Day {}

impl DayTrait for Day {
    fn get_id_str() -> String {
        "day_04".to_string()
    }
    fn get_result_str() -> String {
        let mut ticket_sum: i32 = 0;
        let mut scratch_card_list = Vec::<ScratchCard>::new();
        DayCommon::for_each_line_in_input_file_indexed::<Day, _>(|(idx, s)| {
            Day::for_scratch_card(&mut scratch_card_list, &mut ticket_sum, idx, s);
        });

        let mut scratch_card_sum = 0;
        for card in scratch_card_list {
            if !card.accounted_for {
                continue;
            }
            scratch_card_sum += card.amount;
        }
        // Process string
        return format!("- Ticket sum {}\n- Power {}", ticket_sum, scratch_card_sum);
    }
}

impl Day {
    fn for_scratch_card(
        scratch_card_list: &mut Vec<ScratchCard>,
        ticket_sum: &mut i32,
        idx: usize,
        s: &str,
    ) {
        if idx >= scratch_card_list.len() {
            scratch_card_list.push(ScratchCard {
                amount: 1,
                accounted_for: true,
            });
        }
        scratch_card_list[idx].accounted_for = true;
        let amount_of_this_ticket = scratch_card_list[idx].amount;

        let amount_of_won_tickets = Day::process_part_1(s);
        println!(
            "Ticket {}: amount {}, number won {}",
            idx, amount_of_this_ticket, amount_of_won_tickets
        );
        if amount_of_won_tickets > 0 {
            *ticket_sum += 2_i32.pow((amount_of_won_tickets - 1) as u32);
        }
        for next in (idx + 1)..(idx + 1 + (amount_of_won_tickets as usize)) {
            if next >= scratch_card_list.len() {
                scratch_card_list.push(ScratchCard {
                    amount: 1,
                    accounted_for: false,
                });
            }
            scratch_card_list[next].amount += amount_of_this_ticket;
        }
    }

    fn process_part_1(str_value: &str) -> i32 {
        if str_value.is_empty() {
            return 0;
        }
        let (_, setlist_str) = str_value
            .split_once(':')
            .expect("Expect to have at least one dot.");
        let (win_num_list_str, num_list_str) = setlist_str
            .split_once('|')
            .expect(&format!("Expect separator in '{}'", str_value));
        let win_num_list: Vec<&str> = win_num_list_str.split_whitespace().collect();
        let mut amount_of_matches = 0;
        for num in num_list_str.split_whitespace() {
            if win_num_list.contains(&num) {
                amount_of_matches += 1;
            }
        }

        return amount_of_matches;
    }
}

#[cfg(test)]
mod day_04_test {
    // Needed because otherwise the module doesn't know about
    // the day struct even though it's in the same f***ing file.
    use crate::days::day_04::day_impl::Day;
    use crate::days::day_04::day_impl::ScratchCard;
    use crate::days::day_base::DayCommon;
    #[test]
    fn test_with_base_str() {
        let test_str = concat!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n"
        );
        let mut ticket_sum: i32 = 0;
        let mut scratch_card_list = Vec::<ScratchCard>::new();
        DayCommon::for_each_line_in_input_str_indexed(test_str, |(idx, s)| {
            Day::for_scratch_card(&mut scratch_card_list, &mut ticket_sum, idx, s);
        });
        assert_eq!(ticket_sum, 13);
    }
    #[test]
    fn test_with_base_str_str_interp() {
        let test_str = concat!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n"
        );
        let mut ticket_sum: i32 = 0;
        let mut scratch_card_list = Vec::<ScratchCard>::new();
        DayCommon::for_each_line_in_input_str_indexed(test_str, |(idx, s)| {
            Day::for_scratch_card(&mut scratch_card_list, &mut ticket_sum, idx, s);
        });
        let mut scratch_card_sum = 0;
        for card in scratch_card_list {
            if !card.accounted_for {
                continue;
            }
            scratch_card_sum += card.amount;
        }
        assert_eq!(scratch_card_sum, 30);
    }
}
