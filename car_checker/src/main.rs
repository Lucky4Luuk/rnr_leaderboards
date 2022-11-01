// Testing file

use car_checker::regulations::Regulations;

fn main() {
    let car_data = car_checker::from_utf16_file("bepis Dysoon Group C Mid V12.csv").expect("Failed to read!");
    // println!("car data: {:#?}", car_data);
    let result = car_checker::regulations::mcs_s1_group_c::MCS_S1_Group_C::default().check(car_data);
    println!("result: {:#?}", result);
}
