// Rust notes

//currently learning about references here...
fn main() {
	let mut jacob = 21;

	// creating a REFERENCE to jacob, will behave exactly as 'jacob' does
	//let jacobR = &jacob;

	// creating a mutable REFERENCE to jacob, that can be changed and WILL change  'jacob's value
	let jacobRM = &mut jacob;
	
	*jacobRM =  22;
	println!("{}", jacob );

}

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

//---------------------------------------------//
//*** TUPLES ***
//---------------------------------------------//
// fn main() {
// 	// creating a tuple 
// 	// NOTE: CAN have nested tuples
// 	let johnshon = (5, "ch" , 20.5 , 23);

// 	println!("{}", johnshon.2 );


// }


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