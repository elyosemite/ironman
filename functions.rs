pub fn another_function(x: i32) {
        println!("The value of x is {x}");
}

pub fn print_labbled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

pub fn withdraw(amount: u32, balance: &mut u32) -> Result<(), String> {
    if *balance < amount {
        Err(String::from("Insufficient funds"))
    } else {
        *balance -= amount;
        Ok(())
    }
}

pub fn deposit(amount: u32, balance: &mut u32) -> Result<(), String> {
    if amount <= 0 {
        Err(String::from("Deposit amount must be positive"))
    } else {
        *balance += amount;
        Ok(())
    }
}