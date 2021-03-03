-- Add migration script here
DROP TABLE IF EXISTS txs;

CREATE TABLE txs (
    network VARCHAR,
    hash VARCHAR,
    height VARCHAR,
    memo VARCHAR,
    PRIMARY KEY (hash)
);
