  // Logical operation
//use std::io
fn main (){
       /* let cond = 2.0 < (3 as f32);

        //let cond2 = true && cond; // logical AND operator
        //let cond3 = false || cond; // logical OR operator
        let cond3 = !(false && cond);

        println!("{}", cond3);
   */	

    // if else statement
	
	use std::io;
		println!("enter any friut of ur choice: ");
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("failed to read_line");
		let myfriut = input.trim();

	if myfriut == "cookie" {
		println!("I like cookies too"); 	
	}
	else if myfriut == "mango" {
		println!("that is mango")
	} 
	else {
		println!("Oh, that's too bad!");
	}
}
