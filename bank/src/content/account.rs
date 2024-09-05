#[derive(Debug)]
pub struct Account {
    id: u32,
    balance: i32,
    holder: String,
}
impl Account {
    pub fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }

    pub fn print_account(&mut self) {
        println!("{:?}", self);
    }

    pub fn change_account(&mut self) {
        self.balance = 10;
    }

    pub fn print_holder(&mut self) {
        println!("{:?}", self.holder);
    }

    pub fn make_and_print_account(&self, id: u32, holder: String) -> Account {
        let account = Account::new(id, holder);

        println!("{:?}", account);

        account
    }

    pub fn return_details(&self) -> String {
        self.id.to_string() + " " + &self.balance.to_string() + " " + &self.holder
    }
}
