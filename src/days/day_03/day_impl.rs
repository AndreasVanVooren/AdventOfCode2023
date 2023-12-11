use crate::days::day_base::DayCommon;
use crate::days::day_base::DayTrait;
use crate::utils::grid;
use std::cmp::{min, max};

#[derive(Debug)]
enum GridElemType {
    None,           // Dot
    Digit(u32),     // 0-9
    Symbol(char)    // Anything else. We'll store it since it might be relevant later on.
}

impl GridElemType {
    pub fn is_none(&self) -> bool {
        return matches!(*self, GridElemType::None);
    }
    pub fn is_digit(&self) -> bool {
        return matches!(*self, GridElemType::Digit(_));
    }
    pub fn is_symbol(&self) -> bool {
        return matches!(*self, GridElemType::Symbol(_));
    }
    pub fn as_digit(&self) -> Option<u32> {
        match self {
            GridElemType::None => None,
            GridElemType::Digit(x) => Some(*x),
            GridElemType::Symbol(_) => None,
        }
    }
    pub fn as_symbol(&self) -> Option<char> {
        match self {
            GridElemType::None => None,
            GridElemType::Digit(_) => None,
            GridElemType::Symbol(x) => Some(*x),
        }
    }
}

#[derive(Debug)]
struct GridElem {
    elem_type: GridElemType,
    adjacent_to_symbol: bool,
    handled: bool,
}

impl GridElem {
    pub fn is_none(&self) -> bool {
        return self.elem_type.is_none();
    }
    pub fn is_digit(&self) -> bool {
        return self.elem_type.is_digit();
    }
    pub fn is_symbol(&self) -> bool {
        return self.elem_type.is_symbol();
    }
    pub fn as_digit(&self) -> Option<u32> {
        return self.elem_type.as_digit();
    }
    pub fn as_symbol(&self) -> Option<char> {
        return self.elem_type.as_symbol();
    }
}

pub struct Day {}

impl DayTrait for Day {
    fn get_id_str() -> String {
        "day_03".to_string()
    }
    fn get_result_str() -> String {
        let mut grid = DayCommon::input_file_to_grid::<Day, GridElem, _>(Day::char_to_grid_elem_conv);
        let part_num_sum: u32 = Day::process_part_1(&mut grid).unwrap();
        let gear_num_sum: u32 = Day::process_part_2(&mut grid).unwrap();
        return format!("{}\n{}", part_num_sum, gear_num_sum);
    }
}

impl Day {
    const PKG_ASSOCIATIONS: [(&'static str, i32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

    fn char_to_grid_elem_conv(_:grid::Coord, c: char) -> GridElem {
            GridElem {
                elem_type: match c {
                    c if c == '.'       => GridElemType::None,
                    c if c.is_digit(10) => GridElemType::Digit(c.to_digit(10).unwrap()),
                    c                   => GridElemType::Symbol(c),
                },
                adjacent_to_symbol: false,
                handled: false
            }
    }

    fn check_and_fill_adjacent_symbols(da_griddy: &mut grid::Grid<GridElem>)
    {
        let num_rows = da_griddy.len();
        // First, make sure to fill in all bits that are adjacent.
        // We just do this in a simple row iteration.
        for main_row in 0 .. num_rows {
            let num_cols = da_griddy[main_row].len();
            for main_col in 0 .. num_cols {
                if !da_griddy[main_row][main_col].is_digit() {
                    continue;
                }
                for adj_row in max(main_row as i32 - 1, 0) as usize ..= min(main_row + 1, num_rows - 1) as usize {
                    for adj_col in max(main_col as i32 - 1, 0) as usize ..= min(main_col + 1, num_cols - 1) as usize {
                        if adj_row == main_row && adj_col == main_col {
                            // This is literally the same index.
                            continue;
                        }
                        if !da_griddy[adj_row][adj_col].is_symbol() {
                            // Currently we only care about symbols being adjacent
                            continue;
                        }
                        
                        da_griddy[main_row][main_col].adjacent_to_symbol = true;
                    }
                }
            }
        }
    }

    fn process_part_1(da_griddy: &mut grid::Grid<GridElem>) -> Option::<u32> {
        if da_griddy.is_empty() {
            return None;
        }
        
        Self::check_and_fill_adjacent_symbols(da_griddy);
        
        // Then, iterate every element, until we encounter a digit adjacent to a symbol.
        let num_rows = da_griddy.len();
        
        let mut part_num_sum = 0;
        for main_row in 0 .. num_rows {
            let num_cols = da_griddy[main_row].len();
            for main_col in 0 .. num_cols {
                if !(da_griddy[main_row][main_col].is_digit() && da_griddy[main_row][main_col].adjacent_to_symbol && !da_griddy[main_row][main_col].handled) {
                    continue;
                }
                let mut starting_index = main_col;
                for col_to_left in (0 .. main_col).rev() {
                    if !da_griddy[main_row][col_to_left].is_digit() {
                        break;
                    }
                    starting_index = col_to_left;
                }
                let mut part_num = 0;
                for digit_col in starting_index .. num_rows {
                    let digit = da_griddy[main_row][digit_col].as_digit();
                    if digit.is_none() {
                        break;
                    }
                    da_griddy[main_row][digit_col].handled = true;
                    part_num *= 10;
                    part_num += digit.unwrap();
                }
                part_num_sum += part_num;
            }
        }
        
        
        for main_row in 0 .. num_rows {
            let num_cols = da_griddy[main_row].len();
            for main_col in 0 .. num_cols {
                // Handled was temp, so reset them
                da_griddy[main_row][main_col].handled = false;
            }
        }
        
        return Some(part_num_sum);
    }

    fn process_part_2(da_griddy: &mut grid::Grid<GridElem>) -> Option::<u32> {
        if da_griddy.is_empty() {
            return None;
        }
        
        // Iterate every element and find gears
        let num_rows = da_griddy.len();
        
        let mut gear_num_sum = 0;
        for main_row in 0 .. num_rows {
            let num_cols = da_griddy[main_row].len();
            for main_col in 0 .. num_cols {
                let cur_symbol = da_griddy[main_row][main_col].as_symbol();
                if cur_symbol.is_none() {continue;}
                if cur_symbol? != '*' {continue;}
                let mut gear_ratio = 1;
                let mut gears_found = 0;
                for adj_row in max(main_row as i32 - 1, 0) as usize ..= min(main_row + 1, num_rows - 1) as usize {
                    for adj_col in max(main_col as i32 - 1, 0) as usize ..= min(main_col + 1, num_cols - 1) as usize {
                        if adj_col == main_col && adj_row == main_row {continue;}
                        if da_griddy[adj_row][adj_col].handled {continue;}
                        if !da_griddy[adj_row][adj_col].is_digit() {continue;}
                        
                        let mut starting_index = adj_col;
                        for col_to_left in (0 .. adj_col).rev() {
                            if !da_griddy[adj_row][col_to_left].is_digit() {
                                break;
                            }
                            starting_index = col_to_left;
                        }
                        let mut gear_part_num = 0;
                        for digit_col in starting_index .. num_rows {
                            let digit = da_griddy[adj_row][digit_col].as_digit();
                            if digit.is_none() {
                                break;
                            }
                            da_griddy[adj_row][digit_col].handled = true;
                            gear_part_num *= 10;
                            gear_part_num += digit.unwrap();
                        }
                        gears_found += 1;
                        gear_ratio *= gear_part_num
                    }
                }
                if gears_found == 2 { gear_num_sum += gear_ratio; }
            }
        }
        
        for main_row in 0 .. num_rows {
            let num_cols = da_griddy[main_row].len();
            for main_col in 0 .. num_cols {
                // Handled was temp, so reset them
                da_griddy[main_row][main_col].handled = false;
            }
        }
        
        return Some(gear_num_sum);
    }
}

#[cfg(test)]
mod day_03_test {
    // Needed because otherwise the module doesn't know about
    // the day struct even though it's in the same f***ing file.
    use crate::days::day_03::day_impl::Day;
    use crate::days::day_03::day_impl::GridElem;
    use crate::days::day_base::DayCommon;
    use crate::utils::grid;
    #[test]
    fn test_with_base_str() {
        let test_str = concat!(
            "467..114..\n",
            "...*......\n",
            "..35..633.\n",
            "......#...\n",
            "617*......\n",
            ".....+.58.\n",
            "..592.....\n",
            "......755.\n",
            "...$.*....\n",
            ".664.598..\n",
        );
        let mut grid = DayCommon::input_str_to_grid::<GridElem, _>(&test_str, Day::char_to_grid_elem_conv);
        let part_num_sum: u32 = Day::process_part_1(&mut grid).expect("Couldn't get part num sum, grid was probably empty");
        assert_eq!(part_num_sum, 4361);
    }
    #[test]
    fn test_with_base_str_str_interp() {
        let test_str = concat!(
            "467..114..\n",
            "...*......\n",
            "..35..633.\n",
            "......#...\n",
            "617*......\n",
            ".....+.58.\n",
            "..592.....\n",
            "......755.\n",
            "...$.*....\n",
            ".664.598..\n",
        );
        let mut grid = DayCommon::input_str_to_grid::<GridElem, _>(&test_str, Day::char_to_grid_elem_conv);
        let gear_num_sum: u32 = Day::process_part_2(&mut grid).expect("Couldn't get gear num sum, grid was probably empty");
        assert_eq!(gear_num_sum, 467835);
    }
}
