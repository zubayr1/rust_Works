#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}
impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }

    fn print_account(&mut self) {
        println!("{:?}", self);
    }

    fn change_account(&mut self) {
        self.balance = 10;
    }

    fn print_holder(&mut self) {
        println!("{:?}", self.holder);
    }

    fn make_and_print_account(&self, id: u32, holder: String) -> Account {
        let account = Account::new(id, holder);

        println!("{:?}", account);

        account
    }

    fn return_details(&self) -> String {
        self.id.to_string() + " " + &self.balance.to_string() + " " + &self.holder
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
    name: String,
}
impl Bank {
    fn new(name: String) -> Self {
        Bank {
            accounts: Vec::new(),
            name,
        }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

fn main() {
    let mut bank = Bank::new("ABC".to_string());

    let mut account1 = Account::new(123, "zake".to_string());
    let accountref = &mut account1;

    accountref.change_account();

    accountref.print_account();

    accountref.print_holder();

    let _ = accountref.make_and_print_account(11, "fuck".to_string());

    println!("{}", accountref.return_details());

    bank.add_account(account1);
}
