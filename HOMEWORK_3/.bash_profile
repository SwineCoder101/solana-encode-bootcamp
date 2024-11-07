# Define environment variables
export SOLANA_WALLET_DIR=~/encode-academy/solana-wallet
export SOLANA_KEYPAIR_FILE=$SOLANA_WALLET_DIR/test-keypair.json
export SOLANA_DEVNET_URL=https://api.devnet.solana.com

# Alias to create a keypair
alias create_solana_keypair='mkdir -p $SOLANA_WALLET_DIR && solana-keygen new --outfile $SOLANA_KEYPAIR_FILE && export SOLANA_PUBKEY=$(solana-keygen pubkey $SOLANA_KEYPAIR_FILE) && echo "Public Key: $SOLANA_PUBKEY"'

# Alias to display the public key
alias display_solana_pubkey='solana-keygen pubkey $SOLANA_KEYPAIR_FILE'

# Alias to verify the address
alias verify_solana_address='solana-keygen verify $SOLANA_PUBKEY $SOLANA_KEYPAIR_FILE'

# Alias to connect to the dev network
alias connect_solana_devnet='solana config set --url $SOLANA_DEVNET_URL && solana config get'

# Alias to get some tokens from dev net
# Usage: airdrop_solana_tokens <RECIPIENT_ACCOUNT_ADDRESS>
alias airdrop_solana_tokens='solana airdrop 1 $1 --url $SOLANA_DEVNET_URL'

# Note: You will receive the transaction ID and can look for this on the dev net block explorer