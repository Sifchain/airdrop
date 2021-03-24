-- Add migration script here
-- Add migration script here

DROP TABLE IF EXISTS gaia_addresses;

CREATE TABLE gaia_addresses (
                                id SERIAL UNIQUE NOT NULL,
                                address varchar
)
