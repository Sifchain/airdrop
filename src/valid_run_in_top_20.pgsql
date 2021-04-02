-- CREATE OR REPLACE VIEW valid_poolers AS
   select * from top_20_percent
        where exists
        (
        select 1 from valid_rune
        where rune_address=from_address
        );