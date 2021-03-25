-- Add migration script here
-- Add migration script here
DROP TABLE IF EXISTS token_sale_submissions;
CREATE TABLE token_sale_submissions (
                                        id SERIAL UNIQUE NOT NULL,
                                        timestamp varchar,
                                        twitter_handle VARCHAR UNIQUE,
                                        sif_address VARCHAR UNIQUE ,
                                        contribution_code varchar UNIQUE,
                                        PRIMARY KEY (id)
);


