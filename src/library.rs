pub struct Book {
    title: String,
    author: String,
    year: u32,
    is_borrowed: bool,
}

pub struct Library {
    books: Vec<Book>,
}

impl Book {
    pub fn new_book(title: String, author: String, year: u32, is_borrowed: bool) -> Book {
        Book {
            title,
            author,
            year,
            is_borrowed,
        }
    }

    fn display_info(&self) -> String {
        let info = format!(
            "Titre : {}, Auteur : {}, Année : {}",
            self.title, self.author, self.year
        );
        println!("{}", info);
        info
    }
}

impl Library {
    pub fn new() -> Library {
        Library { books: Vec::new() }
    }
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    pub fn borrow_book(&mut self, title: &str) -> bool {
        if let Some(book) = self
            .books
            .iter_mut()
            .find(|b| b.title == title && !b.is_borrowed)
        {
            book.is_borrowed = true;
            true
        } else {
            false
        }
    }
    pub fn return_book(&mut self, title: &str) -> bool {
        if let Some(book) = self
            .books
            .iter_mut()
            .find(|b| b.title == title && b.is_borrowed)
        {
            book.is_borrowed = false;
            true
        } else {
            false
        }
    }

    pub fn display_available_books(&self) {
        println!("Livres disponibles :");
        for book in &self.books {
            if !book.is_borrowed {
                book.display_info();
            }
        }
    }

    pub fn search_book(&self, title: &str) -> Option<String> {
        self.books
            .iter()
            .find(|b| b.title.to_lowercase() == title.to_lowercase())
            .map(|book| book.display_info())
    }
}
