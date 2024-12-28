create table users (
    id SERIAL PRIMARY KEY,
    username varchar not null,
    password varchar not null,
    token varchar
);

create table tab (
    id SERIAL PRIMARY KEY,
    title varchar not null,
    tab INTEGER[6][] NULL,
    UserID int REFERENCES users(id) ON DELETE CASCADE,
    visibility INTEGER
);
