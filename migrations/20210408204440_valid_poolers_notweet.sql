-- Add migration script here
CREATE OR REPLACE VIEW valid_poolers_notweet AS
   select * from top_20_percent
        where exists
        (
        select 1 from valid_rune_notweet
        where rune_address=from_address
        );