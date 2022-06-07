#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use postgres::{Client, NoTls};
use std::{env};
use regex::{Regex, RegexSetBuilder};
// home/ryzenmaster/.cargo/config -> path to env
fn main () {
    //select(String::from("SELECT id, name, data FROM person")).unwrap();
    /* match generic(String::from("INSERT INTO person (name, data) VALUES ($1, $2)")) {
        Ok(updated) => println!("updated {} row(s)", updated),
        Err(e) => println!("Error: {}", e),
    }; */
    let ype: UseCase = defineUseCase(String::from("drop INTO person (name, data) VALUES ($1, $2)"));
    match ype {
        UseCase::ReturnValue => println!("will return the value"),
        UseCase::ReturnCount => println!("will return the rows affected"),
        UseCase::ReturnBool => println!("will return true or false"),
        UseCase::Unknown => println!("bad sintax"),
    };
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

fn generic (query: String) -> Result<u64, postgres::Error> {
    let mut client: Client = connect();
    let name:String = String::from("Gustavo");
    let data:String = String::from("Rust is awesome!");
    client.execute(&query, &[&name, &data.as_bytes()])
    //client.execute(&query, &[])?;
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

enum UseCase {
    ReturnValue,
    ReturnCount,
    ReturnBool,
    Unknown,
}
/* querie types {
    Select,
    Update,
    Insert,
    Delete,
    Create,
    Drop,
    Alter,
    Grant,
    Revoke,
    Begin,
    Commit,
    Rollback,
    Unknown,
} */

fn defineUseCase(query: String) -> UseCase {
    let set = RegexSetBuilder::new(&[
        r"^select",
        r"^insert",
        r"^update",
        r"^delete",
        r"^alter",
        r"^grant",
        r"^revoke",
        r"^drop",
        r"^create",
        //r"/^begin",
        //r"/^commit",
        //r"/^rollback",
    ]).case_insensitive(true).build().unwrap();
    let matches: Vec<usize> = set.matches(&query).into_iter().collect();
    match matches[0] {
        0 => UseCase::ReturnValue,
        1..=3 => UseCase::ReturnCount,
        4..=8 => UseCase::ReturnBool,
        _ => UseCase::Unknown,
    }
}
