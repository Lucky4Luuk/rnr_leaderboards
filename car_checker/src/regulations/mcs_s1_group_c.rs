#[derive(Default)]
pub struct MCS_S1_Group_C;

impl super::Regulations for MCS_S1_Group_C {
    fn check(&self, car_data: crate::CarData) -> Result<(), String> {
        self.check_tech_pool(&car_data)?;
        self.check_other(&car_data)?;
        self.check_stats(&car_data)?;
        self.check_banned_parts(&car_data)?;
        self.check_engine(&car_data)?;
        Ok(())
    }
}

impl MCS_S1_Group_C {
    fn check_tech_pool(&self, car_data: &crate::CarData) -> Result<(), String> {
        if !car_data.get_usize("Trim Interior Techpool")? == 5 {
            return Err(String::from("Trim interior techpool must be 5!"));
        }
        if !car_data.get_usize("Variant Top End Techpool")? == 5 {
            return Err(String::from("Variant top end techpool must be 5!"));
        }
        if !car_data.get_usize("Variant Bottom End Techpool")? == 5 {
            return Err(String::from("Variant bottom end techpool must be 5!"));
        }
        if !car_data.get_usize("Family Top End Techpool")? == 5 {
            return Err(String::from("Family top end techpool must be 5!"));
        }
        if !car_data.get_usize("Variant Exhaust Techpool")? == 5 {
            return Err(String::from("Variant exhaust techpool must be 5!"));
        }
        if !car_data.get_usize("Family Techpool")? == 5 {
            return Err(String::from("Family techpool must be 5!"));
        }
        if !car_data.get_usize("Model Chassis Techpool")? == 7 {
            return Err(String::from("Model chassis techpool must be 7!"));
        }
        if !car_data.get_usize("Trim Assist Techpool")? == 5 {
            return Err(String::from("Trim assist techpool must be 5!"));
        }
        if !car_data.get_usize("Trim Body Techpool")? == 6 {
            return Err(String::from("Trim body techpool must be 5!"));
        }
        if !car_data.get_usize("Trim Chassis Techpool")? == 7 {
            return Err(String::from("Trim chassis techpool must be 7!"));
        }
        if !car_data.get_usize("Trim Brake Techpool")? == 5 {
            return Err(String::from("Trim brake techpool must be 5!"));
        }
        if !car_data.get_usize("Variant Fuel System Techpool")? == 5 {
            return Err(String::from("Variant fuel system techpool must be 5!"));
        }
        if !car_data.get_usize("Model Body Techpool")? == 6 {
            return Err(String::from("Model body techpool must be 6!"));
        }
        if !car_data.get_usize("Trim Tyre Techpool")? == 7 {
            return Err(String::from("Trim Tyre techpool must be 7!"));
        }
        if !car_data.get_usize("Variant Aspiration Techpool")? == 5 {
            return Err(String::from("Variant aspiration techpool must be 5!"));
        }
        if !car_data.get_usize("Trim Drivetrain Techpool")? == 6 {
            return Err(String::from("Trim drivetrain techpool must be 6!"));
        }
        if !car_data.get_usize("Trim Fixture Techpool")? == 5 {
            return Err(String::from("Trim fixture techpool must be 5! How the fuck did you manage this?"));
        }
        if !car_data.get_usize("Trim Suspension Techpool")? == 5 {
            return Err(String::from("Trim suspension techpool must be 5!"));
        }
        if !car_data.get_usize("Variant Family Techpool")? == 5 {
            return Err(String::from("Variant family techpool must be 5!"));
        }
        if !car_data.get_usize("Trim Aerodynamics Techpool")? == 8 {
            return Err(String::from("Trim aerodynamics techpool must be 8!"));
        }
        if !car_data.get_usize("Trim Safety Techpool")? == 5 {
            return Err(String::from("Trim safety techpool must be 5!"));
        }
        Ok(())
    }

    fn check_other(&self, car_data: &crate::CarData) -> Result<(), String> {
        if !car_data.get("Body Name")?.starts_with("LMP") {
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

        Ok(())
    }

    fn check_stats(&self, car_data: &crate::CarData) -> Result<(), String> {
        if car_data.get_f32("Safety Rating")?.round() < 50.0 {
            return Err(String::from("Safety rating must be at least 50.0!"));
        }
        if car_data.get_f32("Trim Economy")?.round() > 23.5 {
            return Err(String::from("Fuel economy must be 23.5 liters / 100km or better!"));
        }
        // if car_data.get_cost()? > 100_000 {
        //     return Err(String::from("Total cost must be under 100.000$!"));
        // }

        if car_data.get_f32("Rear Downforce")?.floor() > 850.0 {
            return Err(String::from("Too much rear downforce!"));
        }
        if car_data.get_f32("Front Downforce")?.floor() > 850.0 {
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
        if car_data.get("Leaded Fuel")? != "TRUE" {
            return Err(String::from("Not running leaded fuel!"));
        }
        if car_data.get_usize("Fuel Octane")? != 110 {
            return Err(String::from("Running the wrong fuel octane!"));
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
        if car_data.get("Muffler 1")? == "None" || car_data.get("Muffler 2")? == "None" {
            return Err(String::from("Mufflers cannot be None!"));
        }
        if car_data.get("Aspiration")? != "None" {
            // Car has a turbo
            if car_data.get_f32("Variant Aspiration Quality")? as isize != 0 {
                return Err(String::from("Turbo quality must be 0!"));
            }
            if car_data.get_f32("Family Displacement")? > 1.6 || car_data.get_f32("Variant Displacement")? > 1.6 {
                return Err(String::from("Displacement cannot be more than 1.6L!"));
            }
            if car_data.get_f32("Peak Boost")? > 2.06 {
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
        } else {
            // Car does not have a turbo
            if car_data.get_f32("Family Displacement")? > 3.8 || car_data.get_f32("Variant Displacement")? > 3.8 {
                return Err(String::from("Displacement cannot be more than 3.8L!"));
            }
            if ((car_data.get_f32("Engine Reliability")? * 10.0).round() / 10.0) < 40.0 {
                return Err(String::from("Engine reliability is too low!"));
            }
        }

        Ok(())
    }
}
