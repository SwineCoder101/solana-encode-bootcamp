# Exercise 2

## Solana EcoSystem

### How many validators are there currently ?
As of 6th Nov there are 1,424 validators as shown in https://solanabeach.io/validators

### What is special about this block? https://explorer.solana.com/block/0/programs
This block is regarded as the genesis block for solana, this can be depicted looking at the epoch and slot which are both 0.



### What is special about this address? https://explorer.solana.com/address/1nc1nerator11111111111111111111111111111111
This is the address of the Icinerator program which is used to burn sol/tokens (remove it from the circulated supply)


### What is this transaction doing ? https://explorer.solana.com/tx/45pGoC4Rr3fJ1TKrsiRkhHRbdUeX7633XAGVec6XzVdpRbzQgHhe6ZC6Uq164MPWtiqMg7wCkC6Wy3jy2BqsDEKf
It appears this transaction is transferring 11,365,066.99 SOL to the Incinerator program,, which means it is burning the supply of sol
The siginer is coming from two accounts which are owned by the system program
payer & signer: 3o6xgkJ9sTmDeQWyfj3sxwon18fXJB9PV5LDc8sfgR4a
signer: 6yKHERk8rsbmJxvMpPuwPs1ct3hRiP7xaJF2tvnGU6nK


### What is the largest balance you can find in an account ?
Solana uses a 64-bit unsigned integer (uint64) to store token balances in SPL token accounts. Therefore, the maximum possible balance in a Solana account is (2 ^ 64) − 1.

### What advantages will the end user see when using Solana compared to other blockchains ?
Low Gas Fees: Solana's architecture and consensus mechanism (Proof of History combined with Proof of Stake) allow for transactions to be processed quickly and efficiently, typically resulting in lower transaction fees compared to some other blockchains.

High Throughput: Solana is designed to handle a high volume of transactions per second (TPS), currently capable of processing thousands of transactions per second. This high throughput means users experience faster confirmation times and overall smoother transaction processing.

Program Upgradeability: Solana's architecture supports program upgradeability without requiring data migration, which can provide a seamless experience for users and developers when updates or improvements are made to smart contracts or applications running on the blockchain.

Efficiency and Scalability: Solana's consensus mechanism and architecture are designed to be highly efficient and scalable. The Proof of History (PoH) mechanism helps order transactions efficiently, and the Proof of Stake (PoS) consensus further enhances security and scalability while maintaining energy efficiency.

Energy Efficiency: Solana is known for its relative energy efficiency compared to Proof of Work (PoW) based blockchains like Bitcoin, primarily due to its use of Proof of Stake (PoS) combined with Proof of History (PoH) consensus mechanisms.