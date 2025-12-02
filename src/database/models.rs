/// Book data model for MVP
#[derive(Debug, Clone)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub authors: Vec<String>,
    pub path: String,
    pub has_cover: bool,
    pub timestamp: String,
}

impl Book {
    pub fn author_list(&self) -> String {
        self.authors.join(", ")
    }

    pub fn display_title(&self) -> String {
        if self.title.len() > 50 {
            format!("{}...", &self.title[..47])
        } else {
            self.title.clone()
        }
    }
}