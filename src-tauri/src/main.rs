#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

//use tauri::{window::WindowBuilder, AppHandle, WindowUrl};
use postgres::{Client, NoTls, SimpleQueryMessage};
use regex::{Regex, RegexSetBuilder};
use random::Source;
use serde::ser::{Serializer, SerializeSeq, SerializeMap, SerializeStruct, SerializeTupleVariant};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            randInt,
            execute,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn randInt() -> f32 {
    println!("oi");
    let mut source = random::default();/* .seed([42, 69]) ; */
    let val: u16 = source.read::<u16>();
    let result: f32 = val as f32 / 65535.0;
    result.into() //returns value to front end
}

#[tauri::command]
fn execute (query: String) -> String {
    let result: Result<ReturnType, String> = matchQuery(&query);
    match result {
        Ok(data) => {
            let j: String = serde_json::to_string(&data).unwrap();
            j.into()
        }
        Err(msg) => msg.into()
    }
}

fn matchQuery(query: &String) -> Result<ReturnType, String> {
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
        0 => simpleQuerySelect(&query),
        1..=3 => simpleQueryModify(&query),
        4..=8 => simpleQueryBoolean(&query),
        _ => Err("Something terrible happened".to_string()),
    }
}

fn simpleQuerySelect(query: &String) -> Result<ReturnType, String> {
    let mut client: Client = connect().expect("error while connecting to database");
    let rows: Vec<SimpleQueryMessage> = match client.simple_query(&query) {
        Ok(rows) => rows,
        Err(e) => return Err(format! ("{}", e))
    };

    let mut result: Vec<Vec<String>> = Vec::new();
    let mut columns: Vec<String> = Vec::new();
    let mut modified: u32 = 0;
    for (i, message) in rows.iter().enumerate() {
        match message {
            SimpleQueryMessage::Row(b) => {
                let mut iter: Vec<String> = Vec::new();
                for num in 0..b.len() {
                    iter.push(match b.get(num) {
                        Some(x) => x.to_string(),
                        None => String::from("null"),
                    })
                }
                result.push(iter);
                if i == 0 {
                    for cols in b.columns() {
                        columns.push(cols.name().to_string());
                    }
                }
            },
            SimpleQueryMessage::CommandComplete(dk) => modified = u32::try_from(*dk).unwrap(),
            _ => result.push(vec![String::from("not implemented")]),
        };
    };
    return Ok(ReturnType::Value{values: result, columns: columns, changes: modified });
}

fn simpleQueryModify(query: &String) -> Result<ReturnType, String> {
    let mut client: Client = connect().expect("error while connecting to database");
    let rows: Vec<SimpleQueryMessage> = match client.simple_query(&query) {
        Ok(rows) => rows,
        Err(e) => return Err(format! ("{}", e))
    };

    let mut modified: u32 = 0;
    for i in rows.iter() {
        match i {
            SimpleQueryMessage::CommandComplete(dk) => modified = u32::try_from(*dk).unwrap(),
            SimpleQueryMessage::Row(b) => {},
            _ => panic!("something terrible happened"),
        };
    };
    return Ok(ReturnType::Count{changes: modified });
}

fn simpleQueryBoolean(query: &String) -> Result<ReturnType, String> {
    let mut client: Client = connect().expect("error while connecting to database");
    let rows: Vec<SimpleQueryMessage> = match client.simple_query(&query) {
        Ok(rows) => return Ok(ReturnType::Bool{ result: true }),
        Err(e) => return Err(format! ("{}", e))
    };
}

fn connect () -> Result<Client, postgres::Error> {
    let host: String = String::from("localhost");
    let user: String = String::from("postgres");
    let password: String = String::from("password123");
    let dbname: String = String::from("rustgres");
    Client::connect(&format!("host={} user={} password={} dbname={}", host, user, password, dbname), NoTls)
}

#[derive(Serialize, Deserialize)]
enum ReturnType {
    Value { columns: Vec<String>, values: Vec<Vec<String>>, changes: u32 },
    Count { changes: u32 },
    Bool { result: bool },
    Unknown { message: String },
}
