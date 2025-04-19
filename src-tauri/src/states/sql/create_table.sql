CREATE TABLE IF NOT EXISTS anime (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    name_cn TEXT NOT NULL,
    image TEXT NOT NULL,
    release_date TEXT
);

CREATE TABLE IF NOT EXISTS episode (
    id INTEGER PRIMARY KEY,
    anime_id INTEGER NOT NULL,
    sort INTEGER NOT NULL,
    ep INTEGER,
    name TEXT NOT NULL,
    name_cn TEXT NOT NULL,
    air_date TEXT,
    progress INTEGER NOT NULL DEFAULT 0,
    last_watch_time TEXT,
    torrent_id TEXT,
    FOREIGN KEY (anime_id) REFERENCES anime(id)
);