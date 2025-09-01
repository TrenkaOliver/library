mod book;
mod user;

use std::io;
use book::{Book, Category};
use user::User;

pub struct Library {
    books: Vec<Book>,
    users: Vec<User>,
    current_user: usize,
    longest_title: usize,
    longest_author: usize,
    longest_owner: usize,
    longest_category: usize
}

impl Library {
    pub fn new() -> Self {
        Library { 
            books: Vec::new(), 
            users: Vec::new(), 
            current_user: 0, 
            longest_title: 0, 
            longest_author: 0, 
            longest_owner: 0, 
            longest_category: 9
        }
    }

    fn get_user_input(message: &str) -> String {
        println!("{message}");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("couldn't read input");

        name.trim().to_string()
    }

    fn update_longest(longest: &mut usize, new_value: &str) {
        if *longest < new_value.len() {
            *longest = new_value.len();
        }
    }

    fn log_in(&mut self) -> bool {
        let name = Self::get_user_input("\nEnter your name!");

        for (index, user) in self.users.iter().enumerate() {
            if user.name == name {
                self.current_user = index;
                println!("Succesfully logged in as {name}!\n");
                return true;
            }
        }

        println!("no existing account with the name: {name}");
        false
    }

    fn register(&mut self) -> bool { 
        let name = Self::get_user_input("\nEnter your name!");

        for user in &self.users {
            if user.name == name {
                println!("There's already a user named: {name}");
                return false;
            }
        }

        println!("Succesfully logged in as {name}!\n");   
        self.users.push(User{name});
        self.current_user = self.users.len() - 1;

        true
    }

    pub fn start(&mut self) {
        loop {
            match Self::get_user_input("\n1: Login\n2: Register").as_str() {
                "1" => if self.log_in() {break;},
                "2" => if self.register() {break;},
                _ => println!("Enter a 1 or 2"),
            }
        }
    }

    pub fn borrow_book(&mut self) {
        let title = Self::get_user_input("\nEnter book's title to borrow");

        match Book::find_book_i_by_title(&self.books, &title) {
            Some(index) => {
                if self.books[index].owner.is_none() {
                    println!("You borrowed {title} succesfully!\n");
                    self.books[index].owner = Some(self.users[self.current_user].name.clone());
                    Self::update_longest(&mut self.longest_owner, &self.users[self.current_user].name);
                } else {
                    println!("This book is currenty borrowed by {}\n", self.books[index].owner.as_ref().unwrap());
                }
            }

            None => println!("There's no book titled {title}\n"),
        }
    }

    pub fn return_book(&mut self) {
        let title = Self::get_user_input("\nEnter book's title to return");

        match Book::find_book_i_by_title(&self.books, &title) {
            Some(index) => {
                if self.books[index].owner.as_deref().unwrap_or("None") == self.users[self.current_user].name {
                    println!("You returned {title} succesfully!\n");
                    self.books[index].owner = None;
                } else {
                    println!("You don't posess this book\n");
                }
            }

            None => println!("There's no book titled {title}\n"),
        }
    }

    pub fn add_book(&mut self) {
        let title = Self::get_user_input("Enter the book's title");
        let author = Self::get_user_input("Enter the book's author");
        let category = match Category::from_string(&Self::get_user_input("Enter the book's category")) {
            Ok(res) => res,
            Err(r) => {println!("There's no category named: {r}\n"); return;}
        };

        Self::update_longest(&mut self.longest_title, &title);
        Self::update_longest(&mut self.longest_author, &author);

        self.books.push(Book::new(title, author, category));
    }

    pub fn inspect_books(&self) {
        print!("\n");

        if self.books.len() == 0 {
            println!("No books available");
        }

        for book in self.books.iter() {
            book.display_book(self.longest_title, self.longest_author, self.longest_category, self.longest_owner);
        }

        print!("\n");
    }

}

