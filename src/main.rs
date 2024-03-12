
mod media;

use crate::media::Book;
use crate::media::Movie;
//use crate::media::Book::shout;


// enum Media_type {
//     BOOK,
//     CD,
//     GAME,
// }

fn main() {

    let mut book_vec:Vec<Book> = Vec::new();
    
    //media::Book::shout();

    let mybook1 = Book {
        author_first_name: String::from("Leo"),
        author_last_name: String::from("Tolstoy"),
        title: String::from("War and Peace"),
        year_published: 1867
    };

    let mybook2 = Book {
        author_first_name: String::from("Alexandre"),
        author_last_name: String::from("Dumas"),
        title: String::from("The Count of Monte Cristo"),
        year_published: 1846
    };

    let mybook3 = Book {
        author_first_name: String::from("Charles"),
        author_last_name: String::from("Dickens"),
        title: String::from("A Tale of Two Cities"),
        year_published: 1859
    };

    book_vec.push(mybook1);
    book_vec.push(mybook2);
    book_vec.push(mybook3);

    for v in &book_vec {
        println!("{}",v.get_title());
    }

    // let x = book_vec[0].get_title();
    // println!("{}",x);

    // let x = mybook.title;
    // println!("{}",x);
    //mybook.shout();
    // let y = mybook.get_title();
    // println!("{}",y);
    // mybook.shout();

    let mymovie = Movie {
        title: String::from("Stand By Me"),
        year_released: 1986
    };

    // let aa = mymovie.title;
    // println!("{}",aa);

    let ab = mymovie.get_title();
    println!("{}",ab);

    book_vec[0].shout();
  

}
