-- Add migration script here
CREATE OR REPLACE VIEW valid_delegators AS
   select * from top_20_delegators
        where exists
        (
        select 1 from valid_atom
        where address=from_address
        );