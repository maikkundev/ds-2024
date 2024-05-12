pub struct Record {
    id: i32,
    pub string1: String,
    pub string2: String,
    pub string3: String,
}

impl Record {
    fn new(&self, id: i32, string1: String, string2: String, string3: String) -> Self {
        Self {
            id,
            string1,
            string2,
            string3,
        }
    }
}
