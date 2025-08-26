use std::io;

mod book;
mod user;

use book::{Category, Book};
use user::User;

struct Library {
    books: [Book; 5],
    users: [User; 3],
    current_user: usize,
    longest_title: usize,
    longest_author: usize,
    longest_owner: usize,
    longest_category: usize
}

impl Library {
    fn log_in(&mut self) {
        
        println!("\nPlease log in");

        'main: loop {
            println!("Enter your name!");
            let mut name = String::new();
            match io::stdin().read_line(&mut name) {
                Ok(_) => (),
                Err(_) => {println!("Error reading input!"); continue;},
            }

            let name = name.trim();
            for (index, user) in self.users.iter().enumerate() {
                if user.name == name {
                    self.current_user = index;
                    println!("Succesfully logged in as {name}!\n");
                    break 'main;
                }
            }

            println!("{name} is not an existing name in out database");
        }
    }

    fn borrow_book(&mut self) {
        println!("\nEnter book's title to borrow");
        let mut title = String::new();
        match io::stdin().read_line(&mut title) {
            Ok(_) => (),
            Err(_) => {println!("Error reading input"); return;}
        }

        match Book::find_book_i_by_title(&self.books, title.trim()) {
            Some(index) => {
                if self.books[index].owner == "None" {
                    println!("You borrowed {title} succesfully!\n");
                    self.books[index].owner = self.users[self.current_user].name.clone();
                } else {
                    println!("This book is currenty borrowed by {}\n", self.books[index].owner);
                }
            }

            None => println!("There's no book titled {title}\n"),
        }
    }

    fn return_book(&mut self) {
        println!("\nEnter book's title to return");
        let mut title = String::new();
        match io::stdin().read_line(&mut title) {
            Ok(_) => (),
            Err(_) => {println!("Error reading input"); return;}
        }

        match Book::find_book_i_by_title(&self.books, title.trim()) {
            Some(index) => {
                if self.books[index].owner == self.users[self.current_user].name {
                    println!("You borrowed {title} succesfully!\n");
                    self.books[index].owner = String::from("None");
                } else {
                    println!("You don't posess this book\n");
                }
            }

            None => println!("There's no book titled {title}\n"),
        }
    }

    fn update_longest_owner(&mut self) {
        if self.longest_owner < self.users[self.current_user].name.len() {
            self.longest_owner = self.users[self.current_user].name.len();
        }
    }

    fn inspect_books(&self) {
        print!("\n");

        for book in self.books.iter() {
            book.display_book(self.longest_title, self.longest_author, self.longest_category, self.longest_owner);
        }

        print!("\n");
    }

}

fn main() {

    let mut library = Library {
        books: [
            Book::new(String::from("Whispers of the Quantum Tree"), String::from("Lyra Moonshadow"), Category::Science),
            Book::new(String::from("Neon Circuits: Tales from Tomorrow"), String::from("Orion Flux"), Category::Fiction),
            Book::new(String::from("The Mirage Codex"), String::from("Selene Vey"), Category::Fantasy),
            Book::new(String::from("The Clockwork Alchemist"), String::from("Evander Voss"), Category::Science),
            Book::new(String::from("Embers of the Forgotten Sun"), String::from("Kael Riven"), Category::Adventure),
        ],
        users: [
            User {name: String::from("starlight42")},
            User {name: String::from("ironquill")},
            User {name: String::from("lunarbyte")},
        ],
        current_user: 0,
        longest_title: 34,
        longest_author: 15,
        longest_owner: 4,
        longest_category: 9,
    };

    println!("Welcome to the Library");

    library.log_in();

    loop {
        println!("1: Borrow book\n2: Return book\n3: View book infos\n4: Log out\n5: Exit");

        let mut choice = String::new();
        match io::stdin().read_line(&mut choice) {
            Ok(_) => (),
            Err(_) => {println!("error reading the input"); continue;}
        }; 

        let choice = choice.trim();
        
        match choice {
            "1" => {
                library.borrow_book();
                library.update_longest_owner();
            },
            "2" => {
                library.return_book();
                library.update_longest_owner();
            }
            "3" => library.inspect_books(),
            "4" => {
                println!("Succesfully Logged out!");
                library.log_in();
            },
            "5" => break,
            _ => println!("Select a number between 1 and 4!"),
        }
    }
    
    println!("Thanks for using our library");
}
