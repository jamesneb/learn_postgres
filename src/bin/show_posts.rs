extern crate diesel;
extern crate learn_postgres;

use self::diesel::prelude::*;
use self::learn_postgres::*;
use self::models::*;

fn main() {
    use learn_postgres::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
