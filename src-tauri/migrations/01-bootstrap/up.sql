CREATE TABLE Artists (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    cover_path TEXT
);

INSERT INTO Artists (name) VALUES ('Unknown Artist');

CREATE TABLE Albums (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    cover_path TEXT,
    artist_id INTEGER NOT NULL,
    FOREIGN KEY (artist_id) REFERENCES Artists(id)
);

INSERT INTO Albums (name, artist_id) VALUES ('Unknown Album', 1);

CREATE TABLE Tracks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    album_order INTEGER NOT NULL,
    album_id INTEGER NOT NULL,
    FOREIGN KEY (album_id) REFERENCES Albums(id)
);
