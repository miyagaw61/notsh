extern crate renert;

use renert::*;
use std::io::{stdout, Write};

fn read_and_exe() {
	eprint!("$ ");
	input! {
		cmd: String
	};
	process(&[&cmd]);
}

fn main() {
	loop {
		read_and_exe();
	}
}
