-- Add migration script here
-- Add migration script here
CREATE OR REPLACE VIEW valid_rune_notweet AS
    SELECT id, network, hash, height, 
memo, twitter_handle, sif_address, 
twitter_error, tweet_id, tweet_found, 
media_found, tweets_checked, manually_verified, 
from_address 
FROM (
        SELECT *, rank() over (partition by sif_address order by id asc) r
        FROM (
            SELECT *, rank() over (partition by from_address order by height desc) rr
            FROM txs
            WHERE sif_address IS NOT NULL AND
            trim(sif_address) != '' AND 
            LEFT(sif_address, 3) = 'sif' AND 
            network = 'RUNE'  
            -- tweet_found = true AND 
            -- media_found = true
        )b
        WHERE rr = 1 AND
        sif_address IS NOT NULL AND
         trim(sif_address) != '' AND 
         LEFT(sif_address, 3) = 'sif' AND 
         network = 'RUNE'  
        --  tweet_found = true AND 
        --  media_found = true
    ) a
    WHERE r = 1