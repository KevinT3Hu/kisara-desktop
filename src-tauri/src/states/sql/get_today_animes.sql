SELECT DISTINCT a.*
FROM anime a
JOIN episode e ON a.id = e.anime_id
WHERE date(e.air_date) = date('now', 'localtime');
