-- Add migration script here

DROP TABLE IF EXISTS token_sale_contribution_codes;
CREATE TABLE token_sale_contribution_codes (
                                        id SERIAL UNIQUE NOT NULL,
                                        contribution_code varchar,
                                        PRIMARY KEY (id,contribution_code)
);


