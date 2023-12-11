use std::path::Path;
use std::path::PathBuf;

use crate::utils::grid;
use std::io;

pub trait DayTrait {
    fn get_result_str() -> String;
    fn get_id_str() -> String;
}

pub struct DayCommon {}
impl DayCommon {
    pub fn get_day_dir_str<T: DayTrait>() -> PathBuf {
        PathBuf::from(format!("./src/days/{}", T::get_id_str()))
    }
    pub fn get_input_path_str<T: DayTrait>() -> PathBuf {
        let mut temp: PathBuf = DayCommon::get_day_dir_str::<T>();
        temp.push("input");
        return temp;
    }
    pub fn get_input_path_fs<T: DayTrait>() -> io::Result<PathBuf> {
        return DayCommon::get_input_path_str::<T>().canonicalize();
    }

    pub fn for_each_line_in_path<P, F>(path: P, func: F)
    where
        P: AsRef<Path>,
        F: FnMut(&str),
    {
        let str_value = std::fs::read_to_string(path);
        DayCommon::for_each_line_in_input_str(&str_value.expect("Expected file to be read"), func);
    }
    pub fn for_each_line_in_input_file<T: DayTrait, F>(func: F)
    where
        F: FnMut(&str),
    {
        let path = DayCommon::get_input_path_fs::<T>();
        DayCommon::for_each_line_in_path(path.expect("Expected path to be valid."), func);
    }
    pub fn for_each_line_in_input_str<F>(str_value: &str, func: F)
    where
        F: FnMut(&str),
    {
        str_value.lines().for_each(func);
    }

    pub fn input_str_to_grid<T, F>(str_value: &str, func: F) -> grid::Grid<T>
    where
        F: Fn(grid::Coord, char) -> T,
    {
        let mut grid = grid::Grid::<T>::new();
        let mut row_num: i64 = 0;
        DayCommon::for_each_line_in_input_str(str_value, |s| {
            if s.is_empty() {
                return;
            }
            let mut col_num: i64 = 0;
            let mut row = Vec::<T>::new();
            for c in s.chars() {
                row.push(func(
                    grid::Coord {
                        x: col_num,
                        y: row_num,
                    },
                    c,
                ));
                col_num += 1;
            }
            grid.push(row);
            row_num += 1;
        });
        return grid;
    }
    pub fn input_file_to_grid<DT: DayTrait, T, F>(func: F) -> grid::Grid<T>
    where
        F: Fn(grid::Coord, char) -> T,
    {
        let mut grid = grid::Grid::<T>::new();
        let mut row_num: i64 = 0;
        DayCommon::for_each_line_in_input_file::<DT, _>(|s| {
            if s.is_empty() {
                return;
            }
            let mut col_num: i64 = 0;
            let mut row = Vec::<T>::new();
            for c in s.chars() {
                row.push(func(
                    grid::Coord {
                        x: col_num,
                        y: row_num,
                    },
                    c,
                ));
                col_num += 1;
            }
            grid.push(row);
            row_num += 1;
        });
        return grid;
    }
}
