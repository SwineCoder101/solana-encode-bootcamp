## Exercise 3

### Create a keypair
mkdir ~/my-solana-wallet solana-keygen new --outfile ~/my-solana-wallet/my-keypair.json

display the result with solana-keygen pubkey ~/my-solana-wallet/my-keypair.json

verify your address solana-keygen verify <PUBKEY> ~/my-solana-wallet/my-keypair.json

### Connect to the dev network
solana config set --url https://api.devnet.solana.com You can check the configuration with solana config get

Get some tokens from dev net solana airdrop 1 <RECIPIENT_ACCOUNT_ADDRESS> --url https://api.devnet.solana.com

you will receive the transaction ID, and can look for this on the dev net block explorer



A bash profile has been created for these commands, probably quite trivial to use.
Below are the how to instructinos:


#### init the profile in the bash terminal
source ~/.bash_profile

#### Create a keypair and store the public key
create_solana_keypair

#### Display the public key
display_solana_pubkey

#### Verify the address
verify_solana_address

#### Connect to the dev network
connect_solana_devnet

#### Get some tokens from dev net (replace <RECIPIENT_ACCOUNT_ADDRESS> with the actual address)
airdrop_solana_tokens <RECIPIENT_ACCOUNT_ADDRESS>