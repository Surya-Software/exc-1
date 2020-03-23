/*
 *   Copyright (c) 2020 
 *   All rights reserved.
 */
    extern crate clap;
    
    use clap::{Arg, App};
   
    
    fn main() {
    
      let matches = App::new("My Training Program")
      .version("0.1.0")
      .author("XYZ")
      .about("Teaches me argument parsing")
      .arg(Arg::with_name("file")
               .short("f")
               .long("file")
               .takes_value(true)
               .help("A cool file"))
      .arg(Arg::with_name("out")
               .short("o")
               .long("out")
               .takes_value(true)
               .help("Five less than your favorite number"))
      .get_matches();
    
      let input_file = matches.value_of("file").expect("different number of arguments are provided or argument is missing. Provide the arguments correctly");
      let output_file = matches.value_of("out").expect("different number of arguments are provided or argument is missing. Provide the arguments correctly");
      let ip_file_ref = lib::read_create::read_file(input_file);
      let mut op_file_ref = lib::read_create::create_file(output_file);
    let hash_map = lib::operation::line_by_line(ip_file_ref);
    lib::write::write_file(hash_map,&mut op_file_ref);
    }
