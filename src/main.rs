mod ledger;
use crate::ledger::InitBank;

fn main() {
    //TODO: implement tests and check implementation in all methods
    InitBank(1, format!("ledger.txt"));
}