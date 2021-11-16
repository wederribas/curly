//////////////////////////////////////////////////////////////////////////
//                        _                                             //
//      ___  _   _  _ __ | | _   _                                      //
//     / __|| | | || '__|| || | | |                                     //
//    | (__ | |_| || |   | || |_| |                                     //
//     \___| \__,_||_|   |_| \__, |                                     //
//                           |___/                                      //
//                                                                      //
// Copyright (C) 2021 - 2021, Weder Ribas, <me@wederribas.com>          //
//                                                                      //
// Welcome to the curly project. This is a cURL clone, or if you        //
// prefer, a command line tool do transfer data using URLs, heavily     //
// inspired by the cURL project.                                        //
//                                                                      //
// Weder's goal when he started this project was just to learn          //
// the Rust programming language and also learn more about computer     //
// networking and protocols. The cURL project served as the perfect     //
// inspiration for that intent. Weder loves cURL and thought it would   //
// be cool to use it as an inspiration.                                 //
//                                                                      //
//////////////////////////////////////////////////////////////////////////

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "curly", about = "A cURL clone written in Rust.")]
struct Cli {
    #[structopt(short, long)]
    help: bool,
    url: String,
}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", args)
}
