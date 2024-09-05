mod content;

use content::{account::Account, bank::Bank};

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
