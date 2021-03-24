-- Add migration script here
ALTER TABLE token_sale_submissions
    ADD COLUMN timestamp varchar;
