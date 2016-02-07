#[macro_use]
extern crate nom;

use nom::{digit};
use std::str;
use std::str::{FromStr};

// named!(two_digits<&[u8], i32>,
// 	chain!(
// 		a: digit ~
// 		b: digit ,
// 		|| {
// 			//let x = a.to_digit(10).expect("digit");
// 			//let y = b.to_digit(10).expect("digit");
// 			//(x * 10 + y) as i32;
// 			[a, b];
// 		}));

named!(year<i32>,
	map_res!(
		map_res!(
			many1!(digit),
			str::from_utf8
		),
		FromStr::from_str
	)
);


fn main() {
	let r = year(b"1993");
    println!("Hello, world! We parsed {:?}", r);
}
