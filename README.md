# Dvote

This simple backend canister allows people to vote for their favorite programming languange.
It has the following methods:
1. ```get_votes()``` which returns a collection of votes.

2. ```vote()``` which allows a user to vote for a languange.

3. ```reset_votes()``` which resets the vote for each languange to 0.

I was inspired to build this by the Motoko tutorial by DFNITY which is  at https://internetcomputer.org/docs/current/tutorials/developer-journey/level-1/1.3-first-dapp

###### I'm  planning  to build a frontend canister for this soon.

##### Set up steps
make sure you have this installed
# 1. Rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# 2. Web Assembly
rustup target add wasm32-unknown-unknown

# 3. Candid Extractor
cargo install candid-extractor

# 4. DFX (DFINIT EXTENSION)
DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"

# 5. NodeJs


Run
#### Note: This code has only been tested on an Ubuntu Operating system
# 1. Start
Run ```cargo install candid-extractor``` to install it if you don't already have it

Run ```chmod +x did.sh``` in the root to grant execute permissions to the bash file responsible for building our canister.

Finally run ```npm run deploy``` 





