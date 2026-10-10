-- Your SQL goes here
CREATE TABLE tours (
id SERIAL PRIMARY KEY,
name TEXT NOT NULL UNIQUE,
ratings_avg FLOAT DEFAULT 4.5,
ratings_qty INT,
images TEXT[],
image_cover TEXT,
start_dates DATE[],
duration INT,
max_group_size INT,
difficulty TEXT,
price INT,
summary TEXT,
description TEXT,
created_at TIMESTAMPTZ DEFAULT NOW(),
updated_at TIMESTAMPTZ
);
