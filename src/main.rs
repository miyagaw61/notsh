extern crate renert;

use renert::*;

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
