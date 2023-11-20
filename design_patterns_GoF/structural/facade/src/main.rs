#![allow(dead_code)]
#![allow(unused_imports)]

/*
`Pattern Facade` скрывает сложности системы и предоставляет простой интерфейс к сложной системе классов, библиотеке или фреймворку.

Оберните сложную подсистему более простым интерфейсом.
Один класс Facade, представляющий всю подсистему.

Объект Facade должен быть довольно простым защитником или посредником. Он не должен становиться всезнающим оракулом или объектом «бога».

**Проблема**
Вашему коду приходится работать с большим количеством объектов некой сложной библиотеки или фреймворка.
Вы должны самостоятельно инициализировать эти объекты, следить за правильным порядком зависимостей и так далее.
В результате бизнес-логика ваших классов тесно переплетается с деталями реализации сторонних классов.
Такой код довольно сложно понимать и поддерживать.

`Pattern Facade` может иметь урезанный интерфейс, не имеющий 100% функциональности, которой можно достичь, используя сложную подсистему напрямую.
Но он предоставляет именно те фичи, которые нужны клиенту, и скрывает все остальные.

`Pattern Facade` полезен, если вы используете какую-то сложную библиотеку со множеством подвижных частей, но вам нужна только часть её возможностей.

Аналогия
Когда вы звоните в магазин и делаете заказ по телефону, сотрудник службы поддержки является вашим фасадом ко всем службам и отделам магазина.
Он предоставляет вам упрощённый интерфейс к системе создания заказа, платёжной системе и отделу доставки.

**Эмпирические правила**

- `Pattern Facade` задаёт новый интерфейс, тогда как `Pattern Adapter` повторно использует старый.
`Pattern Adapter` оборачивает только один класс, а `Pattern Facade` оборачивает целую подсистему.
Кроме того, `Pattern Adapter` позволяет двум существующим интерфейсам работать сообща, вместо того, чтобы задать полностью новый.

- Abstract Factory может быть использована вместо `Pattern Facade` для того, чтобы скрыть платформо-зависимые классы.

- Объекты Facade часто являются синглтонами, поскольку требуется только один объект Facade.

*/

use wallet_facade::*;
mod wallet_facade {
    use crate::{
        account::Account, ledger::Ledger, notification::Notification, security_code::SecurityCode,
        wallet::Wallet,
    };

    /// Facade hides a complex logic behind the API.
    pub struct WalletFacade {
        account: Account,
        wallet: Wallet,
        code: SecurityCode,
        notification: Notification,
        ledger: Ledger,
    }

    impl WalletFacade {
        pub fn new(account_id: String, code: u32) -> Self {
            println!("Starting create account");

            let this = Self {
                account: Account::new(account_id),
                wallet: Wallet::new(),
                code: SecurityCode::new(code),
                notification: Notification,
                ledger: Ledger,
            };

            println!("Account created");
            this
        }

        pub fn add_money_to_wallet(
            &mut self,
            account_id: &String,
            security_code: u32,
            amount: u32,
        ) -> Result<(), String> {
            println!("Starting add money to wallet");
            self.account.check(account_id)?;
            self.code.check(security_code)?;
            self.wallet.credit_balance(amount);
            self.notification.send_wallet_credit_notification();
            self.ledger.make_entry(account_id, "credit".into(), amount);
            Ok(())
        }

        pub fn deduct_money_from_wallet(
            &mut self,
            account_id: &String,
            security_code: u32,
            amount: u32,
        ) -> Result<(), String> {
            println!("Starting debit money from wallet");
            self.account.check(account_id)?;
            self.code.check(security_code)?;
            self.wallet.debit_balance(amount);
            self.notification.send_wallet_debit_notification();
            self.ledger.make_entry(account_id, "debit".into(), amount);
            Ok(())
        }
    }
}

use wallet::*;
mod wallet {
    pub struct Wallet {
        balance: u32,
    }

    impl Wallet {
        pub fn new() -> Self {
            Self { balance: 0 }
        }

        pub fn credit_balance(&mut self, amount: u32) {
            self.balance += amount;
        }

        pub fn debit_balance(&mut self, amount: u32) {
            self.balance
                .checked_sub(amount)
                .expect("Balance is not sufficient");
        }
    }
}

use account::*;
mod account {
    pub struct Account {
        name: String,
    }

    impl Account {
        pub fn new(name: String) -> Self {
            Self { name }
        }

        pub fn check(&self, name: &String) -> Result<(), String> {
            if &self.name != name {
                return Err("Account name is incorrect".into());
            }

            println!("Account verified");
            Ok(())
        }
    }
}

use ledger::*;
mod ledger {
    pub struct Ledger;

    impl Ledger {
        pub fn make_entry(&mut self, account_id: &String, txn_type: String, amount: u32) {
            println!(
                "Make ledger entry for accountId {} with transaction type {} for amount {}",
                account_id, txn_type, amount
            );
        }
    }
}

use notification::*;
mod notification {
    pub struct Notification;

    impl Notification {
        pub fn send_wallet_credit_notification(&self) {
            println!("Sending wallet credit notification");
        }

        pub fn send_wallet_debit_notification(&self) {
            println!("Sending wallet debit notification");
        }
    }
}

use security_code::*;
mod security_code {
    pub struct SecurityCode {
        code: u32,
    }

    impl SecurityCode {
        pub fn new(code: u32) -> Self {
            Self { code }
        }

        pub fn check(&self, code: u32) -> Result<(), String> {
            if self.code != code {
                return Err("Security code is incorrect".into());
            }

            println!("Security code verified");
            Ok(())
        }
    }
}

// cargo run --bin facade
fn main() -> Result<(), String> {
    let mut wallet = WalletFacade::new("abc".into(), 1234);
    println!();

    // Wallet Facade interacts with the account, code, wallet, notification and
    // ledger behind the scenes.
    wallet.add_money_to_wallet(&"abc".into(), 1234, 10)?;
    println!();

    wallet.deduct_money_from_wallet(&"abc".into(), 1234, 5)
}
