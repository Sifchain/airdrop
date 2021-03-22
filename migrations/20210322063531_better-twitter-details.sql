-- Add migration script here

ALTER TABLE txs
    ADD COLUMN tweet_id bigint
