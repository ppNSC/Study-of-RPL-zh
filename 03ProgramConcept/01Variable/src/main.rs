fn main() {
    // non variable
    let non_variable = 5;
    println!("The non-variable value is {non_variable}");

    // variable, mutable
    let mut variable = 6;
    println!("The variable value is {variable}");
    variable = 7;
    println!("The variable value is {variable} after modified");

    // const
    const ONE_HOUR_IN_SECONDS: u32 = 1 * 60 * 60;
    println!("One hour is {ONE_HOUR_IN_SECONDS} s");

    // shadowing 
    let name = "shicheng";
    {
        let name = name.len();
        println!("The length of name is: {name}");
    }
    println!("The name is: {name}");
}
