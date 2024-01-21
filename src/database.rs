use sqlx::sqlite::SqlitePool;

pub fn connect() {
    println!("Connected to database");
}

pub fn insert() {
    println!("Inserted data into database");
}

pub fn update() {
    println!("Updated data in database");
}

pub fn delete() {
    println!("Deleted data from database");
}