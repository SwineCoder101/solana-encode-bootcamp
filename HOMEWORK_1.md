## Exercise 1

### Decentralized Version of Monopoly

A decentralized version of monopoly we require a series of programs/accounts (or smart contracts) to store the state of the game
and allow for secure peer to peer interactions for players within instructions.

**Essential Pieces of Functionality:**
- **SPL Token:** Used for in-game currency.
- **NFTs:** Proof of ownership for properties and houses.
- **User Registry:** Tracks players and manages access control.
- **Token Escrow:** Facilitates secure peer-to-peer transactions.
- **Treasury Program:** Manages operations similar to a bank.
- **Oracle VRF:** Provides reliable randomness for game elements.
- **Auction Program:** Enables bidding and ensures fairness.

**Potential Cheating Methods:**
- Exploiting vulnerabilities in web2 systems.
- Manipulating game accounts and transactions.
- Lying about ownership or transactions.
- Manipulating game randomness.

People can easily cheat in a web2 system if they bypass the security elements of the centralized server (ie jwt/auth tokens). They can inject requests/responses and exploit vulnerabilities within the executable code. Game accounts can be exploited if they are using so form of centralized auth provider and trades can easily be manipulated in the game. Players can also potentially lie on what they own amongst peers, although the game is realtime it is still possible to forge payments and ownable assets. Games require randomness, anyone can try to manipulate this.

**Preventing Cheating:**
- Implementing Token Escrow for secure transactions.
- Using NFTs and tokens for verifiable ownership.
- Utilizing Oracle VRF for fair game randomness.
- Maintaining a transparent User Registry.

A token Escrow can be created to handle deals. This will help prevent others from cheating as both players are required to deposit items (cash/property) into
a game vault and both users need to attest/agree on the potential deal.
NFTs and tokens will be used to provide proof of ownership, so no one can forge this information as its stored on chain and visible for all to see
An Oracle verifiable random function must be used to prevent others from manipulating randomness
The state balance will be stored in a user registry for all to see.

### Central Bank Digital Currencies (CBDCs)

**Are CBDCs a Move Towards Decentralization?**
CBDCs are digital currencies issued by central banks, which are inherently centralized entities. While they introduce digital forms of fiat currency, they do not promote decentralization like cryptocurrencies do.

This is because they are issued by a centralized authority and not by a decentralized governanent network which defeats the purpose and ethos of blockchain.
When we save money into digital accounts, we have no assurance the asset is not being spent or moved across, we just see an account balance given by the bank. CBDCs are just a digital form of money programmed by these organisations which we the public trust. Blockchain gives us the option, transparency and freedom to control what we own without any intervention from other authorities. It is a technical solution to a wide problem.

**Impact on Cryptocurrency Adoption:**
- **Help:** Familiarizes users with digital assets.
- **Hinder:** Competes with existing cryptocurrencies.
- **Neutral:** Depends on regulatory stance and integration with existing financial systems.




