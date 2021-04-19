-- Add migration script here
CREATE OR REPLACE VIEW top_20_delegators AS
    SELECT * FROM gaia_delegators ORDER BY coins DESC LIMIT 13094;
