pub fn value_of_x_message(x: i32) -> String {
    format!("The value of x is {x}")
}

pub fn labeled_measurement(value: i32, unit_label: char) -> String {
    format!("The measurement is: {value}{unit_label}")
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
    if amount == 0 {
        Err(String::from("Deposit amount must be positive"))
    } else {
        *balance += amount;
        Ok(())
    }
}

pub fn loop_with_break_value() -> i32 {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter == 40 {
            break counter * 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_simple_messages() {
        assert_eq!(value_of_x_message(5), "The value of x is 5");
        assert_eq!(labeled_measurement(5, 'h'), "The measurement is: 5h");
    }

    #[test]
    fn withdraw_rejects_insufficient_funds() {
        let mut balance = 100;
        assert!(withdraw(150, &mut balance).is_err());
        assert_eq!(balance, 100);
    }

    #[test]
    fn withdraw_and_deposit_update_the_balance() {
        let mut balance = 100;
        withdraw(30, &mut balance).unwrap();
        deposit(20, &mut balance).unwrap();
        assert_eq!(balance, 90);
    }

    #[test]
    fn deposit_rejects_a_zero_amount() {
        let mut balance = 100;
        assert!(deposit(0, &mut balance).is_err());
    }

    #[test]
    fn loop_can_return_a_value_via_break() {
        assert_eq!(loop_with_break_value(), 80);
    }
}
