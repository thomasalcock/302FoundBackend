user(
	id: u64,
	user_name: String,
	full_name: String,
	email: String,
	phone_numer: String,
	alias: u64)

CREATE TABLE user (
    id INTEGER PRIMARY KEY autoincrement,
    user_name VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone_number VARCHAR(255),
    alias INTEGER
);

CREATE TABLE trust (
    id INTEGER PRIMARY KEY autoincrement,
    user INTEGER NOT NULL,
    trustee INTEGER NOT NULL,
    FOREIGN KEY (user) REFERENCES user(id),
    FOREIGN KEY (trustee) REFERENCES user(id)
);

CREATE TABLE location (
    id INTEGER PRIMARY KEY autoincrement,
    user_id INTEGER NOT NULL,
    latitude INTEGER NOT NULL,
    longitude INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user(id)
);

INSERT INTO user (id, user_name, full_name, email, phone_numer, alias) VALUES ({}, {}, {}, {}, {}, {})
UPDATE user SET user_name = {}, full_name = {}, email = {}, phone_numer = {}, alias = {} WHERE id = {}
SELECT * FROM user

INSERT INTO trust (id, user, trustee) VALUES ({}, {}, {})
UPDATE trust SET user = {}, trustee = {} WHERE id = {}


// Alle Trustees eines Nutzers (maybe)
SELECT user.* FROM trust JOIN user ON trust.trustee = user.id WHERE trust.user = {}

INSERT INTO location (id, user_id, latitude, longitude) VALUES ({}, {}, {}, {})
UPDATE location SET user_id = {}, latitude = {}, longitude = {} WHERE id = {}
SELECT * FROM location WHERE user_id = {}



INSERT INTO user (id, user_name, full_name, email, phone_numer, alias) VALUES (1, 'jdoe', 'John Doe', 'j@doe.com', '123-456-7890', 1)
INSERT INTO user (id, user_name, full_name, email, phone_numer, alias) VALUES ('janedoe', 'Jane Doe', 'jane@doe.com', '098-765-4321', 2 )
INSERT INTO user (id, user_name, full_name, email, phone_numer, alias) VALUES (3, 'samsmith', 'Sam Smith', 's@smith.com', '555-555-5555', 3 )
INSERT INTO user (id, user_name, full_name, email, phone_numer, alias) VALUES (4, 'mmustermann', 'Max Mustermann', 'm@mustermann.com', '444-444-4444', 4 )
INSERT INTO user (id, user_name, full_name, email, phone_numer, alias) VALUES (5, 'emilyjones', 'Emily Jones', 'e@jones.de', '333-333-3333', 5 )
INSERT INTO user (id, user_name, full_name, email, phone_numer, alias) VALUES (6, 'liwang', 'Li Wang', 'l@wang.de', '222-222-2222', 6 )
