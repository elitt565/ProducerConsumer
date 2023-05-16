mod bank;

pub use self::bank::Bank;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str;
use std::collections::VecDeque;
use std::thread;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static ledger_lock: Mutex<u32> = Mutex::new(0);
static mut ledger: VecDeque<Ledger> = VecDeque::new();
static mut lazy_bank_obj: Lazy<Bank> = Lazy::new(|| {
    let mut b = Bank::new(10);
    b
});

struct Ledger {
    pub from: u32,
    pub to: u32,
    pub amount: u32,
    pub mode: u32,
    pub ledger: u32,
}

/**
 * @brief creates a new bank object with 10 accounts, initializes the ledger,
 * and sets up workers
 *
 * @param num_workers: the number of workers needed to be made and ran
 * @param filename: name of the file where all ledgers are kept
 */
pub fn InitBank(num_workers: u32, filename: String) {
    let mut bank_obj;
    unsafe {
        &*lazy_bank_obj;
        bank_obj = *Lazy::get(&lazy_bank_obj).unwrap();
        bank_obj.print_account();
    }
    load_ledger(filename);
    let n = num_workers;
    thread::spawn(move || {
        for i in 1..=n {
            worker(i);
        }
    }).join().unwrap();
    bank_obj.print_account();
}

/**
 * @brief The function load_ledger() takes a file name as an input and parses
 * each line of the file into struct ledger objects and appends it to the list
 * list<struct Ledger> ledger
 *
 * @param filename: name of the file where all ledgers are kept
 */
pub fn load_ledger(filename: String) {
    let f = File::open(format!("./{}", filename)).unwrap();
    let mut reader = BufReader::new(f);
    let mut ledger_id = 0;
    let mut line = String::new();
    let mut len = reader.read_line(&mut line).unwrap();
    while len != 0 {
        while !line.chars().last().unwrap().is_numeric() {
            line.pop();
        }
        let v: Vec<&str> = line.split(' ').collect();
        let l = Ledger {
            from: v[0].parse().unwrap(),
            to: v[1].parse().unwrap(),
            amount: v[2].parse().unwrap(),
            mode: v[3].parse().unwrap(),
            ledger: ledger_id,
        };
        ledger_id += 1;
        unsafe {
            ledger.push_back(l);
        }
        line = String::new();
        len = reader.read_line(&mut line).unwrap();
    }
}

/**
 * @brief Remove items from the list and execute the instruction and runs the
 * function indicated by "mode" in struct Ledger.
 *
 * @param workerID: ID of worker thread that is running at the moment
 */
fn worker(workerID: u32) {
    let bank_obj;
    unsafe {
        &*lazy_bank_obj;
        bank_obj = Lazy::get_mut(&mut lazy_bank_obj).unwrap();
    }
    let mut g = ledger_lock.lock().unwrap();
    unsafe {
        while !ledger.is_empty() {
            let l = ledger.pop_front().unwrap();
            drop(g);
            let mode = l.mode;
            if mode == 0 {
                bank_obj.deposit(workerID, l.ledger, l.from as usize, l.amount);
            }
            else if mode == 1 {
                bank_obj.withdraw(workerID, l.ledger, l.from as usize, l.amount);
            }
            else {
                bank_obj.transfer(workerID, l.ledger, l.from as usize, l.to as usize, l.amount);
            }
            g = ledger_lock.lock().unwrap();
        }
    }
    drop(g);
}