#[derive(Clone, PartialEq, Debug)]
pub struct Song {
    title: String,
    id: String,
}

impl Song {
    pub fn new(title: &str, id: &str) -> Self {
        Self {
            title: title.to_owned(),
            id: id.to_owned(),
        }
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}
