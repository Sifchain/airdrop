-- Add migration script here
ALTER TABLE txs
    DROP COLUMN tweet_found,
    ADD COLUMN tweet_found bool
