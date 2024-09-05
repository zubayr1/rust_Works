mod content;

use content::{catalog::Catalog, media::Media};

enum MightHaveAValue<'a> {
    Thereis(&'a Media),
    NoValue,
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

    let podcase = Media::Podcast { episode: 420 };

    audioBook.print_media();
    goodMovie.print_media();
    badBook.print_media();
    podcase.print_media();

    let adesc = audioBook.description();
    let mdesc = goodMovie.description();
    let bdesc = badBook.description();
    println!();
    println!("{:?} \n{:?} \n{:?}", adesc, mdesc, bdesc);

    println!();
    let mut catalog = Catalog::new();
    catalog.add(audioBook);
    catalog.add(goodMovie);
    catalog.add(badBook);

    println!("{:?}", catalog);
    println!("{:?}", catalog.items.get(0).unwrap());

    match catalog.items.get(10) {
        Some(value) => {
            println!("ITEM: {:?}", value);
        }
        None => {
            println!("ITEM: NONE");
        }
    }

    match catalog.get_by_id(1) {
        Some(value) => {
            println!("ITEM: {:?}", value);
        }
        None => {
            println!("ITEM: NONE");
        }
    }
}
