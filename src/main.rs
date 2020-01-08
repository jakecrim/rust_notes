// Rust notes


fn main() {

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
// 3.
// 4.
// 5.
// 6.
// 7.
// 8.
// 9.




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

// 	println!("{}", johnshon.2 );


// }

//__________________________________________________________________________________________________________ //

//---------------------------------------------//
//*** Calling Functions ***
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