CREATE TABLE IF NOT EXISTS "user"
(
    id       uuid        NOT NULL PRIMARY KEY,
    username varchar(50) NOT NULL,
    email    varchar(50),
    phone    varchar(50)
    );

CREATE UNIQUE index user_id_uindex
    ON "user" (id);
