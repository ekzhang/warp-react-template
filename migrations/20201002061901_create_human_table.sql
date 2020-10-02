CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE episode AS ENUM ('new hope', 'empire', 'jedi');

CREATE TABLE humans (
    id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    name text NOT NULL,
    appears_in episode,
    home_planet text NOT NULL
);
