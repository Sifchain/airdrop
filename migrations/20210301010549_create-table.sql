-- Add migration script here
DROP TABLE IF EXISTS txs;

CREATE TABLE txs (
                     id SERIAL UNIQUE NOT NULL,
                     network VARCHAR,
                     hash VARCHAR,
                     height VARCHAR,
                     memo VARCHAR,
                     valid_memo bool,
                     twitter_handle VARCHAR,
                     sif_address VARCHAR,
                     PRIMARY KEY (network, hash)
);

DROP TABLE IF EXISTS rune_stakers;

CREATE TABLE rune_stakers (
    id SERIAL UNIQUE NOT NULL,
    asset varchar,
    rune_address varchar,
    asset_address varchar,
    last_stake bigint,
    last_unstake bigint
)