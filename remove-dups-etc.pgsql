SELECT id, network, hash, height, memo, twitter_handle, sif_address, twitter_error, tweet_id, tweet_found, media_found, tweets_checked, manually_verified, from_address FROM (
    SELECT *, rank() over (partition by sif_address order by id asc) r
    FROM valid_rune
    WHERE sif_address IS NOT NULL AND trim(sif_address) != '' AND LEFT(sif_address, 3) = 'sif'
) a
WHERE r = 1
-- GROUP BY  sif_address
-- HAVING COUNT(sif_address) = 1