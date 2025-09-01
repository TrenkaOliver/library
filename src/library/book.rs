pub enum Category {
    Science,
    Fiction,
    Fantasy,
    Adventure,
    Drama
}

impl Category {
    pub fn from_string(category: &str) -> Result<Self, &str> {
        match category.to_lowercase().as_str() {
            "adventure" => Ok(Self::Adventure),
            "fantasy" => Ok(Self::Fantasy),
            "fiction" => Ok(Self::Fiction),
            "science" => Ok(Self::Science),
            "drama" => Ok(Self::Drama),
            _ => Err(category)
        }
    }

    fn as_string(&self) -> &'static str {
        match self {
            Self::Adventure => "Adventure",
            Self::Fantasy => "Fantasy",
            Self::Fiction => "Fiction",
            Self::Science => "Science",
            Self::Drama => "Drama"
        }
    }
}

pub struct Book {
    title: String,
    author: String,
    category: Category,
    pub owner: Option<String>,
}

impl Book {
    pub fn new(title: String, author: String, category: Category) -> Self {
        Book {
            title,
            author,
            category,
            owner: None,
        }
    }

    pub fn find_book_i_by_title(books: &[Book], title: &str) -> Option<usize> {
        let title = title.to_lowercase();
        
        for (index, book) in books.iter().enumerate() {
            if book.title.to_lowercase() == title {
                return Some(index);
            }
        }

        None
    }

    pub fn display_book(&self, longest_title: usize, longest_author: usize, longest_category: usize, longest_owner:usize) {
        print!("{:width$} | ", self.title, width = longest_title);
        print!("{:width$} | ", self.author, width = longest_author);
        print!("{:width$} | ", self.category.as_string(), width = longest_category);
        println!("{:width$}", self.owner.as_ref().unwrap_or(&"None".to_string()), width = longest_owner);
    }
}