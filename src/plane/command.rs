#[derive(Clone, Debug)]
pub struct Command {
    pub verb: String,
    pub arg: String,
}

impl Command {
    pub fn new(verb: impl Into<String>, arg: impl Into<String>) -> Self {
        Self {
            verb: verb.into(),
            arg: arg.into(),
        }
    }
}
