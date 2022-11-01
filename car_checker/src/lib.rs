use std::collections::HashMap;

use anyhow::Result;

pub mod regulations;

pub fn from_utf16_file(path: &str) -> Result<CarData> {
    let bytes = std::fs::read(path)?;
    let (_prefix, utf16, _suffix) = unsafe { bytes.align_to::<u16>() };
    let utf8 = String::from_utf16(&utf16)?;

    from_utf8_string(utf8)
}

pub fn from_utf8_string(utf8: String) -> Result<CarData> {
    let mut rdr = csv::Reader::from_reader(utf8.as_bytes());
    let mut iter = rdr.deserialize();
    let mut data: HashMap<String, String> = HashMap::new();
    if let Some(record) = iter.next() {
        data = record?;
    }
    Ok(CarData(data))
}

pub struct CarData(HashMap<String, String>);

impl CarData {
    pub fn get<S: Into<String>>(&self, key: S) -> Result<String, String> {
        self.0.get(&key.into()).map(|v| v.clone()).ok_or(String::from("Expected value not found!"))
    }

    pub fn get_f32<S: Into<String>>(&self, key: S) -> Result<f32, String> {
        self.get(key).map(|v| v.parse::<f32>())?.map_err(|e| e.to_string())
    }

    pub fn get_usize<S: Into<String>>(&self, key: S) -> Result<usize, String> {
        self.get_f32(key).map(|v| v as usize)
    }

    pub fn get_cost(&self) -> Result<usize, String> {
        // Dollar Value =   (Car + Engine Material Costs) +
        //                  25 * (Car + Engine Production Units) +
        //                  (Car Engineering Time * Car Production Units) +
        //                  (Engine Engineering Time * Engine Production Units)
        // let car = self.get_usize("Trim Material Cost")?;
        // let engine_material_costs = self.get_usize("Engine Material Cost")?;
        // let car_production_units = self.get_usize("Trim Production Units")?;
        // let engine_production_units = self.get_usize("Engine Production Units")?;
        // let car_engineering_time = self.get_usize("Trim Engineering Time")?;
        // let engine_engineering_time = self.get_usize("Engine Engineering Time")?;
        // let total_cost =    (car + engine_material_costs) +
        //                     25 * (car + engine_production_units) +
        //                     (car_engineering_time * car_production_units) +
        //                     (engine_engineering_time * engine_production_units);
        let trim_cost = self.get_usize("Trim Total Costs")?;
        let engine_cost = self.get_usize("Engine Total Cost")?;
        let total_cost = trim_cost + engine_cost;
        println!("Total cost: {}", total_cost);
        Ok(total_cost)
    }
}

impl std::ops::Deref for CarData {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
