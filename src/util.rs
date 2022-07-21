use crate::*;

pub(crate) fn assert_at_least_one_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Required attached deposite of at least 1 yoctoNEAR"
    );
}

pub(crate) fn assert_one_yocto() {
    assert!(
        env::attached_deposit() == 1,
        "Required attached deposite  1 yoctoNEAR"
    );
}

pub(crate) fn refund_deposite(storage_used: u64) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposite = env::attached_deposit();
    assert!(
        attached_deposite >= required_cost,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost
    );

    let refund = attached_deposite - required_cost;
    if refund > 0 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}
