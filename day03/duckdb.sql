CREATE TABLE inputs AS
SELECT *
FROM read_csv_auto('input.txt', sep=' ') tbl("action", "amount");

-- part 1