// Define the Account trait
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

// Define the BankAccount struct
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

// Implement the Account trait for BankAccount
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited {} into account {}. New balance: {}", amount, self.account_number, self.balance);
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
            println!("Withdrew {} from account {}. New balance: {}", amount, self.account_number, self.balance);
        } else {
            println!("Insufficient funds in account {}. Current balance: {}", self.account_number, self.balance);
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Create two BankAccount instances
    let mut account1 = BankAccount {
        account_number: 1001,
        holder_name: String::from("Alice"),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: 1002,
        holder_name: String::from("Bob"),
        balance: 500.0,
    };

    // Deposit into account1
    account1.deposit(200.0);

    // Withdraw from account2
    account2.withdraw(100.0);

    // Display balance of both accounts
    println!("Account {} ({}): Current balance: {}", account1.account_number, account1.holder_name, account1.balance());
    println!("Account {} ({}): Current balance: {}", account2.account_number, account2.holder_name, account2.balance());
}
