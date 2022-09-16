-- Add migration script here

DROP TABLE IF EXISTS products;
DROP TABLE IF EXISTS customers;
DROP TABLE IF EXISTS customers_products;
DROP TABLE IF EXISTS subscriptions;

CREATE TABLE customers
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT   NOT NULL,
    name        TEXT                                NOT NULL,
    key         TEXT                                NOT NULL
);

CREATE TABLE products
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT   NOT NULL,
    name        TEXT                                NOT NULL
);

CREATE TABLE customers_products
(
    customer_id       INTEGER                       NOT NULL,
    product_id        INTEGER                       NOT NULL,
    FOREIGN KEY (customer_id) REFERENCES customers(id) on delete cascade
);

CREATE TABLE users_customers
(
    user_id           INTEGER                       NOT NULL,
    customer_id       INTEGER                       NOT NULL,
    FOREIGN KEY (customer_id) REFERENCES customers(id) on delete cascade
);

CREATE TABLE subscriptions
(
    user_id           INTEGER                       NOT NULL,
    customer_id       INTEGER                       NOT NULL,
    product_id        INTEGER                       NOT NULL,
    FOREIGN KEY (customer_id) REFERENCES customers(id) on delete cascade,
    FOREIGN KEY (product_id) REFERENCES products(id) on delete cascade
);
