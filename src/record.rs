use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Record {
    id: i32,
    pub gender: String,
    pub first_name: String,
    pub last_name: String,
}

impl Record {
    fn new(&self, id: i32, gender: String, first_name: String, last_name: String) -> Self {
        Self {
            id,
            gender,
            first_name,
            last_name,
        }
    }

    pub fn import_records() -> Result<Vec<Record>, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new().from_path("MOCK_DATA.csv")?;
        let mut records = vec![];

        for result in rdr.deserialize() {
            let record: Record = result?;
            records.push(record);
        }

        Ok(records)
    }
}
