pub struct Record {
    pub name: String,
    num1: i32,
    num2: i32,
    num3: i32,
}

impl Record {
    fn new(&self, name: String, num1: i32, num2: i32, num3: i32) -> Self {
        Self {
            name,
            num1,
            num2,
            num3,
        }
    }
}
