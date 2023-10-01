#![allow(unused)]

trait Account {
    fn deposit(&mut self, dep_amnt: f64) -> Result<(), String>;
    fn withdraw(&mut self, wthdr_amnt: f64) -> Result<(), String>;
    fn balance(&mut self) -> f64;
}

struct BankAccount {
    account_number: u128,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, dep_amnt: f64) -> Result<(), String> {
        if dep_amnt < 0.0 || dep_amnt == 0.0 {
            Err("invalid input to complete deposit operation".to_string())
        } else {
            self.balance += dep_amnt;
            Ok(())
        }
    }
    fn withdraw(&mut self, wthdr_amnt: f64) -> Result<(), String> {
        if self.balance - wthdr_amnt.abs() < 0.0 {
            Err("insufficient funds to complete withdraw operation".to_string())
        } else {
            self.balance -= wthdr_amnt.abs();
            Ok(())
        }
    }
    fn balance(&mut self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut bank_account_alice = BankAccount {
        account_number: 848758115758,
        holder_name: "Alice Smith".to_string(),
        balance: 1000.0,
    };
    let mut bank_account_ben = BankAccount {
        account_number: 84875465758,
        holder_name: "Ben Walles".to_string(),
        balance: 500.0,
    };

    let dep_res = bank_account_alice.deposit(725.00);
    let with_res = bank_account_ben.withdraw(430.00);

    match dep_res {
        Ok(()) => (),
        Err(err) => println!("{}", err),
    }

    match with_res {
        Ok(()) => (),
        Err(err) => println!("{}", err),
    }

    let balance_alice = bank_account_alice.balance();
    let balance_ben = bank_account_ben.balance();

    println!(
        "{}'s account balance: {}\n{}'s account balance: {}\n",
        bank_account_alice.holder_name, balance_alice, bank_account_ben.holder_name, balance_ben
    );
}
