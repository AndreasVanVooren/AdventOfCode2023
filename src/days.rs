use crate::days::day_01::day_impl::Day as Day_01;
use crate::days::day_02::day_impl::Day as Day_02;
use crate::days::day_03::day_impl::Day as Day_03;
use crate::days::day_04::day_impl::Day as Day_04;
use crate::days::day_base::DayTrait;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_base;

pub struct Days {}

impl Days {
    fn create_and_print<T: DayTrait>() {
        println!("Result for {}:", T::get_id_str());
        println!("{}", T::get_result_str());
    }
    pub fn do_it() {
        Self::create_and_print::<Day_01>();
        Self::create_and_print::<Day_02>();
        Self::create_and_print::<Day_03>();
        Self::create_and_print::<Day_04>();
    }
}
