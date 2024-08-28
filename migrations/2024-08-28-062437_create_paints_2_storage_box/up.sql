CREATE TABLE paints_2_storage_boxes (
    id SERIAL PRIMARY KEY,
    number INTEGER NOT NULL,
    paint INTEGER NOT NULL REFERENCES paints(id),
    storage_box INTEGER NOT NULL REFERENCES storage_boxes(id)
)
