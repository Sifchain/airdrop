-- Add migration script here
ALTER TABLE txs
    DROP COLUMN tweent_url,
    DROP COLUMN valid_memo,
    DROP COLUMN tweet_found,
    ADD COLUMN tweet_found varchar