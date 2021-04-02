-- Add migration script here
CREATE OR REPLACE VIEW valid_KYC AS
    SELECT id, timestamp, twitter_handle, sif_address, contribution_code, twitter_error, tweet_id, tweet_found, media_found, tweets_checked FROM (
        SELECT *, rank() over (partition by sif_address order by id asc) r
        FROM token_sale_submissions
        WHERE sif_address IS NOT NULL AND trim(sif_address) != '' AND LEFT(sif_address, 3) = 'sif' AND tweet_found = true AND media_found = true
    ) a
    WHERE r = 1