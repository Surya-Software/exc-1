/*
 *   Copyright (c) 2020 
 *   All rights reserved.
 */

pub mod read_create {
    use std::fs::File;
    use std::io::BufReader;
  
 pub fn read_file (f:&str) ->  BufReader<File> {
                         let file_handle= File::open(f).expect("could not open the input file");
                         let bufreader = BufReader::new(file_handle);
                         return bufreader;
                       }

 pub fn create_file (file:&str) -> File {
     let file_handle = File::create(file).expect("unable to create a output file");
     return file_handle;
 }
}



pub mod operation {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;
    use std::collections::HashMap;
    struct Person {
        first_name: String,
        amount: String,
      
      }
pub fn line_by_line (ip_file_ref:BufReader<File>) -> HashMap<std::string::String,i32> {
   
    let mut cnt = 0;
    let mut amt = HashMap::new();
    for line_iter in ip_file_ref.lines() {
        
            let line = line_iter.expect("could not read the line");

            let seperated_str: Vec<&str> = line.split(",").collect();
            let person = Person{first_name:String::from(seperated_str[0]), amount:String::from(seperated_str[2])};
          
             let mut total_amount: i32 = person.amount.parse().expect("could not parse amount into integer");
              if cnt != 0 {
                      if amt.contains_key(seperated_str[0]) {
                           let val= amt.get(&person.first_name).expect("did not find the key in the hashmap, provide a valid key");
                           total_amount = *val + total_amount;
                        }
                   }
              amt.insert(person.first_name,total_amount);
               cnt = cnt + 1;

            }
        return amt;
     }
}


pub mod write {
    
use std::collections::HashMap;
 use std::fs::File;
 use std::io::Write;
    pub fn write_file(amt:HashMap<std::string::String,i32>,f1:&mut File) {

    for (key,value) in amt.iter() {
        writeln!(f1, "{},{}", key, value).expect("could not write into the output file");
         }
     }
}