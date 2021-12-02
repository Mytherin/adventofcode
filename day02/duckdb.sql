CREATE TABLE inputs AS
SELECT *
FROM read_csv_auto('input.txt', sep=' ') tbl("action", "amount");

-- part 1
SELECT
   SUM(CASE (action)
     WHEN 'forward' THEN amount
     ELSE 0 END) *
   SUM(CASE (action)
     WHEN 'down' THEN amount
     WHEN 'up' THEN -amount
     ELSE 0 END) AS part1
FROM inputs;

-- part 2
WITH aim_computation AS (
SELECT action, amount, SUM(
	CASE (action)
	WHEN 'down' THEN amount
	WHEN 'up' THEN -amount
	ELSE 0 END
) OVER (ORDER BY rowid) AS aim
FROM inputs)
SELECT
   SUM(CASE (action)
     WHEN 'forward' THEN amount
     ELSE 0 END) *
   SUM(CASE (action)
     WHEN 'forward' THEN amount*aim
     ELSE 0 END) AS part2
FROM aim_computation;
