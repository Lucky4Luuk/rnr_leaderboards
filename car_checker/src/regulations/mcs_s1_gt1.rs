#[derive(Default)]
pub struct MCS_S1_GT1;

impl super::Regulations for MCS_S1_GT1 {
    fn check(&self, car_data: crate::CarData) -> Result<(), String> {
        self.check_tech_pool(&car_data)?;
        self.check_other(&car_data)?;
        self.check_stats(&car_data)?;
        self.check_banned_parts(&car_data)?;
        self.check_engine(&car_data)?;
        Ok(())
    }
}

impl MCS_S1_GT1 {
    fn check_tech_pool(&self, car_data: &crate::CarData) -> Result<(), String> {
        for (key, value) in car_data.iter() {
            if key.contains("Techpool") {
                if value.parse::<f32>().map_err(|e| e.to_string())? as isize != 5 {
                    return Err(String::from("Techpool must be 5!"));
                }
            }
        }
        Ok(())
    }

    fn check_other(&self, car_data: &crate::CarData) -> Result<(), String> {
        if car_data.get("Body Name")?.starts_with("LMP") {
            return Err(String::from("Incorrect body!"));
        }
        if car_data.get_usize("Variant Year")? != 1995 {
            return Err(String::from("Engine year must be 1995!"));
        }
        if car_data.get_usize("Engine Family Year")? != 1995 {
            return Err(String::from("Engine year must be 1995!"));
        }
        if car_data.get_usize("Trim Year")? != 1995 {
            return Err(String::from("Body year must be 1995!"));
        }
        if car_data.get_usize("Model Year")? != 1995 {
            return Err(String::from("Body year must be 1995!"));
        }

        // Check seats
        if car_data.get_usize("Small 1st Row Seats")? > 0 || car_data.get_usize("Small 2nd Row Seats")? > 0 || car_data.get_usize("Small 3rd Row Seats")? > 0 {
            return Err(String::from("Must not have small seats!"));
        }

        // Check quality sliders
        let mut min_quality = 0;
        let mut max_quality = 0;
        for (key, value) in car_data.iter() {
            if key.contains("Quality") {
                let qual = value.parse::<f32>().map_err(|e| e.to_string())? as isize;
                min_quality = min_quality.min(qual);
                max_quality = max_quality.max(qual);
            }
        }

        if min_quality < -10 || max_quality > 10 {
            return Err(String::from("Quality sliders may not be more than +/- 10!"));
        }

        let cmat = car_data.get("Chassis Material")?;
        if cmat.contains("Glued") || cmat.contains("Carbon") {
            return Err(String::from(format!("Chassis material cannot be {}", cmat)));
        }

        let pmat = car_data.get("Panel Material")?;
        if pmat.contains("Carbon") {
            return Err(String::from(format!("Panel material cannot be {}", pmat)));
        }

        if car_data.contains_part("Pushrod") {
            return Err(String::from(format!("Pushrod suspension not allowed!")));
        }

        Ok(())
    }

    fn check_stats(&self, car_data: &crate::CarData) -> Result<(), String> {
        if car_data.get_f32("Safety Rating")?.round() < 55.0 {
            return Err(String::from("Safety rating must be at least 50.0!"));
        }
        if car_data.get_f32("Trim Economy")?.round() > 15.6 {
            return Err(String::from("Fuel economy must be 15.6 liters / 100km or better!"));
        }

        if car_data.get_f32("Rear Downforce")?.floor() > 250.0 {
            return Err(String::from("Too much rear downforce!"));
        }
        if car_data.get_f32("Front Downforce")?.floor() > 250.0 {
            return Err(String::from("Too much front downforce!"));
        }

        Ok(())
    }

    fn check_banned_parts(&self, car_data: &crate::CarData) -> Result<(), String> {
        if car_data.get("Active Aero")? != "None" {
            return Err(String::from("Active aero is not allowed!"));
        }
        if car_data.get("Gearbox Type")?.contains("Adv") {
            return Err(String::from("Only manual and automatic transmissions allowed!"));
        }
        if car_data.get("Differential Type")? != "Geared LSD" {
            return Err(String::from("Geared LSD is required! If you want to tune it, use a racing differential in BeamNG."));
        }
        Ok(())
    }

    fn check_engine(&self, car_data: &crate::CarData) -> Result<(), String> {
        if car_data.get("Leaded Fuel")? == "TRUE" {
            return Err(String::from("Running leaded fuel!"));
        }
        if car_data.get_usize("Fuel Octane")? != 98 {
            return Err(String::from("Running the wrong fuel octane!"));
        }
        if car_data.get("Aspiration")? != "None" {
            // Car has a turbo
            if car_data.get("Aspiration")?.contains("Twin") || car_data.get("Aspiration")?.contains("Quad") {
                return Err(String::from("Twin/Quad turbo setups not allowed!"));
            }
            if car_data.get_usize("Cylinder Count")? > 8 {
                return Err(String::from("Can't have more than 8 cylinders!"));
            }
            if car_data.get("Intake")? == "Race" {
                return Err(String::from("Intake manifold can't be race!"));
            }
            if car_data.get_f32("Variant Aspiration Quality")? as isize != 0 {
                return Err(String::from("Turbo quality must be 0!"));
            }
            if car_data.get_f32("Family Displacement")? > 1.7 || car_data.get_f32("Variant Displacement")? > 1.7 {
                return Err(String::from("Displacement cannot be more than 1.7L!"));
            }
            if car_data.get_f32("Peak Boost")? > 1.86 {
                return Err(String::from("Peak boost cannot be more than 2.06 bar!"));
            }
            if car_data.get_f32("Compressor Size 1")? > 90.0 {
                return Err(String::from("Turbo compressor size too big!"));
            }
            if car_data.get_f32("Turbine Size 1")? > 75.0 {
                return Err(String::from("Turbine size too big!"));
            }
            if ((car_data.get_f32("Engine Reliability")? * 10.0).round() / 10.0) < 55.0 {
                return Err(String::from("Engine reliability is too low!"));
            }

            // Check quality sliders
            let mut min_quality = 0;
            let mut max_quality = 0;
            for (key, value) in car_data.iter() {
                if key.contains("Variant") && key.contains("Quality") {
                    let qual = value.parse::<f32>().map_err(|e| e.to_string())? as isize;
                    min_quality = min_quality.min(qual);
                    max_quality = max_quality.max(qual);
                }
            }
            if min_quality < -5 || max_quality > 5 {
                return Err(String::from("Engine quality sliders may not be more than +/- 5!"));
            }
        } else {
            // Car does not have a turbo
            if car_data.get_f32("Family Displacement")? > 4.5 || car_data.get_f32("Variant Displacement")? > 4.5 {
                return Err(String::from("Displacement cannot be more than 3.5L!"));
            } else if car_data.get_f32("Family Displacement")? > 3.3 || car_data.get_f32("Variant Displacement")? > 3.3 {
                if car_data.contains_part("Tubular Race") {
                    return Err(String::from("Not allowed race headers when your engine is bigger than 3.3L!"));
                }
            }
            if ((car_data.get_f32("Engine Reliability")? * 10.0).round() / 10.0) < 45.0 {
                return Err(String::from("Engine reliability is too low!"));
            }
        }

        Ok(())
    }
}
