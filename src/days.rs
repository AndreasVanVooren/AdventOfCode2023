pub mod day_base;
pub mod day_01;
use crate::days::day_base::day_trait;


pub struct days {
}

impl days {
	fn create_and_print<T: day_trait>() {
		println!("Result for {}:", T::get_id_str());
		println!("{}", T::get_result_str());
	}
	pub fn do_it(){
		Self::create_and_print::<crate::days::day_01::day_impl_01::day>();
	}
}