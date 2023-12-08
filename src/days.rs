pub mod day_base;
pub mod day_01;
use crate::days::day_base::DayTrait;
use crate::days::day_01::day_impl_01::Day as Day_01;


pub struct Days {
}

impl Days {
	fn create_and_print<T: DayTrait>() {
		println!("Result for {}:", T::get_id_str());
		println!("{}", T::get_result_str());
	}
	pub fn do_it(){
		Self::create_and_print::<Day_01>();
	}
}