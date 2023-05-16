# ProducerConsumer
Rust version of Project4_377 Producer_Consumer

## Design
This project is based on the fourth project in the class CS377 in the Spring Semester 2023.

This program imitates a bank that's able to take in and process requests to change the balance of 10 accounts.  It's able to similuate any number of workers through threads and doesn't allow multiple accounts to be modified at once.  It records the number of successful and failed requests and the amount of balance each account has before and after the requests are fulfilled.  Only one bank instance is able to be used over the whole program.

## Documentation
### Building and running
In order to build this project, in the terminal type:
```
cargo build
```

In orderto run this project, in the terminal type:
```
cargo run
```
