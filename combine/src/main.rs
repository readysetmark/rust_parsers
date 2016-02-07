extern crate combine;

use combine::{char, digit, many, parser, Parser, ParserExt, ParseResult, ParseError, State};
use combine::combinator::FnParser;
use combine::primitives::Stream;

#[derive(PartialEq, Debug)]
struct Date {
	year: i32,
	month: i32,
	day: i32
}


fn two_digits_to_int((x, y): (char, char)) -> i32 {
    let x = x.to_digit(10).expect("digit");
    let y = y.to_digit(10).expect("digit");
    (x * 10 + y) as i32
}

//Parsers which are used frequntly can be wrapped like this to avoid writing parser(fn_name) in
//several places.
fn two_digits<I>() -> FnParser<I, fn (State<I>) -> ParseResult<i32, I>>
where I: Stream<Item=char> {
    fn two_digits_<I>(input: State<I>) -> ParseResult<i32, I>
    where I: Stream<Item=char> {
    	println!("State.position = {:?}", input.position);

        (digit(), digit())
            .map(two_digits_to_int)
            .parse_state(input)
    }
    parser(two_digits_)
}

/// Parses a date
/// 2015-10-17
fn date<I>(input: State<I>) -> ParseResult<Date, I>
where I: Stream<Item=char> {
	(many::<String, _>(digit()), char('-'), two_digits(), char('-'), two_digits())
		.map(|(year, _, month, _, day)| {
			Date {
				year: year.parse().unwrap(),
				month: month,
				day: day
			}
		})
		.parse_state(input)
}

fn main() {
	let result : Result<(Date, &str), ParseError<&str>> = parser(date).parse("2015-10-17");

	match result {
		Ok((date, remaining_input)) => {
			println!("{:?}", date);
			println!("{:?}", remaining_input)
		},
		Err(err) => println!("{}", err)
	}
}
