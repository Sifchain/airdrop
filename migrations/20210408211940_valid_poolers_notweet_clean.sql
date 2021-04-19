-- Add migration script here
CREATE OR REPLACE VIEW valid_poolers_notweet_clean AS
    SELECT hash, height, 
memo, twitter_handle, sif_address, 
twitter_error, tweet_id, tweet_found, 
media_found, tweets_checked, manually_verified, 
from_address, asset, rune_address, asset_address, stake_total_rune
FROM (
        SELECT *, rank() over (partition by from_address order by stake_total_rune asc) r
        FROM (
            select * from valid_rune_notweet 
            join valid_poolers_notweet on from_address=rune_address
            where exists
            (
                select 1 from valid_poolers_notweet
                where from_address=rune_address
            )
            order by from_address asc
        )b
        
    ) a
    WHERE r = 1