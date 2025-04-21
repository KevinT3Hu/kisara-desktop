SELECT a.*, e.*
FROM anime a
JOIN (
    SELECT anime_id, MAX(last_watch_time) AS max_watch_time
    FROM episode
    WHERE last_watch_time IS NOT NULL
    GROUP BY anime_id
) latest_episode ON a.id = latest_episode.anime_id
JOIN episode e ON e.anime_id = latest_episode.anime_id AND e.last_watch_time = latest_episode.max_watch_time
ORDER BY latest_episode.max_watch_time DESC;