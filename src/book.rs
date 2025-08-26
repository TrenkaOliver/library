pub enum Category {
    Science,
    Fiction,
    Fantasy,
    Adventure
}

impl Category {
    fn as_string(&self) -> &'static str {
        match self {
            Self::Adventure => "Adventure",
            Self::Fantasy => "Fantasy",
            Self::Fiction => "Fiction",
            Self::Science => "Science",
        }
    }
}

pub struct Book {
    title: String,
    author: String,
    category: Category,
    pub owner: String,
}

impl Book {
    pub fn new(title: String, author: String, category: Category) -> Self {
        Book {
            title,
            author,
            category,
            owner: String::from("None"),
        }
    }

    pub fn find_book_i_by_title(books: &[Book], title: &str) -> Option<usize> {
        for (index, book) in books.iter().enumerate() {
            if book.title == title {
                return Some(index);
            }
        }

        None
    }

    pub fn display_book(&self, longest_title: usize, longest_author: usize, longest_category: usize, longest_owner:usize) {
        print!("{:width$} | ", self.title, width = longest_title);
        print!("{:width$} | ", self.author, width = longest_author);
        print!("{:width$} | ", self.category.as_string(), width = longest_category);
        println!("{:width$}", self.owner, width = longest_owner);
    }
}