-- Add migration script here
DROP TABLE IF EXISTS gaia_delegators;

CREATE TABLE gaia_delegators (
                                address VARCHAR,
                                coins BIGINT,
                            PRIMARY KEY (address, coins)
)