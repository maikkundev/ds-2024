use rand::{
    distributions::{Alphanumeric, DistString},
    prelude::*,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    id: usize,
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
    pub fn new(&self, id: usize, gender: String, first_name: String, last_name: String) -> Self {
        Self {
            id,
            gender,
            first_name,
            last_name,
        }
    }

    pub fn create_rand() -> Record {
        let mut rng = rand::thread_rng();
        Record {
            id: rng.gen_range(0..usize::MAX),
            gender: Alphanumeric.sample_string(&mut rng, 32),
            first_name: Alphanumeric.sample_string(&mut rng, 32),
            last_name: Alphanumeric.sample_string(&mut rng, 32),
        }
    }

    pub fn create_rand_vec(size: usize) -> Vec<Record> {
        (0..size).map(|_| Record::create_rand()).collect()
    }
}
