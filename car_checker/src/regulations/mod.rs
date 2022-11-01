use crate::CarData;

pub mod mcs_s1_group_c;

pub trait Regulations {
    fn check(&self, car_data: CarData) -> core::result::Result<(), String>;
}
