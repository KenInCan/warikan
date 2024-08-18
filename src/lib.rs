use std::collections::HashMap;
use std::mem::swap;

#[derive(Clone, Debug, PartialEq)]
pub struct Balance {
    pub creditor: usize,
    pub debtor: usize,
    pub amount: f32,
}

impl Balance {
    pub fn reverse_relation(&mut self) -> &Balance {
        swap(&mut self.creditor, &mut self.debtor);
        self.amount = 0.0 - self.amount;
        self
    }
}

#[derive(Debug)]
pub struct Payment {
    pub payer_id: usize,
    pub amount: usize,
    pub owers: Vec<usize>,
}

impl Payment {
    pub fn to_balance(self) -> Vec<Balance> {
        let per_person_cost: f32 = self.amount as f32 / (self.owers.len() + 1) as f32;
        let mut test: Vec<Balance> = Vec::new();
        for ower in self.owers {
            let item = Balance {
                creditor: self.payer_id,
                debtor: ower,
                amount: per_person_cost,
            };
            test.push(item);
        }
        test
    }
}

#[derive(Debug)]
pub struct Payments {
    pub payments: Vec<Payment>,
}

impl Payments {
    pub fn aggregate(self) -> Vec<Balance> {
        let mut each_costs: Vec<Balance> = Vec::new();
        let mut final_costs: Vec<Balance> = Vec::new();

        for payment in self.payments {
            let item = payment.to_balance();
            each_costs = [each_costs, item].concat();
        }

        while let Some(item) = each_costs.pop() {
            let mut found = false;

            for element in &mut each_costs {
                if is_same_payer_ower_combo(&item, element) {
                    element.amount += item.amount;
                    found = true;
                    break;
                } else if is_reversed_payer_ower_combo(&item, element) {
                    element.amount -= item.amount;
                    if element.amount < 0.0 {
                        element.reverse_relation();
                    }

                    found = true;
                    break;
                }
            }
            if !found {
                if item.amount == 0.0 {
                    continue;
                }
                final_costs.push(item.clone());
            }
        }
        final_costs
    }
}

pub type Users = HashMap<usize, String>;

fn is_same_payer_ower_combo(a: &Balance, b: &Balance) -> bool {
    a.creditor == b.creditor && a.debtor == b.debtor
}

fn is_reversed_payer_ower_combo(a: &Balance, b: &Balance) -> bool {
    a.creditor == b.debtor && a.debtor == b.creditor
}
