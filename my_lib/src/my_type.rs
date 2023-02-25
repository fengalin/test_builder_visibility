#[derive(Debug, Default)]
pub struct MyType {
    pub field: Option<i32>,
}

impl MyType {
    /// Instantiates a [`MyTypeBuilder`].
    ///
    /// Use this function to define optional fields.
    pub fn builder() -> MyTypeBuilder {
        MyTypeBuilder::default()
    }
}

#[derive(Default)]
pub struct MyTypeBuilder {
    field: Option<i32>,
}

impl MyTypeBuilder {
    pub fn field(mut self, val: i32) -> Self {
        self.field = Some(val);

        self
    }

    pub fn build(self) -> MyType {
        MyType { field: self.field }
    }
}
