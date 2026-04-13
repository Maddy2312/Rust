fn main() {
    // Ownership
    // One value = one owner
    // Only one owner at a time
    // Owner out of scope = memory freed

    // let x:String = String::from("Hello");
    // process_integer(x);
    // println!("The value of x in main() is {}", x);

}

// fn process_integer(item:String){
//     println!("The value of x in process_integer() is {}", item);
// }

fn _add(item1:u8, item2:u8)->u8{
    return item1*item2;
}

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