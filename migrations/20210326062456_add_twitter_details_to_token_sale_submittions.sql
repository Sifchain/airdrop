-- Add migration script here

ALTER table token_sale_submissions
    ADD COLUMN twitter_error varchar,
    ADD COLUMN tweet_id bigint,
    ADD COLUMN tweet_found bool,
    ADD COLUMN media_found bool,
    ADD COLUMN tweets_checked integer
