#[macro_use]
extern crate clap;

use clap::{Arg, App};
use std::io::{stdin, stdout};

mod buff;

use buff::Buffer;

fn main() {
    let matches = App::new("mbuffer")
        .about("memory buffer")
        .version(crate_version!())
        .arg(Arg::with_name("buffer-size")
            .short("s")
            .value_name("SIZE")
            .help("maximum size of the buffer")
            .default_value("1000000000")
            .takes_value(true))
        .arg(Arg::with_name("segment-size")
            .short("g")
            .help("size of a segment of the buffer")
            .default_value("65536"))
        .get_matches();

    let segment_length = matches
        .value_of("segment-size")
        .unwrap();
    let segment_length = segment_length
        .parse::<usize>()
        .expect(&format!("{} is not a number", segment_length));
    let buffer_size = matches
        .value_of("buffer-size")
        .unwrap();
    let buffer_size = buffer_size
        .parse::<usize>()
        .expect(&format!("{} is not a number", buffer_size));

    let mut b = Buffer::new(
        segment_length,
        buffer_size,
        Box::new(stdin()),
        Box::new(stdout())
    );
    if let Err(e) = b.join() {
        eprintln!("{}", e);
        return;
    };
}
