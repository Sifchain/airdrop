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
