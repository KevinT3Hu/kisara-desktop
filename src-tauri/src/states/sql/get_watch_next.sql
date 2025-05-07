WITH
-- 1) find the last watch_time per anime (if any)
last_watched AS (
  SELECT
    anime_id,
    MAX(last_watch_time) AS last_watch
  FROM episode
  WHERE last_watch_time IS NOT NULL
  GROUP BY anime_id
),

-- 2) for those with a last_watch, get the very next episode (by sort)
next_ep AS (
  SELECT
    e.anime_id,
    MIN(e.sort) AS next_sort
  FROM episode e
  JOIN last_watched lw
    ON e.anime_id = lw.anime_id
       AND e.sort > (
         SELECT sort
         FROM episode
         WHERE anime_id = lw.anime_id
           AND last_watch_time = lw.last_watch
       )
  WHERE date(e.air_date) <= date('now')
  GROUP BY e.anime_id
),

-- 3) for anime with no last_watch, pick the first episode already aired
first_ep AS (
  SELECT
    anime_id,
    MIN(sort) AS next_sort
  FROM episode
  WHERE anime_id NOT IN (SELECT anime_id FROM last_watched)
    AND date(air_date) <= date('now')
  GROUP BY anime_id
),

-- 4) combine the two cases
candidates AS (
  SELECT * FROM next_ep
  UNION ALL
  SELECT * FROM first_ep
)

-- final join back to anime & episode to get all columns
SELECT
  a.*,      -- all anime columns
  e.*       -- all episode columns
FROM candidates c
JOIN anime a
  ON a.id = c.anime_id
JOIN episode e
  ON e.anime_id = c.anime_id
     AND e.sort = c.next_sort
;