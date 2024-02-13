# Voix

This simple backend canister allows writers to signup and publish articles and also upvote articles from other writers. 
1. Users can signup using their internet identity.
2. users can publish articles
3. Users can fetch articles from other writers.
4. Users can fetch a specific article.
5. Users can fetch other user profiles.
6. Users can upvote an article only once.
7. The owner of the article can delete the article.
   


### Set up steps
make sure you have this installed
##### 1. Rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

##### 2. Web Assembly
rustup target add wasm32-unknown-unknown

##### 3. Candid Extractor
cargo install candid-extractor

##### 4. DFX (DFINIT EXTENSION)
DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"

##### 5. NodeJs


### Running the canister locally
 Note: This code has only been tested on an Ubuntu Operating system


First, create an empty canister for the canister code to be installed into. To create the canister, run the command:


```dfx canister create yipyap_backend```



Next, you need to compile your program into a WebAssembly module that can be deployed on ICP by building the canister. To build the canister, run the command:


```dfx build yipyap_backend```


Then, install the compiled code into your canister with the command:


```dfx canister install yipyap_backend```
To deploy the canister, start the dfx local execution environment with the command:


```dfx start --clean --background```
Then, you can deploy the canister with the command:

```dfx deploy yipyap_backend```



### Tests
To run tests, run ```npm run test```
