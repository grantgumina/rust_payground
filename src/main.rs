mod database;
use dotenv;

fn main() {
    dotenv::dotenv().ok();

    match database::connect() {
        Ok(_) => println!("Connected to database"),
        Err(e) => println!("Failed to connect to database: {}", e),
    }
}
