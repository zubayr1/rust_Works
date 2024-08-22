#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
}

fn print_media(media: Media) {
    println!("{:?}", media);
}

fn main() {
    let audioBook = Media::AudioBook {
        title: "harry potter".to_string(),
    };

    let goodMovie = Media::Movie {
        title: "LOTR".to_string(),
        director: "abc".to_string(),
    };

    let badBook = Media::Book {
        title: "baddd".to_string(),
        author: "xyz".to_string(),
    };

    print_media(audioBook);
    print_media(goodMovie);
    print_media(badBook);
}
