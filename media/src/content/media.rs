#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast { episode: u32 },
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::AudioBook { title } => {
                format!("AudioBook: {}", title)
            }
            Media::Movie { title, director } => {
                format!("Book: {} {}", title, director)
            }
            Media::Podcast { episode } => {
                format!("Podcast: {}", episode)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }

    pub fn print_media(&self) {
        println!("{:?}", self);
    }
}
