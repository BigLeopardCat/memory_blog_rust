CREATE DATABASE IF NOT EXISTS memory_blog;
USE memory_blog;

CREATE TABLE IF NOT EXISTS user (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user'
);

CREATE TABLE IF NOT EXISTS category (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    introduce VARCHAR(255),
    path_name VARCHAR(255),
    icon VARCHAR(255),
    color VARCHAR(50)
);

CREATE TABLE IF NOT EXISTS note (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    description TEXT,
    cover VARCHAR(255),
    is_top INT DEFAULT 0,
    status VARCHAR(50) DEFAULT 'published',
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    is_public BOOLEAN NOT NULL DEFAULT 1,
    category_id INT,
    FOREIGN KEY (category_id) REFERENCES category(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS tag_one (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    level INT,
    color VARCHAR(50)
);

CREATE TABLE IF NOT EXISTS tag_two (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    level INT,
    color VARCHAR(50),
    tag_one_id INT,
    FOREIGN KEY (tag_one_id) REFERENCES tag_one(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS friend (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    link VARCHAR(255) NOT NULL,
    avatar VARCHAR(255),
    description VARCHAR(255),
    status INT DEFAULT 1
);

CREATE TABLE IF NOT EXISTS talk (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255),
    content TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS web_info (
    id INT AUTO_INCREMENT PRIMARY KEY,
    key_name VARCHAR(255) NOT NULL UNIQUE,
    value TEXT
);

INSERT IGNORE INTO user (username, password, role) VALUES ('admin', '123456', 'admin');
INSERT IGNORE INTO category (name, introduce, path_name, icon, color) VALUES ('Default', 'Default Category', 'default', 'home', '#000000');

CREATE USER IF NOT EXISTS 'memory_blog'@'localhost' IDENTIFIED BY '123456';
GRANT ALL PRIVILEGES ON memory_blog.* TO 'memory_blog'@'localhost';
FLUSH PRIVILEGES;
