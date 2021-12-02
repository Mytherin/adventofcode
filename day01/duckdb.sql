
-- part 1
WITH inputs AS (
	SELECT i, lag(i) OVER () AS last
	FROM read_csv_auto('input.txt') tbl(i)
)
SELECT SUM(CASE WHEN i>last THEN 1 ELSE 0 END) AS part1
FROM inputs;

-- part 2
WITH inputs AS (
	SELECT
		SUM(i) OVER (ROWS BETWEEN 3 PRECEDING AND 1 PRECEDING) prev_window,
		SUM(i) OVER (ROWS BETWEEN 2 PRECEDING AND CURRENT ROW) current_window,
		row_number() OVER () AS rownum
	FROM read_csv_auto('input.txt') tbl(i)
)
SELECT SUM(
	CASE WHEN current_window > prev_window AND rownum > 3
	THEN 1
	ELSE 0 END
) AS part2
FROM inputs;
