#[derive(Default)]
pub struct MCS_S1_GT1;

impl super::Regulations for MCS_S1_GT1 {
    fn check(&self, car_data: crate::CarData) -> Result<(), String> {
        Ok(())
    }
}
