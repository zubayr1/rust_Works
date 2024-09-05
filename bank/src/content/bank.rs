use super::account::Account;

#[derive(Debug)]
pub struct Bank {
    accounts: Vec<Account>,
    name: String,
}
impl Bank {
    pub fn new(name: String) -> Self {
        Bank {
            accounts: Vec::new(),
            name,
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}
