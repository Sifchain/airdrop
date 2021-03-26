-- Add migration script here
DROP TABLE IF EXISTS invalid_token_sale_submissions;
CREATE TABLE invalid_token_sale_submissions (
                                        id SERIAL UNIQUE NOT NULL,
                                        timestamp varchar,
                                        twitter_handle VARCHAR,
                                        sif_address VARCHAR,
                                        contribution_code varchar,
                                        PRIMARY KEY (id)
);


