use warikan::*;

fn main() {
    let mut users = Users::new();

    users.insert(0, "Alice".to_string());
    users.insert(1, "Bob".to_string());
    users.insert(2, "John".to_string());
    users.insert(3, "Ken".to_string());

    // create payment 1
    let payer1: usize = 0;
    let owers1: Vec<usize> = vec![1, 2];
    let amount1 = 300;

    let payment1 = Payment {
        payer_id: payer1,
        owers: owers1,
        amount: amount1,
    };

    // create payment 2
    let payer2: usize = 0;
    let owers2: Vec<usize> = vec![1, 2];
    let amount2 = 900;

    let payment2 = Payment {
        payer_id: payer2,
        owers: owers2,
        amount: amount2,
    };

    // create payment 3
    let payer3: usize = 1;
    let owers3: Vec<usize> = vec![0, 2];
    let amount3 = 1500;

    let payment3 = Payment {
        payer_id: payer3,
        owers: owers3,
        amount: amount3,
    };

    // create payment 4
    let payer4: usize = 0;
    let owers4: Vec<usize> = vec![3];
    let amount4 = 1000;

    let payment4 = Payment {
        payer_id: payer4,
        owers: owers4,
        amount: amount4,
    };

    // create payment 5
    let payer5: usize = 0;
    let owers5: Vec<usize> = vec![3];
    let amount5 = 1;

    let payment5 = Payment {
        payer_id: payer5,
        owers: owers5,
        amount: amount5,
    };

    let payments = Payments {
        payments: vec![payment1, payment2, payment3, payment4, payment5],
    };

    for payment in &payments.payments {
        let payer_name = users.get(&payment.payer_id).unwrap();
        print!("{:<8} payed: ${:<8} for: ", payer_name, payment.amount);
        for ower in &payment.owers {
            let ower_name = users.get(ower).unwrap();
            print!("{}, ", ower_name);
        }
        println!("");
    }
    println!("----------------------");

    let output = payments.aggregate();

    for entry in output {
        let debtor_name = users.get(&entry.debtor).unwrap();
        let creditor_name = users.get(&entry.creditor).unwrap();
        println!("{:<8} owes {:<8} ${:.0}", debtor_name, creditor_name, entry.amount);
    }
}
