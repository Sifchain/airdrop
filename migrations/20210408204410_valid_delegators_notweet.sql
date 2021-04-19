-- Add migration script here
CREATE OR REPLACE VIEW valid_delegators_notweet AS
   select * from top_20_delegators
        where exists
        (
        select 1 from valid_atom_notweet
        where address=from_address
        );