-- Add migration script here
CREATE OR REPLACE VIEW top_20_percent AS
    SELECT * FROM rune_stakers ORDER BY stake_total_rune DESC LIMIT 903;
