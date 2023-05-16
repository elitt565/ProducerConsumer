# ProducerConsumer
Rust version of Project4_377 Producer_Consumer

Link to video presentation: https://drive.google.com/file/d/1xDAPnz68gfSBUtqhylg38ezKufbQ6bOc/view

# Design
This project is based on the fourth project in the class CS377 in the Spring Semester 2023.

This program imitates a bank that's able to take in and process requests to change the balance of 10 accounts.  It's able to similuate any number of workers through threads and doesn't allow multiple accounts to be modified at once.  It records the number of successful and failed requests and the amount of balance each account has before and after the requests are fulfilled.  Only one bank instance is able to be used over the whole program.

# Documentation
## Building and running
In order to build this project, in the terminal type:
```
cargo build
```

In order to run this project, in the terminal type:
```
cargo run
```

## Valid File Inputs
This program accepts a .txt file that has requests in a specific format.  Each request must be in its own line.  A request should be formated as such:
```
FROM_ID TO_ID AMOUNT MODE
```

There are 3 modes that are listed below:
```
0 => Deposit
1 => Withdraw
2 => Transfer
```

In deposit and withdraw, the only ID that's used is the FROM_ID.  The other ID is ignored.

## Output Interpretation
The program will first print out the initial state of the bank (all accounts have balance 0 and the number of successes and failures are both 0).  An example using ledger.txt is given below:
```
ID# 0 | 0
ID# 1 | 0
ID# 2 | 0
ID# 3 | 0
ID# 4 | 0
ID# 5 | 0
ID# 6 | 0
ID# 7 | 0
ID# 8 | 0
ID# 9 | 0
Success: 0 Fails: 0
```

The program will then print out the order of ledgers that it processed (since can be multiple threads, the order can change from run to run).  An example using ledger.txt with four threads is below:
```
Worker 0 completed ledger 0: deposit 38 from account 8
Worker 2 completed ledger 2: deposit 9 from account 1
Worker 2 failed to complete ledger 5: transfer 233 from account 0 to account 7
Worker 1 failed to complete ledger 1: transfer 237 from account 8 to account 9
Worker 3 failed to complete ledger 4: withdraw 29 from account 2
Worker 0 failed to complete ledger 3: withdraw 95 from account 3
Worker 0 completed ledger 9: deposit 110 from account 4
Worker 1 failed to complete ledger 7: withdraw 54 from account 2
Worker 3 failed to complete ledger 8: transfer 280 from account 5 to account 8
Worker 2 completed ledger 6: deposit 334 from account 3
```

The program will finally print out the final state of the bank.  An example with ledger.txt with four threads is below:
```
ID# 0 | 0
ID# 1 | 9
ID# 2 | 0
ID# 3 | 334
ID# 4 | 110
ID# 5 | 0
ID# 6 | 0
ID# 7 | 0
ID# 8 | 38
ID# 9 | 0
Success: 4 Fails: 6
```

## Ledger.rs
### Ledger.rs- Ledger struct
The Ledger struct contains 5 u32 variables.  All of the variables are public.  This is in order to allow access to the fields in the method **worker**.  The fields aren't mutable, however, meaning after it's declared, there's no way to modify the values, making it safe to make those variables public.  The fields of the ledger struct are as follows:
```
from => is the FROM_ID of the ledger
to => is the TO_ID of the ledger (only used for transfers)
amount => is the AMOUNT of the ledger
mode => is the MODE of the ledger
ledger => the ID number of the ledger for us to keep track of
```

### Ledger.rs- global variables
The Ledger.rs file contains 3 global variables.  They are as follows:
```
ledger_lock => the mutex lock used in this file to ensure that no ledgers are read twice and that the program accurately ends when the ledger is empty
ledger => a VecDeque of Ledger structs (basically like a linked list)
lazy_bank_obj => an object of type Lazy<Bank>.  This was necessary because static variables in rust require declaration of the value immediately and no functions can be used to declare the value.  This forced me to use the external crate once_cell in order to be able to call the function new from Bank.rs to create the bank object.
```

### Ledger.rs- InitBank
This function takes in two variables:
```
num_workers => the number of threads this program is going to spawn
filename => the name of the file that is going to be read for the ledgers.
```

It prints out the accounts of the bank before anything is done to it, reads in the ledgers from filename, then runs the function **worker** in num_workers number of threads.  After the threads are all done processing, they're all joined together automatically with the function **scope** from the external crate crossbeam (needed since there needed to be threads spawned that borrowed a local variable on the stack and otherwise it wouldn't work).  The final state of the bank is then printed out.

### Ledger.rs- load_ledger
This function takes in one variable:
```
filename -> the name of the file that is going to be read for the ledgers.
```

It parses through the file and converts each line in the file to a Ledger struct and stores the struct inside of the global variable ledger.

### Ledger.rs- worker
This function takes in two variables:
```
workerID => the ID of the thread that is working in this function (IDs are from 0 to num_workers - 1)
bank_obj => the bank object
```

It goes through ledger from the beginning to the end, removing the ledger that is currently being viewed at.  The function has locks in order to ensure that no ledger is looked at twice and the method stops properly.

## Bank.rs
### Bank.rs- Bank struct
The Bank struct has only one u32 variable.  It is below:
```
num => the number of accounts in the bank
```

I choose to move all the other variables that were in the bank struct in the c++ version of the project to be global variables because mutexes and vectors cannot be copied (meaning it was excessively difficult to impossible to pass around references to the bank struct and modify them) and num_succ and num_fail made more sense to be global to me.

### Bank.rs- Account struct
The Account struct has only 3 variables.  It is below:
```
accountID => the ID of the account
balance => the total balance of the account
lock => a lock to ensure that an Account is modified by multiple threads at a time
```

The balance is public but in order to allow for modification of the account's balance.

### Bank.rs- global variables
The Ledger.rs file contains 3 global variables.  They are as follows:
```
bank_lock => the mutex lock used in this file to ensure that nothing is printed or recorded at the same time as another thing
accounts => a vector of accounts that is used to track the 10 accounts in the bank with their balances
num_succ => the number of successful ledgers done
num_fail => the number of failed ledgers
```

### Bank.rs- new
This method takes in one variable:
```
N => the number of accounts
```

It initializes the bank object to having N accounts and initializes the vector of accounts with N accounts with balance 0.  The bank object is then returned.

### Bank.rs- deposit
This method takes in four variables:
```
workerID => the ID of the worker (thread)
ledgerID => the ID of the ledger entry
accountID => the account ID to deposit to
amount => the amount deposited
```

This function deposits amount into accountID, records a success with the following statement, then returns 0 to signify success:
```
"Worker [workerID] completed ledger [ledgerID]: deposit [amount] into account [accountID]"
```

### Bank.rs- withdraw
This method takes in four variables:
```
workerID => the ID of the worker (thread)
ledgerID => the ID of the ledger entry
accountID => the account ID to withdraw from
amount => the amount withdrew
```

This function withdraws amount from accountID.  A success is recorded with the following statement:
```
"Worker [workerID] completed ledger [ledgerID]: withdraw [amount] from account [accountID]"
```

And a failure is recorded with the following statement:
```
"Worker [workerID] failed to complete ledger [ledgerID]: withdraw [amount] from account [accountID]"
```

If it is a success, the function returns 0.  If it is a failure, the function returns -1.

### Bank.rs- transfer
This method takes in five variables:
```
workerID => the ID of the worker (thread)
ledgerID => the ID of the ledger entry
srcID => the account to transfer money out
destID => the account to receive the money
amount => the amount to transfer
```

This function transfers amount from srcID to destID.  A success is recorded with the following statement:
This function withdraws amount from accountID.  A success is recorded with the following statement:
```
"Worker [workerID] completed ledger [ledgerID]: transfer [amount] from account [srcID] to account [destID]"
```

And a failure is recorded with the following statement:
```
"Worker [workerID] failed to complete ledger [ledgerID]: transfer [amount] from account [srcID] to account [destID]"
```

If it is a success, the function returns 0.  If it is a failure, the function returns -1.

### Bank.rs- print_account
This method prints out the current balance each account in the bank and the number of successful and failed ledgers in the following format:
```
ID# 0 | 0
ID# 1 | 0
ID# 2 | 0
ID# 3 | 0
ID# 4 | 0
ID# 5 | 0
ID# 6 | 0
ID# 7 | 0
ID# 8 | 0
ID# 9 | 0
Success: 0 Fails: 0
```

### Bank.rs- recordSucc
This method takes in one variable:
```
message => the string to print
```

This method increments the number of sucesses by one and prints the message.

### Bank.rs- recordFail
This method takes in one variable:
```
message => the string to print
```

This method increments the number of failures by one and prints the message.
