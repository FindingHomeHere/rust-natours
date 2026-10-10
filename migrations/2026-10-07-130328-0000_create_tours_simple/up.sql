-- Your SQL goes here
CREATE TABLE tours (
id SERIAL PRIMARY KEY,
name TEXT NOT NULL UNIQUE,
ratings_avg NUMERIC(3, 2),
ratings_qty INT,
images TEXT[],
image_cover TEXT,
start_dates DATE[],
duration INT,
max_group_size INT,
difficulty TEXT,
price INT,
summary TEXT,
created_at TIMESTAMPTZ,
updated_at TIMESTAMPTZ
);
