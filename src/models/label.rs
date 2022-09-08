pub struct Label(u8);

impl Label {
    pub fn new(value: &u8) -> Self {
        Self(*value)
    }
}
