## Airdrop


# Requirements

1. rustlang follow: https://www.rust-lang.org/learn/get-started
2. Install sqlx: `cargo install sqlx-cli`    
3. docker / docker-compose
4. direnv (to use the .envrc) 

# Setup

0. Setup .envrc (See .envrc-example)
1. docker-compose up
2. Setup db: 
    * `sqlx database create`
    * `sqlx migrate run`
3. cargo build --release 

# Run RUNE Tx Import script

1. `./target/release/import-rune-txs`

Note. total_count should now equal rows in the airdrop.txs db 

# Run ATOM Tx Import script

1. `./target/release/import-atom-txs`

Note; 
   * This API has been somewhat problematic/slow/drops out. Means that this script might need to be run a few time until all records are imported.


#  Run Twitter processing from the cleaned memo field in the txs table (After txs imports) 

1 `./target/release/process-twitter-from-txs-records`

Note; 
   * This will check for the required tweet with a media attachment. It will also save any error message given back from twitter if something fails. 
Its also
   * Due to the twitter rate limiting you might need to run this script a couple of time to make sure all records have been processed.
   * This script takes quite sometime to run. 

# Run the import Rune pool extract script 

1. `./target/release/import-rune-pool-extract`

Note; This will import data into the `rune_stakers` table

# Run Imported for extract data sets (gaia and token sale codes) 

1. `make import-data-extracts`

# Run Import token sale submissions

1. `./target/release/import-token-sale-data < ./extracts/community_token_giveaway_submissions.csv`

Note; 
   * This will clean and import the data and also run the twitter checks at the same time. 
   * This can take a while to finish and might be stopped by the twitter rate limiter and therefore should be run a couple of times to make sure all data is imported
