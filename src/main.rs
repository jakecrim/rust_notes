// Rust notes

fn main () {
	let mut x = String::from("Hey there");
	let mut y = String::from("Hello dawg woof");
	println!("{}",first_word(&x));
	println!("{}",second_word(&y));
}

fn first_word(x : &String) -> &str {
	let bytes = x.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &x[0..i];
		}
	}

	&x[..]
}

fn second_word(x : &String) -> &str {
	let mut bytes = x.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' '{
			let y = &x[i+1..];
			bytes = y.as_bytes();

			for (i, &item) in bytes.iter().enumerate() {
				if item == b' ' {
					return &y[0..i];
				}
			}
			// Return this IF there were only two words.
			return &y[..];
		}
	}

	&x[..]
}




//__________________________________________________________________________________________________________ //

//--------------------------------------------------//
//*** 			GENERAL NOTES 		               ***
//*** Info: Random Notes I found either important, ***
//*** hard to remember, funny, cool, or who knows  ***  
//*** why but I added it to this list			   ***
//--------------------------------------------------//
// Notes:
// 1. Variables immutable by default
// 2.Constants ALWAYS immutable, type must be annotated  ex: const MAX_POINTS: u32 = 758000;
// 3. Scaler variables: Ints, floats, bools, and  chars
// Length	Signed	Unsigned
// 8-bit	i8		u8
// 16-bit	i16		u16
// 32-bit	i32		u32
// 64-bit	i64		u64
// 128-bit	i128	u128
// arch		isize	usize
// 4. Integer literals
// Number literals	Example
// Decimal			98_222
// Hex				0xff
// Octal			0o77
// Binary			0b1111_0000
// Byte (u8 only)	b'A'
// 5. Floats can be either f32 or f64 (32 and 64 bits respectively)...default is f64
//ex: 
	// fn main() {
    // 		let x = 2.0; // f64

    // 		let y: f32 = 3.0; // f32
	//}
// 6. bools example
	// fn main() {
	// 	let t = true;

	// 	let f: bool = false; // with explicit type annotation
	// }
// 7. Tuples AND Arrays are fixed length.
// 8. Arrays allocate to stack instead of the heap, are used instead of vectors when you KNOW the num of elements wont need to change, like an array
//    containing the days of the week.  
// 9.  Functions can be defined anywhere in the program.
// 10. In rust cannot do statements like: x = y = 6
// 11. *IMPORTANT* Expressions (return something) lines do not contain a semi colon. Statements(return NOTHING) DO have a semi colon.
// 12. Unlike other languages, conditions for if statements MUST be a bool.
// 13. *IMPORTANT* (Straight out of the rust book)
		// To ensure memory safety, there’s one more detail to what happens in this situation in Rust. Instead of trying to copy the allocated memory, Rust considers s1 to no longer be valid and, therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens when you try to use s1 after s2 is created; it won’t work:

		// This code does not compile!
		// let s1 = String::from("hello");
		// let s2 = s1;

		// println!("{}, world!", s1);
		// You’ll get an error like this because Rust prevents you from using the invalidated reference:


		// error[E0382]: use of moved value: `s1`
		//  --> src/main.rs:5:28
		//   |
		// 3 |     let s2 = s1;
		//   |         -- value moved here
		// 4 |
		// 5 |     println!("{}, world!", s1);
		//   |                            ^^ value used here after move
		//   |
		//   = note: move occurs because `s1` has type `std::string::String`, which does
		//   not implement the `Copy` trait
// 14. *FUTURE notes on copying, moving, and losing access to data 
// 15. References are immutable by default just like variables, know you can have only ONE mutable reference to a piece of data in a particular SCOPE.
// 16. 
// 17.
// 18.
// 19.
// 20.




//__________________________________________________________________________________________________________ //


//---------------------------------------------//
//*** INFINITE loop,  unless x is ever >= 10 ***
//---------------------------------------------//
// fn main() {
// 	let mut x = 0;

// 	loop {
// 		x += 1;

// 		println!("Value of x is {}", x);

// 		if x >= 10 {
// 			break;
// 		}
		
// 	}
// }

//__________________________________________________________________________________________________________ //

//---------------------------------------------//
//*** WHILE loop,  while n <= 50, also only prints multiples of 5 ***
//---------------------------------------------//
// fn main() {
	// let mut	n = 1;

	// while n <=  50 {
	// 	if n % 5 == 0 {
	// 		println!("n is {}", n);
	// 	}

	// 	n += 1;                        
	// } 
// }

//__________________________________________________________________________________________________________ //

//---------------------------------------------//
//*** FOR loop,  a few options for iterating ***
//---------------------------------------------//
// fn main() {
// 	// makes a range from 0-129, nums is of type range
// 	let nums = 0..130;

// 	//must have an iterator
//     for i in nums {
// 		println!("Num is {}", i );
// 	}        
	
// 	// can use a vector
// 	let fruits = vec!["Watermelon", "Appple", "Orange"];

// 	//must have an iterator, calls iterator method of fruits vector
// 	for a in fruits.iter() {
// 		println!("Fruit type is: {}", a );
// 	}        
// }
//__________________________________________________________________________________________________________ //

//---------------------------------------------//
//*** ENUM def && MATCH usage ***
//---------------------------------------------//
// enum cardinal_direction {
// 	North,
// 	South,
// 	East,
// 	West
// }

// fn main() {
// 	let player_cardinal:cardinal_direction = cardinal_direction:: North;
// 	//like a switch statement
// 	match player_cardinal {
// 		cardinal_direction::North => println!("Heading North!"),
// 		cardinal_direction::South => println!("Heading South!"),
// 		cardinal_direction::East => println!("Heading East!"),
// 		cardinal_direction::West => println!("Heading West!"),
// 	}
// }
//__________________________________________________________________________________________________________ //

//---------------------------------------------//
//*** CONST defintion ***
//---------------------------------------------//
// const MAX_NUM: u8 = 20;
//
// fn main() {

// 	for n in 1..MAX_NUM {
// 		print!("{}", n );
// 	}


// }
//__________________________________________________________________________________________________________ //

//---------------------------------------------//
//*** TUPLES ***
//---------------------------------------------//
// fn main() {
// 	// creating a tuple 
// 	// NOTE: CAN have nested tuples
// 	let johnshon = (5, "ch" , 20.5 , 23);
//	// to be more specific...
//  let tup: (i32, f64, u8) = (500, 6.4, 1);
// 	println!("{}", johnshon.2 );
// }
//__________________________________________________________________________________________________________ //
//---------------------------------------------//
//*** ARRAYS ***
// Info: Indexed the same as  C based langs
//---------------------------------------------//
// fn main() {
//     let a = [1, 2, 3, 4, 5];
//					// the 5  indicates the array size
// 		let a: [i32; 5] = [1, 2, 3, 4, 5];
// }
//
//__________________________________________________________________________________________________________ //

//				* ALSO *
// Info: This shows a pattern to turn the tup into those 3 seperate variables, also demonstrates how to index a tuple 
// fn main() {
//     let tup = (500, 6.4, 1);
//
//     let (x, y, z) = tup;
//
//     println!("The value of y is: {}", y);
//
//	   let x: (i32, f64, u8) = (500, 6.4, 1);
//
//		let five_hundred = x.0;
//
//		let six_point_four = x.1;
//
//		let one = x.2;
// }
//__________________________________________________________________________________________________________ //

//---------------------------------------------//
//*** Calling Functions ***
// Info: Youhave to declare the type  of each parameter
//---------------------------------------------//
// fn main() {
// 	let x = 5;
// 	print_number(x);
// 	if is_even(x){
// 		println!("Is even!");
// 	}
// 	else {
// 		println!("Is odd!", );
// 	}
// }

// fn print_number(num:u32){
// 	println!("number is {}", num);

// }

// // fn accepts u32 num and "returns" a bool
// fn is_even(num:u32) -> bool{
// 	return (num % 2 == 0);
// }
//__________________________________________________________________________________________________________ //


//---------------------------------------------//
//*** Shadowing ***
//---------------------------------------------//
// fn main() {
// 	let mut x = 20;
// 	{
// 		let x = 15;

// 		// do stuff with this 15, then x will go back to original value, called "shadowing"
// 	}

// 	// will print out 20
// 	println!("{}", x);
// }
//------------------------------------------------------//
//*** Shadowing Cont... ***								//
//  Info: This allows for an immutable var to			//
//		   occasionally be redifined, but				//
//         to in general  still stay an immutable var	//
//------------------------------------------------------//
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     let x = x * 2;

//     println!("The value of x is: {}", x);
// }
//__________________________________________________________________________________________________________ //

//---------------------------------------------//
//*** 			Guessing Game Program 		     ***
//*** Info: Demonstrates usage of rand crate and ***
//*** some basic error handling					 ***
//---------------------------------------------//
// use std::cmp::Ordering;
// use std:: io;
// use rand:: Rng;

// fn main() {
// 	// to learn how to use the crates, like rand, do cargo doc --open in the command line to open up documentation in your browser
// 	let secret_num = rand::thread_rng().gen_range(1, 101);
// 	loop {
// 		println!("Guess the number!");

// 		println!("Input your guess:");
// 		// A new instance of a String, which is growable and UTF-8 encoded text
// 		// '::' means new is an associated function of the String type, not the instance of the string. Similar to a static method in other languages.
// 		let mut guess = String::new();

// 		// calls read_line which takes in user input into guess, checks whether it returns err or ok, and depending on that it may or may not print out the failure message
// 		// basically the expect part does some error checking right there, and will print the passed in err message and crash the program if err is returned from read_line
// 		io::stdin().read_line(&mut guess)  
// 		.expect("Failed to read");	//NOTE: will default to type i32

// 		//						trim eliminates new lines and whitespaces that might exist in the string we are attempting to cast to an u32
// 		let guess: u32 = match guess.trim().parse() {
// 			Ok(num) => num,
// 			Err(_)=> continue,	//the '_' is a catchall val that will continue if parse's result type return val is Err
// 		};

// 		println!("User guessed: {}", guess );

// 		match guess.cmp(&secret_num){
// 			Ordering::Less => println!("Too small!"),
// 			Ordering::Greater => println!("Too large!"),
// 			Ordering::Equal => 
// 			{
// 				println!("Correct!");
// 				break;
// 			}
// 		}
// 	}

// }
//__________________________________________________________________________________________________________ //

//---------------------------------------------//
//*** References ***
//---------------------------------------------//
// fn main() {
// 	let mut jacob = 21;

// 	// creating a REFERENCE to jacob, will behave exactly as 'jacob' does
// 	//let jacobR = &jacob;

// 	// creating a mutable REFERENCE to jacob, that can be changed and WILL change  'jacob's value
// 	let jacobRM = &mut jacob;
	
// 	*jacobRM =  22;
// 	println!("{}", jacob );

// }
//__________________________________________________________________________________________________________ //
//* ANOTHER REFERENCES EXAMPLE *
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
//
//__________________________________________________________________________________________________________ //



//---------------------------------------------//
//*** Range and Reverse in a For Loop ***
//---------------------------------------------//
	// fn main() {
	// 		for number in (1..4).rev() {
	// 			println!("{}!", number);
	// 		}
	// 		println!("LIFTOFF!!!");
	// }
	//__________________________________________________________________________________________________________ //
//---------------------------------------------//
//*** Cloning ***
// info: So that s1 and s2 dont point to the same
//		 piece of memory, is more expensive than the 
//		 normal copying, which is really a "move"
//---------------------------------------------//

// fn main() {
// 	let s1 = String::from("hello");
// 	let s2 = s1.clone();

// 	println!("s1 = {}, s2 = {}", s1, s2);
// }
//
//__________________________________________________________________________________________________________ //
//---------------------------------------------//
//*** Find Nth Fibonacci Number ***
//---------------------------------------------//
// use std::io;
// //use std::cmp::Ordering;

// fn main() {

// 	let mut num = String::new();
// 	println!("What nth Fibonacci number would you like to find?");

// 	// calls read_line which takes in user input into guess, checks whether it returns err or ok, and depending on that it will print out the failure message
// 	// basically the expect part does some error checking right there, and will print the passed in err message and crash the program if err is returned from read_line
// 	io::stdin().read_line(&mut num)
// 	.expect("Failed to read");	//NOTE: will default to type i32

// 	let num: u32 = num.trim().parse().expect("Was not an integer!");

// 	let nth = fib_num(num);

// 	println!("The nth Fibonnacci Number was: {}", nth );
// }

// fn fib_num(x: u32) -> u32{
// 	if x <= 1 {
// 		return x;
// 	}
// 	return fib_num(x-1) + fib_num(x-2);

// }
//
//__________________________________________________________________________________________________________ //
//
//---------------------------------------------------------------------------------------//
//*** SLICES: example, return first word in string, and second word in string function ***
//--------------------------------------------------------------------------------------//
//
// fn main () {
// 	let mut x = String::from("Hey there");
// 	let mut y = String::from("Hello dawg woof");
// 	println!("{}",first_word(&x));
// 	println!("{}",second_word(&y));
// }

// fn first_word(x : &String) -> &str {
// 	let bytes = x.as_bytes();

// 	for (i, &item) in bytes.iter().enumerate() {
// 		if item == b' ' {
// 			return &x[0..i];
// 		}
// 	}

// 	&x[..]
// }

// fn second_word(x : &String) -> &str {
// 	let mut bytes = x.as_bytes();

// 	for (i, &item) in bytes.iter().enumerate() {
// 		if item == b' '{
// 			let y = &x[i+1..];
// 			bytes = y.as_bytes();

// 			for (i, &item) in bytes.iter().enumerate() {
// 				if item == b' ' {
// 					return &y[0..i];
// 				}
// 			}
// 			// Return this IF there were only two words.
// 			return &y[..];
// 		}
// 	}

// 	&x[..]
// }