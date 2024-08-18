use warikan::*;

#[test]
fn should_convert_to_balance() {
    let mut users = Users::new();

    users.insert(0, "Alice".to_string());
    users.insert(1, "Bob".to_string());
    users.insert(2, "John".to_string());

    // create payment 1
    let payer1: usize = 0;
    let owers1: Vec<usize> = vec![1, 2];
    let amount1 = 900;

    let payment1 = Payment {
        payer_id: payer1,
        owers: owers1,
        amount: amount1,
    };

    // create test
    let test1 = Balance {
        creditor: 0,
        debtor: 1,
        amount: 300.0,
    };

    let test2 = Balance {
        creditor: 0,
        debtor: 2,
        amount: 300.0,
    };

    let test_case: Vec<Balance> = vec![test1, test2];

    assert_eq!(payment1.to_balance(), test_case);
}

#[test]
fn should_convert_to_balance_in_floats() {
    let mut users = Users::new();

    users.insert(0, "Alice".to_string());
    users.insert(1, "Bob".to_string());

    // create payment 1
    let payer1: usize = 0;
    let owers1: Vec<usize> = vec![1];
    let amount1 = 1;

    let payment1 = Payment {
        payer_id: payer1,
        owers: owers1,
        amount: amount1,
    };

    // create test
    let test1 = Balance {
        creditor: 0,
        debtor: 1,
        amount: 0.5,
    };

    let test_case: Vec<Balance> = vec![test1];

    assert_eq!(payment1.to_balance(), test_case);
}

#[test]
fn should_reverse_balance() {
    let mut test1 = Balance {
        creditor: 0,
        debtor: 1,
        amount: -300.0,
    };

    let test2 = Balance {
        creditor: 1,
        debtor: 0,
        amount: 300.0,
    };

    assert_eq!(*test1.reverse_relation(), test2);
}

#[test]
fn should_hande_combination_of_operations() {
    let mut users = Users::new();

    users.insert(0, "Alice".to_string());
    users.insert(1, "Bob".to_string());
    users.insert(2, "John".to_string());

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

    let payments = Payments {
        payments: vec![payment1, payment2, payment3],
    };

    let test1 = Balance {
        creditor: 1,
        debtor: 0,
        amount: 100.0,
    };

    let test2 = Balance {
        creditor: 0,
        debtor: 2,
        amount: 400.0,
    };

    let test3 = Balance {
        creditor: 1,
        debtor: 2,
        amount: 500.0,
    };

    let test_case: Vec<Balance> = vec![test3, test2, test1];

    assert_eq!(test_case, payments.aggregate());
}

#[test]
fn should_add_up_the_amount() {
    let mut users = Users::new();

    users.insert(0, "Alice".to_string());
    users.insert(1, "Bob".to_string());

    // create payment 1
    let payer1: usize = 0;
    let owers1: Vec<usize> = vec![1];
    let amount1 = 200;

    let payment1 = Payment {
        payer_id: payer1,
        owers: owers1,
        amount: amount1,
    };

    // create payment 2
    let payer2: usize = 0;
    let owers2: Vec<usize> = vec![1];
    let amount2 = 800;

    let payment2 = Payment {
        payer_id: payer2,
        owers: owers2,
        amount: amount2,
    };

    let payments = Payments {
        payments: vec![payment1, payment2],
    };

    let test1 = Balance {
        creditor: 0,
        debtor: 1,
        amount: 500.0,
    };

    let test_case: Vec<Balance> = vec![test1];

    assert_eq!(test_case, payments.aggregate());
}

#[test]
fn should_cancel_out() {
    let mut users = Users::new();

    users.insert(0, "Alice".to_string());
    users.insert(1, "Bob".to_string());

    // create payment 1
    let payer1: usize = 0;
    let owers1: Vec<usize> = vec![1];
    let amount1 = 200;

    let payment1 = Payment {
        payer_id: payer1,
        owers: owers1,
        amount: amount1,
    };

    // create payment 2
    let payer2: usize = 1;
    let owers2: Vec<usize> = vec![0];
    let amount2 = 200;

    let payment2 = Payment {
        payer_id: payer2,
        owers: owers2,
        amount: amount2,
    };

    let payments = Payments {
        payments: vec![payment1, payment2],
    };

    let test_case: Vec<Balance> = Vec::new();

    assert_eq!(test_case, payments.aggregate());
}
