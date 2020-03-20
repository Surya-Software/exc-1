extern crate clap;
use clap::{Arg, App};

fn main () {

       let matches = App::new("My Training Program")
                     .version("0.1.0")
                    .author("XYZ")
                    .about("Teaches me argument parsing")
            .arg(Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .takes_value(true))
            .arg(Arg::with_name("out")
                     .short("o")
                     .long("out")
                     .takes_value(true))
       .get_matches();

    let myfile = matches.value_of("file").expect("argument missing");
    let ofile = matches.value_of("out").expect("argument missing");
    let f3 = lib::filerw::rf(myfile);
    let mut f1 = lib::filerw::wf(ofile);
    let (y, m, d) = lib::system_date::get_date();
    lib::oper::line_by_line(f3,&mut f1,y,m,d);
}
