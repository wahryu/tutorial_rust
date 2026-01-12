enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with CreditCard {} amoun {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with Bank Transfer {} {} amoun {}", bank, number, amount);
            }
            Payment::EWallet(wallet, number) => {
                println!("Paying with EWallet {} {} amoun {}", wallet, number, amount);
            }
        }
    }
}

fn test_payment() {
    let _payment1: Payment = Payment::CreditCard(String::from("43535"));
    _payment1.pay(20000);

    let _payment2: Payment = Payment::BankTransfer(String::from("BCA"), String::from("435fhhg35"));
    let _payment3: Payment = Payment::EWallet(String::from("GoPay"), String::from("AWW43535"));
}




// Patern Matching Enum
enum Level {
    Reguler,
    Premium,
    Platinum,
}

fn test_matching() {
    let level: Level = Level::Reguler;

    match level {
        Level::Reguler => {
            println!("Reguler")
        }
        Level::Premium => {
            println!("Premium")
        }
        Level::Platinum => {
            println!("Platinum")
        }
    }
}