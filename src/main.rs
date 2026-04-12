fn main() {
    // Scalar Types

    //     Integer

                let mut num: u8 = 23;
                println!("This is stored in num: {}", num);
                num=199;
                println!("This is stored in num: {}", num);

    //     Floating Pointer

    // Compound Types 

    //     Primitive Types
        
    //         Array
    //         Tuple
            let emp_info:(&str, u8) = ("Mandy", 24);
            let emp_name: &str = emp_info.0;
            let emp_age: u8 = emp_info.1;

            let (employee_name, employee_age) = emp_info;



            println!("Employee Name= {} and Employee Age= {}", employee_name, employee_age);
            println!("Employee Name= {} and Employee Age= {}", emp_name, emp_age);

    //     Complex Types

    //         Strings
                // &str - Fixed Length Strings - Special memory
                // String - Dyanamic Length Strings - Heap memory

                let mut string_literal:&str = "Hello";
                println!("This is String literal: {}", string_literal);

                let mut string_literal:String = String::from("Hello");
                string_literal.push_str("string");
                println!("This is String literal: {}", string_literal);



                let num1: u8 = 10;
                let num2: u8 = 12;
                let result: u8 = add(num1, num2);
                println!("The sum of num1 and num2 is {}", result);
}

fn add(item1:u8, item2:u8)->u8{
    return item1*item2;
}