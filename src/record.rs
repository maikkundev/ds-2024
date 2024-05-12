use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Record {
    id: i32,
    pub gender: String,
    pub first_name: String,
    pub last_name: String,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.gender.eq(&other.gender)
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.gender.partial_cmp(&other.gender)
    }
}

impl Record {
    pub fn new(&self, id: i32, gender: String, first_name: String, last_name: String) -> Self {
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
