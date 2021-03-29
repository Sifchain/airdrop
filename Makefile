

extract_data:
	cargo r --bin rune
	cargo r --bin atom
	cargo r --bin rune_pool_extract
	cargo r --bin check_twitter


import-data-extracts:
	psql "postgresql://postgres:password@localhost:5432/airdrop" -f ./import_extracted_data.sql
