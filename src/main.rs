// use std::any::type_name;
// use std::io::{self, Write};

// fn print_type_of<T>(_: &T) {
//     println!("{}", type_name::<T>());
// }


fn main() {

    // let x = 10;
    // print_type_of(&x);


    // let mut input = String::new();
    // print!("Enter a number: ");
    // io::stdout().flush().unwrap();
    // io::stdin().read_line(&mut input).expect("fail to readline");
    // println!("You entered: {}", input);


}

// fn process_integer(item:String){
//     println!("The value of x in process_integer() is {}", item);
// }

// fn _add(item1:u8, item2:u8)->u8{
//     return item1*item2;
// }

fn _variable_and_function(){
    // Scalar Types

    //     Integer

    //     Floating Pointer

    // Compound Types 

    //     Primitive Types
        
    //         Array
    //         Tuple
            // let emp_info:(&str, u8) = ("Mandy", 24);
            // let emp_name: &str = emp_info.0;
            // let emp_age: u8 = emp_info.1;

            // let (employee_name, employee_age) = emp_info;



            // println!("Employee Name= {} and Employee Age= {}", employee_name, employee_age);
            // println!("Employee Name= {} and Employee Age= {}", emp_name, emp_age);

    //     Complex Types

    //         Strings
                // &str - Fixed Length Strings - Special memory
                // String - Dyanamic Length Strings - Heap memory

                // let mut string_literal:&str = "Hello";
                // println!("This is String literal: {}", string_literal);

                // let mut string_literal:String = String::from("Hello");
                // string_literal.push_str("string");
                // println!("This is String literal: {}", string_literal);



                // let num1: u8 = 10;
                // let num2: u8 = 12;
                // let result: u8 = add(num1, num2);
                // println!("The sum of num1 and num2 is {}", result);
}

fn _ownership_and_borrowing(){
    // Ownership
    // One value = one owner
    // Only one owner at a time
    // Owner out of scope = memory freed

    // let x:String = String::from("Hello");
    // // process_integer(x);
    // println!("The value of x in main() is {}", x);

    // // Borrowing
    // let mut word:String = String::from("Hello");
    // let w1 = &mut word;
    // w1.push_str(" string");
    // println!("w1 is {}", w1);
    
    // let w2 = &mut word;
    // w2.push_str(" string");
    // println!("w1 is {}", w2);

    // let r1 = &word;
    // let r2 = &word;
    // println!("r1 is {} and r2 is {}", r1, r2);


}
