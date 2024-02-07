struct Book {
    title: String,
}

struct Author {
    first_name: String,
    last_name: String,
    books: Vec<Book>
}

pub fn example() {
    let author = Author {
        first_name: "Gabriel".to_string(),
        last_name: "Garcia Marquez".to_string(),
        books: vec![
            Book {
                title: "In Evil Hour".to_string(),
            },
            Book {
                title: "One Hundred Years of Solitude".to_string(),
            }
        ],
    };

    println!("List of {} {} books:", author.first_name, author.last_name);

    for book in author.books {
        println!("{}", book.title);
    }
}
