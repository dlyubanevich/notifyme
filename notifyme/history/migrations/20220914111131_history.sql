-- Add migration script here

DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS customers;

CREATE TABLE "users" (
	"id"	INTEGER NOT NULL,
	"timestamp"	INTEGER NOT NULL,
	"user_id"	INTEGER NOT NULL,
	"event"	TEXT NOT NULL,
	"data"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "customers" (
	"id"	INTEGER NOT NULL UNIQUE,
	"timestamp"	INTEGER NOT NULL,
	"user_id"	INTEGER NOT NULL,
	"customer"	TEXT NOT NULL,
	"event"	TEXT NOT NULL,
	"data"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
