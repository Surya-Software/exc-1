/*
 *   Copyright (c) 2020 
 *   All rights reserved.
 */
pub mod read_create {
    use std::fs::File;
    use std::io::BufReader;

 pub fn read_file (f:&str) ->  BufReader<File> {
                         let file_handle= File::open(f).expect("could not open the file");
                         let bufreader = BufReader::new(file_handle);
                         return bufreader;
                       }

 pub fn create_file (f:&str) -> File {
     let f1 = File::create(f).expect("unable to create a file");
     return f1;
 }
}

pub struct Dobs {
  pub date:i32,
   pub month:i32,
  pub year:i32,
}

impl Dobs {

 pub fn get_date(&mut self) -> &Dobs{
    extern crate chrono;
    use chrono::{DateTime, Utc};
let system_date: DateTime<Utc> = Utc::now();
 let year= system_date.format("%Y").to_string();
 let date = system_date.format("%d").to_string();
 let month = system_date.format("%m").to_string();
  self.year= year.parse().expect("could not parse year into integer");
  self.month = month.parse().expect("could not parse month into integer");
 self.date = date.parse().expect("could not parse date into integer");
 return self;
 }
}

pub mod oper {
use std::fs::File;
use std::io::Write;
use std::io::{BufReader, BufRead};
struct Person {
dob: String,
first_name: String,
last_name: String,

}
pub fn line_by_line (read_file:BufReader<File>,write_file:&mut File,ye:i32,mo:i32,da:i32) {

for line_iter in read_file.lines() {

 let line = line_iter.expect("could not read a line");

  let separated_str: Vec<&str> = line.split(",").collect();

  let person = Person { dob: String::from(separated_str[2]),
     first_name: String::from(separated_str[1]),
     last_name: String::from(separated_str[0]),
    };

  let seperated_dob: Vec<&str> = (person.dob).split("-").collect();

  let year: i32 = seperated_dob[0].parse().expect("could not parse year into integer");
  let month: i32 = seperated_dob[1].parse().expect("could not parse month into integer");
  let date: i32 = seperated_dob[2].parse().expect("could not parse date into integer");


  let mut age = ye - year;
  age = age - 1;
  if month < mo {
      age = age + 1;
  }

  if month == mo {
      if date < da {
          age = age + 1;
      }
  }
  writeln!(write_file, "{}, {}, {}", person.first_name, person.last_name, age).expect("could not write into the file");
}
}
}
