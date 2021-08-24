CREATE TABLE IF NOT EXISTS public.identity__user
(
    id UUID PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS public.identity__user_username
(
    id UUID PRIMARY KEY REFERENCES identity__user(id),
    username VARCHAR NOT NULL UNIQUE
    );

CREATE TABLE IF NOT EXISTS public.identity__user_email
(
    id UUID PRIMARY KEY REFERENCES identity__user(id),
    email VARCHAR NOT NULL UNIQUE
    );

CREATE TABLE IF NOT EXISTS public.identity__user_phone
(
    id UUID PRIMARY KEY REFERENCES identity__user(id),
    phone VARCHAR NOT NULL UNIQUE
    );

CREATE TABLE IF NOT EXISTS public.identity__user_enabled
(
    id UUID PRIMARY KEY REFERENCES identity__user(id),
    enabled bool NOT NULL
    );

DROP VIEW IF EXISTS identity__user_view;
CREATE VIEW identity__user_view AS
SELECT identity__user.id, username, email, phone, enabled
FROM identity__user
         LEFT JOIN identity__user_username
                   ON identity__user.id = identity__user_username.id
         LEFT JOIN identity__user_email
                   ON identity__user.id = identity__user_email.id
         LEFT JOIN identity__user_phone
                   ON identity__user.id = identity__user_phone.id
         LEFT JOIN identity__user_enabled
                   ON identity__user.id = identity__user_enabled.id

CREATE TABLE IF NOT EXISTS public.identity__group
(
    id         UUID PRIMARY KEY,
    group_name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS public.identity__user_group
(
    id       UUID PRIMARY KEY,
    user_id  UUID REFERENCES identity__user (id),
    group_id UUID REFERENCES identity__group (id),
    UNIQUE (user_id, group_id)
    );

INSERT INTO public.identity__user (id)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608');

INSERT INTO public.identity__user_username (id, username)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', 'dev-operator');

INSERT INTO public.identity__user_email (id, email)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', 'projecthocvienconggiao@gmail.com');

INSERT INTO public.identity__user_phone (id, phone)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', '+84 909683979');

INSERT INTO public.identity__user_enabled (id, enabled)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', true);

INSERT INTO public.identity__group(id, group_name)
VALUES ('d2d7b1e2-31d0-4e9d-b434-ce2f1035509d', 'AdminGroup');

INSERT INTO public.identity__group(id, group_name)
VALUES ('bc1a36eb-da8f-441b-92dc-823dfbb393c4', 'OperatorGroup');

INSERT INTO public.identity__group(id, group_name)
VALUES ('0266be52-2710-4622-bd45-7ccf7e8fa503', 'ProfessorGroup');

INSERT INTO public.identity__group(id, group_name)
VALUES ('828b522b-e749-461f-a667-16e98d103066', 'StudentGroup');

INSERT INTO public.identity__user_group(id, user_id, group_id)
VALUES ('a6e01203-afed-46d7-b20d-a8dc1c172877', '40e6215d-b5c6-4896-987c-f30f3678f608',
        '828b522b-e749-461f-a667-16e98d103066');

INSERT INTO public.identity__user_group(id, user_id, group_id)
VALUES ('1eac117c-2018-4c4f-82ec-49cfa468f0ee', '40e6215d-b5c6-4896-987c-f30f3678f608',
        '0266be52-2710-4622-bd45-7ccf7e8fa503');