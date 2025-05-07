-- Get anime and episode information for the coming week starting from the current date and group by day.
SELECT a.*, e.*
FROM anime a
JOIN episode e ON a.id = e.anime_id
WHERE
    e.air_date BETWEEN DATE('now', '+0 day') AND DATE('now', '+6 days')
ORDER BY DATE(e.air_date) ASC;