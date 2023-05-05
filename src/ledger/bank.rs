use std::sync::Mutex;

static bank_lock: Mutex<u32> = Mutex::new(0);
static mut accounts: Vec<Account> = Vec::new();

pub struct Account {
    accountID: u32,
    pub balance: u32,
    lock: Mutex<u32>,
}

#[derive(Clone, Copy)]
pub struct Bank {
    num: u32,
    num_succ: u32,
    num_fail: u32,
}

impl Bank {
    /**
     * @brief Construct a new Bank:: Bank object.  Initalizes all variables
     * (including the private variables) and initializes all accounts
     *
     * @param N: number of accounts
     */
    pub fn new(N: u32) -> Bank {
        let mut b = Bank {
            num: N,
            num_succ: 0,
            num_fail: 0,
        };
        for i in 0..N {
            let acc = Account {
                accountID: i,
                balance: 0,
                lock: Mutex::new(0),
            };
            unsafe {
                accounts.push(acc);
            }
        }
        return b;
    }
    /**
     * @brief Destroy the Bank:: Bank object by freeing all memory and destroying
     * all locks
     */
    pub fn drop(&mut self) {
        //TODO: finish this method (drop/destroy)
        // pthread_mutex_destroy(&bank_lock);
        // for(int i = 0; i < num; i++) {
        //     pthread_mutex_destroy(&((accounts + i) -> lock));
        // }
        // free(accounts);
    }

    //Functions With Accounts

    /**
     * @brief Adds money to an account and logs the result in the format:
     * "Worker [workerID] completed ledger [ledgerID]: deposit [amount] into
     * account [accountID]"
     *
     * @param workerID: the ID of the worker (thread)
     * @param ledgerID: the ID of the ledger entry
     * @param accountID: the account ID to deposit
     * @param amount: the amount deposited
     * @return int (0 since no failure)
     */
    pub fn deposit(&mut self, workerID: u32, ledgerID: u32, accountID: usize, amount: u32) -> i32 {
        unsafe {
            let mut g = accounts.get(accountID).unwrap().lock.lock().unwrap();
            // TODO: Find a way to update balance or use method below:
            // let oldAcc = accounts.get(accountID).unwrap();
            // let newAcc = Account {
            //     accountID: oldAcc.accountID,
            //     balance: oldAcc.balance + amount,
            //     lock: Mutex::new(0),
            // };
            // accounts.push(newAcc);
            // accounts.swap_remove(accountID);
            self.recordSucc(format!("Worker {} completed ledger {}: deposit {} from account {}", workerID, ledgerID, amount, accountID));
            drop(g);
        }
        return 0;
    }
    /**
     * @brief Withdraws money from an account if possible.  If not, logs failure.
     * 
     * Sucesses logged in the following format:
     * "Worker [workerID] completed ledger [ledgerID]: withdraw [amount] from
     * account [accountID]"
     * 
     * Failures logged in the following format:
     * "Worker [workerID] failed to complete ledger [ledgerID]: withdraw [amount]
     * from account [accountID]"
     *
     * @param workerID the ID of the worker (thread)
     * @param ledgerID the ID of the ledger entry
     * @param accountID the account ID to withdraw
     * @param amount the amount withdrawn
     * @return int 0 on success -1 on failure
     */
    pub fn withdraw(&mut self, workerID: u32, ledgerID: u32, accountID: usize, amount: u32) -> i32 {
        let fail = format!("Worker {} failed to complete ledger {}: withdraw {} from account {}", workerID, ledgerID, amount, accountID);
        let success = format!("Worker {} completed ledger {}: withdraw {} from account {}", workerID, ledgerID, amount, accountID);
        unsafe {
            let mut g = accounts.get(accountID).unwrap().lock.lock().unwrap();
            if accounts.get(accountID).unwrap().balance < amount {
                self.recordFail(fail);
                drop(g);
                return -1;
            }
            // TODO: Find a way to update balance or use method below:
            // let oldAcc = accounts.get(accountID).unwrap();
            // let newAcc = Account {
            //     accountID: oldAcc.accountID,
            //     balance: oldAcc.balance - amount,
            //     lock: Mutex::new(0),
            // };
            // accounts.push(newAcc);
            // accounts.swap_remove(accountID);
            self.recordSucc(success);
            drop(g);
        }
        return 0;
    }
    /**
     * @brief Transfer from one account to another if possible.  If not, logs failure.
     * 
     * Sucesses logged in the following format:
     * "Worker [workerID] completed ledger [ledgerID]: transfer [amount] from account
     * [srcID] to account [destID]"
     * 
     * Failures logged in the following format:
     * "Worker [workerID] failed to complete ledger [ledgerID]: transfer [amount] from
     * account [srcID] to account [destID]"
     *
     * @param workerID the ID of the worker (thread)
     * @param ledgerID the ID of the ledger entry
     * @param srcID the account to transfer money out
     * @param destID the account to receive the money
     * @param amount the amount to transfer
     * @return int 0 on success -1 on error
     */
    pub fn transfer(&mut self, workerID: u32, ledgerID: u32, src_id: usize, dest_id: usize, amount: u32) -> i32 {
        let fail = format!("Worker {} failed to complete ledger {}: transfer {} from account {} to account {}", workerID, ledgerID, amount, src_id, dest_id);
        let success = format!("Worker {} completed ledger {}: transfer {} from account {} to account {}", workerID, ledgerID, amount, src_id, dest_id);
        if src_id == dest_id {
            unsafe {
                let mut g = accounts.get(src_id).unwrap().lock.lock().unwrap();
                self.recordFail(fail);
                drop(g);
                return -1;
            }
        }
        let srcSmaller = src_id < dest_id;
        let smaller = match srcSmaller{
            true => src_id,
            false => dest_id,
        };
        let larger = match srcSmaller{
            true => dest_id,
            false => src_id,
        };
        unsafe {
            let mut g1 = accounts.get(smaller).unwrap().lock.lock().unwrap();
            let mut g2 = accounts.get(larger).unwrap().lock.lock().unwrap();
            if accounts.get(src_id).unwrap().balance < amount {
                self.recordFail(fail);
                drop(g2);
                drop(g1);
                return -1;
            }
            // TODO: Find a way to update balance or use method below:
            // let oldSrcAcc = accounts.get(dest_id).unwrap();
            // let newSrcAcc = Account {
            //     accountID: oldSrcAcc.accountID,
            //     balance: oldSrcAcc.balance - amount,
            //     lock: Mutex::new(0),
            // };
            // accounts.push(newSrcAcc);
            // accounts.swap_remove(src_id);
            // let oldDestID = accounts.get(dest_id).unwrap();
            // let newDestID = Account {
            //     accountID: oldDestID.accountID,
            //     balance: oldDestID.balance - amount,
            //     lock: Mutex::new(0),
            // };
            // accounts.push(newDestID);
            // accounts.swap_remove(dest_id);
            // self.recordSucc(success);
            // drop(g2);
            // drop(g1);
        }
        return 0;
    }

    //Support Functions

    /**
     * @brief prints account information
     */
    pub fn print_account(self) {
        for i in 0..self.num {
            unsafe {
                let mut g = accounts.get(i as usize).unwrap().lock.lock().unwrap();
                println!("ID# {} | {}", accounts.get(i as usize).unwrap().accountID, accounts.get(i as usize).unwrap().balance);
                drop(g); 
            }
        }
        let mut g = bank_lock.lock().unwrap();
        println!("Success: {} Fails: {}", self.num_succ, self.num_fail);
        drop(g);
    }
    /**
     * @brief helper function to increment the bank variable "num_succ" and print log
     * message.
     *
     * @param message
     */
    fn recordSucc(&mut self, message: String) {
        let mut g = bank_lock.lock().unwrap();
        println!("{}", message);
        self.num_succ += 1;
        drop(g);
    }
    /**
     * @brief helper function to increment the bank variable "num_fail" and print log
     * message.
     *
     * @param message
     */
    fn recordFail(&mut self, message: String) {
        let mut g = bank_lock.lock().unwrap();
        println!("{}", message);
        self.num_fail += 1;
        drop(g);
    }
}