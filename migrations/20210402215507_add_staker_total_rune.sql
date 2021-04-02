-- Add migration script here
ALTER TABLE rune_stakers
    ADD COLUMN stake_total_rune bigint;