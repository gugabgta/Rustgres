use postgres::{Client, NoTls};
use std::{env};

// home/ryzenmaster/.cargo/config -> path to env
fn main () {
    //select(String::from("SELECT id, name, data FROM person")).unwrap();
    generic(String::from("INSERT INTO person (name, data) VALUES ($1, $2)")).unwrap();
}

fn select (query: String) -> Result<(), postgres::Error> {
    let mut client: Client = connect();
    for row in client.query(&query, &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }
    Ok(())
}

fn generic (query: String) -> Result<(), postgres::Error> {
    let mut client: Client = connect();
    client.execute(&query, &[&String::from("Rust"), &String::from("Rust is awesome!")])?;
    //client.execute(&query, &[])?;
    Ok(())
}

fn connect () -> Client {
    let host: String = env::var("HOST").unwrap();
    let user: String = env::var("USERNAME").unwrap();
    let password: String = env::var("PASSWORD").unwrap();
    let dbname: String = env::var("DBNAME").unwrap();
    match Client::connect(&format!("host={} user={} password={} dbname={}", host, user, password, dbname), NoTls) {
        Ok(client) => client,
        Err(err) => panic!("Error connecting: {}", err),
    }
}
