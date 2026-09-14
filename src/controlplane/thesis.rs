#[derive(Default)]
pub struct Thesis {
    pub content_addressed: bool,
    pub human_authorized: bool,
    pub reversible: bool,
    pub consequence_classed: bool,
    pub audited: bool,
}

impl Thesis {
    pub fn default_for(_doctrine: &str) -> Self {
        Default::default()
    }

    pub fn from_doctrine(doctrine: &str) -> Self {
        Self::default_for(doctrine)
    }
}
