use postgres::{ Client, NoTls };
use postgres::Error as PostgresError;
use std::net:: {TcpListener, TcpStream };
use std::io::{ Read, Write };
use std::env;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email, String
}

const DB_URL: &str = env!("DATABASE_URL");

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

fn main() {
    if let ERR(_) = set_database() {
        println!("Error setting database");
        return;
    }

    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    printlin!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

#[macro_use]
extern crate serde_drive;
