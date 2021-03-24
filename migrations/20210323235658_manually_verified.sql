-- Add migration script here

ALTER TABLE txs
    ADD COLUMN manually_verified bool;

