create table users (
    id int PRIMARY KEY,
    username varchar not null,
    password varchar not null
);

create table tab (
    id SERIAL PRIMARY KEY,
    title varchar not null,
    tab INTEGER[6][] NULL,
    UserID int REFERENCES users(id) ON DELETE CASCADE
);
