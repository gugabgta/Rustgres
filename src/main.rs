#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use postgres::{Client, NoTls, SimpleQueryMessage};
use std::{env};
use regex::{Regex, RegexSetBuilder};
// home/ryzenmaster/.cargo/config -> path to env
fn main () {
    /* let ype: UseCase = defineUseCase(String::from("drop INTO person (name, data) VALUES ($1, $2)"));
    match ype {
        UseCase::ReturnValue => println!("will return the value"),
        UseCase::ReturnCount => println!("will return the rows affected"),
        UseCase::ReturnBool => println!("will return true or false"),
        UseCase::Unknown => println!("bad sintax"),
    }; */
    let query = String::from("select * from person where name = 'Gustavo'");
    match select(query) {
        Ok(x) => println!("{:?}", x),
        Err(e) => println!("{:?}", e),
    };
}

fn select (query: String) -> Result<Vec<Vec<String>>, postgres::Error> {
    let mut client: Client = connect();
    let rows: Vec<SimpleQueryMessage> = match client.simple_query(&query) {
        Ok(rows) => rows,
        Err(e) => {
            println!("{}", e);
            return Err(e);
        }
    };

    let mut result: Vec<Vec<String>> = Vec::new();
    //accessing the first row
    for i in rows.iter() {
        let test = match i {
            SimpleQueryMessage::Row(b) => {
                let mut iter: Vec<String> = Vec::new();
                for num in 0..b.len() {
                    iter.push(match b.get(num) {
                        Some(x) => x.to_string(),
                        None => String::from("null"),
                    })
                }
                result.push(iter);
            },
            SimpleQueryMessage::CommandComplete(dk) => result.push(vec![String::from(format!("modified {} rows", dk))]),
            _ => result.push(vec![String::from("not implemented")]),
        };
    };
    Ok(result)
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
