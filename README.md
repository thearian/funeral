# Funeral: Death Will Encryption Service
The pupose is to make an encryption from skrach only for encrypting **the data you want to be only read after your own death**

## Usage
Note that this repository is still in development stage.\
But you can sneak some features and test them out using either `Cargo`:
```bash
cargo run <WILL_FILEPATH> <SELECTED_PASSWORD> <LOCK_STATUS>
```
which selected password is still in develop and the code doesn't really care what you give as password\
and LOCK_STATUS is either U for Unlock of L for Lock\
\
or using the binaries at the `bin` folder as compiled code of each version. Usage is the same.
```bash
& ./bin/funeral.exe <WILL_FILEPATH> <SELECTED_PASSWORD> <LOCK_STATUS>
```

## Development
The repository is a `Cargo` project written with `Rust` and `Python` as helpers.\
(python helpers will be removed as soon as their rust equivalent is replaced)
### FCM: Funeral Command Manager
There is a development tools to help you out start working with this repo and it is `FCM` short for **Funeral Command Manager**.
#### FCM run
It is very much the same as `cargo run`. It needs the arguments and runs the will encryption.
```bash
& ./bin/fcm.exe run <WILL_FILEPATH> <SELECTED_PASSWORD> <LOCK_STATUS>
```
#### FCM dev
This one is made to cut the arguments and works as a quick run. Please make an `a.txt` file in the root and fill it with some text before running.
```bash
& ./bin/fcm.exe dev
```