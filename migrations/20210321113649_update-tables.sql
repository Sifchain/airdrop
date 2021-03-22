-- Add migration script here

ALTER TABLE txs
ADD COLUMN tweet_found bool,
ADD COLUMN tweent_url varchar,
ADD COLUMN twitter_error varchar;
