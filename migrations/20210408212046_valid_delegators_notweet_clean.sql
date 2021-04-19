-- Add migration script here
CREATE OR REPLACE VIEW valid_delegators_notweet_clean AS

   SELECT hash, height, 
memo, twitter_handle, sif_address, 
twitter_error, tweet_id, tweet_found, 
media_found, tweets_checked, manually_verified, 
from_address, address, coins
FROM (
        SELECT *, rank() over (partition by from_address order by coins asc) r
        FROM (
            select * from valid_atom_notweet 
            join valid_delegators_notweet on from_address=address
            where exists
            (
                select 1 from valid_delegators_notweet
                where from_address=address
            )
            order by from_address asc
        )b
        
    ) a
    WHERE r = 1