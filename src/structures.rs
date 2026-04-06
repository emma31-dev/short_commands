#[allow(unused)]
pub struct Command {
    short: String,
    long: String,
}

#[allow(unused)]
impl Command {
    pub fn new(short: String, long: String) -> Self {
        Self { short, long }
    }
}
