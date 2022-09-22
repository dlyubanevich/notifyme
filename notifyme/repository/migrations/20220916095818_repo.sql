-- Add migration script here

DROP TABLE IF EXISTS products;
DROP TABLE IF EXISTS customers;
DROP TABLE IF EXISTS customers_products;
DROP TABLE IF EXISTS subscriptions;

CREATE TABLE "customers" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"key"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "products" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "customers_products" (
	"customer_id"	INTEGER NOT NULL,
	"product_id"	INTEGER NOT NULL,
	FOREIGN KEY("customer_id") REFERENCES "customers"("id") on delete cascade
);

CREATE TABLE "users_customers" (
	"user_id"	INTEGER NOT NULL,
	"customer_id"	INTEGER NOT NULL,
	FOREIGN KEY("customer_id") REFERENCES "customers"("id") on delete cascade
);

CREATE TABLE "subscriptions" (
	"id"	INTEGER NOT NULL,
	"user_id"	INTEGER NOT NULL,
	"customer_id"	INTEGER NOT NULL,
	"product_id"	INTEGER NOT NULL,
	FOREIGN KEY("customer_id") REFERENCES "customers"("id") on delete cascade,
	FOREIGN KEY("product_id") REFERENCES "products"("id") on delete cascade,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "notifications" (
	"id"	INTEGER NOT NULL,
	"customer_id"	INTEGER NOT NULL,
	"product_id"	INTEGER NOT NULL,
	"text"	TEXT NOT NULL,
	FOREIGN KEY("product_id") REFERENCES "products"("id") ON DELETE CASCADE,
	FOREIGN KEY("customer_id") REFERENCES "customers"("id") ON DELETE CASCADE,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "active_subscriptions" (
	"subscription_id"	INTEGER NOT NULL,
	FOREIGN KEY("subscription_id") REFERENCES "subscriptions"("id") ON DELETE CASCADE
);

CREATE TABLE "active_notifications" (
	"notification_id"	INTEGER NOT NULL,
	FOREIGN KEY("notification_id") REFERENCES "notifications"("id") ON DELETE CASCADE
);
