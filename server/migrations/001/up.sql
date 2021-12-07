CREATE EXTENSION citext;
CREATE DOMAIN email AS citext
    CHECK ( value ~ '^[a-zA-Z-1-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

CREATE TABLE statham_users
(
    id       SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email    email,
    otp      VARCHAR -- Not used for now, could be used to prevent people just asking for things.
);

CREATE TABLE statham_servers
(
    id            SERIAL PRIMARY KEY,
    hostname      VARCHAR NOT NULL,
    v4_ip_address inet,
    v6_ip_address inet
);

CREATE TABLE statham_passwords (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    machine_id INTEGER NOT NULL,
    password VARCHAR,
    last_updated  date NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX statham_unique_triple
    ON statham_passwords (
                           user_id,
                           machine_id
    );