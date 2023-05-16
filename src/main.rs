mod ledger;
use crate::ledger::InitBank;

fn main() {
    //TODO: implement tests and check implementation in all methods
    InitBank(4, format!("ledger.txt"));
}