-- Add migration script here
DROP TABLE IF EXISTS token_sale_submissions;
CREATE TABLE token_sale_submissions (
                                        id SERIAL UNIQUE NOT NULL,
                                        twitter_handle VARCHAR,
                                        sif_address VARCHAR,
                                        contribution_code varchar,
                                        PRIMARY KEY (id)
);


