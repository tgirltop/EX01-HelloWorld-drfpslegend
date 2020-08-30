
# Swap-rs
A Rust programm to make your own transactions, swap/trade from your terminal ! (Mini-project to learn Rust)


This programm allow to send a transaction, using web3 crate (i will do it also using ethers-rs because web3-rs just have basic functions), and catching asset price (ETH in the example).
You can use Testnet (Goerli, Sepolia ...etc) and Mainnet too.

The second part of "Swap-rs" allow you to directly swap an asset to an other, using uniswap, and to configure, maximum gas allowed.

Note that you can store sensitivs informations like adress and private key by using serde_JSON crate.
Be sure to add all necessary dependencies in your .toml.
For the RPC, i suggest Alchemy or Infura.