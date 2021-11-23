-- Your SQL goes here
CREATE TABLE `books` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    `olid` TEXT NOT NULL,
    `uid` TEXT UNIQUE,
    `title` TEXT NOT NULL,
    `author` TEXT,
    `work` TEXT
);

CREATE TABLE `authors` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    `olid` TEXT NOT NULL,
    `name` TEXT NOT NULL
);

CREATE TABLE `works` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    `olid` TEXT NOT NULL,
    `title` TEXT NOT NULL,
    `author` TEXT
);
