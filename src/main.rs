#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use whirlpool::{Whirlpool, Digest};
use std::io::{self, BufRead};


use std::io::Write;
use std::io::Read;
use rocket::Request;
use rocket::response::content::Json;
use rocket_contrib::templates::Template;
use serde::Serialize;


#[get("/")]
fn index() -> Template {
  #[derive(Serialize)]
  struct Userdata {
    login: String,
    password: String
  }
  let data = Userdata {
    login: String ::from("Jane"),
    password: String::from("Doe")
  };

  Template::render("home", data)
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    print!("{}", req);
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

fn write_file(filename : &str, content : &String)  {
  let mut file = std::fs::File::create(filename).expect("create failed");
  file.write_all(content.as_bytes()).expect("write failed");
}

fn read_file(filename : &str) -> String {
  let mut file = std::fs::File::open(filename).unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   contents
}

fn hash(content : &str)-> String{
  let mut hasher = Whirlpool::new();
  hasher.update(content);
  let hash = hasher.finalize();

  hash[0].to_string()
}

fn set_pwd(){
  println!("Set password");
  let mut newpwd = String::new();
  let stdin2 = io::stdin();
  stdin2.lock().read_line(&mut newpwd).unwrap();
  //lecture de l'input 
  let setpwd = hash(&newpwd);
  write_file("version.txt", &setpwd);
  println!(".{}.", setpwd);
  //hash de l'input et stockage dans hash.txt
}

fn check_pwd()->u8{

  let raw_pwd = ask_pwd();
  let hashed = hash(&raw_pwd);
  //write_file("hash.txt", &hashed);
  //println!("{}", read_file("version.txt"));
  let hash_lu =  read_file("version.txt");
  if hashed == hash_lu{
    println!("OK pwd");
    return 1
  }
  else {
    println!("Error pwd");
    return 0
  }
}

fn ask_pwd()->String{
  let mut pwd = String::new();
  print!("Enter password : ");
  let stdin = io::stdin();
  stdin.lock().read_line(&mut pwd).unwrap();
  pwd
}

fn send_password(){
  
}

fn main() {
  
  set_pwd(); 
  println!("{}",check_pwd());
  

 
  
  /*
  rocket::ignite()
    .register(catchers![not_found])
    .mount("/", routes![index])
    .mount("/api", routes![hello])
    .attach(Template::fairing())
    .launch();
    */
}
