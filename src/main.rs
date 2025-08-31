use std::io;

mod library;

use library::Library;



fn main() {

    let mut library = Library::new();

    println!("Welcome to the Library");

    library.start();

    loop {
        println!("1: Borrow book\n2: Return book\n3: Add book\n4: View book infos\n5: Log out\n6: Exit");

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
            },
            "3" => library.add_book(),
            "4" => library.inspect_books(),
            "5" => {
                println!("Succesfully Logged out!");
                library.start();
            },
            "6" => break,
            _ => println!("Select a number between 1 and 6!"),
        }
    }
    
    println!("Thanks for using our library");
}
