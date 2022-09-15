-- Add migration script here

DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS customers;

CREATE TABLE users
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT   NOT NULL,
    timestamp   INTEGER                             NOT NULL, 
    user_id     INTEGER                             NOT NULL,
    event       TEXT                                NOT NULL,
    data        TEXT                                NOT NULL
);

CREATE TABLE customers
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT   NOT NULL,
    timestamp   INTEGER                             NOT NULL, 
    user_id     INTEGER                             NOT NULL,
    customer_id INTEGER                             NOT NULL,
    event       TEXT                                NOT NULL,
    data        TEXT                                NOT NULL 
);
