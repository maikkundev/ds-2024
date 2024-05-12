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
}
