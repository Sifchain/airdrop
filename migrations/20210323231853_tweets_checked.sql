-- Add migration script here
ALTER TABLE txs
    ADD COLUMN tweets_checked integer;
