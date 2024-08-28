CREATE TABLE paints (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    brand INTEGER NOT NULL REFERENCES brands(id),
    color VARCHAR(9)
)
