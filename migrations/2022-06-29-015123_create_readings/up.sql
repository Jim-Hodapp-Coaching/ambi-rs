CREATE TABLE readings (
    id SERIAL PRIMARY KEY,
    temperature DOUBLE PRECISION,
    humidity DOUBLE PRECISION,
    dust_concentration DOUBLE PRECISION,
    pressure INTEGER,
    air_purity CHARACTER VARYING(255)
)