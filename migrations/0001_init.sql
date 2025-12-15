-- Users
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    realname TEXT,
    password TEXT NOT NULL,
    cellphone TEXT,
    enable INTEGER NOT NULL DEFAULT 1,
    department_id INTEGER,
    role_id INTEGER,
    create_at TEXT DEFAULT (CURRENT_TIMESTAMP),
    update_at TEXT DEFAULT (CURRENT_TIMESTAMP)
);

-- Departments
CREATE TABLE IF NOT EXISTS departments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    parent_id INTEGER,
    leader TEXT,
    create_at TEXT DEFAULT (CURRENT_TIMESTAMP),
    update_at TEXT DEFAULT (CURRENT_TIMESTAMP),
    UNIQUE(name, parent_id)
);

-- Roles
CREATE TABLE IF NOT EXISTS roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    intro TEXT,
    create_at TEXT DEFAULT (CURRENT_TIMESTAMP),
    update_at TEXT DEFAULT (CURRENT_TIMESTAMP)
);

-- Menus
CREATE TABLE IF NOT EXISTS menus (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    type INTEGER NOT NULL,
    url TEXT,
    icon TEXT,
    sort INTEGER,
    parent_id INTEGER,
    create_at TEXT DEFAULT (CURRENT_TIMESTAMP),
    update_at TEXT DEFAULT (CURRENT_TIMESTAMP),
    UNIQUE(name, parent_id)
);

-- RoleMenus (join)
CREATE TABLE IF NOT EXISTS role_menus (
    role_id INTEGER NOT NULL,
    menu_id INTEGER NOT NULL,
    PRIMARY KEY (role_id, menu_id)
);

-- Goods
CREATE TABLE IF NOT EXISTS goods (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    old_price REAL,
    new_price REAL,
    "desc" TEXT,
    status INTEGER,
    img_url TEXT,
    inventory_count INTEGER,
    sale_count INTEGER,
    favor_count INTEGER,
    address TEXT,
    create_at TEXT DEFAULT (CURRENT_TIMESTAMP),
    update_at TEXT DEFAULT (CURRENT_TIMESTAMP)
);

-- Categories
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    parent_id INTEGER,
    create_at TEXT DEFAULT (CURRENT_TIMESTAMP),
    update_at TEXT DEFAULT (CURRENT_TIMESTAMP),
    UNIQUE(name, parent_id)
);

-- Stories
CREATE TABLE IF NOT EXISTS stories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    create_at TEXT DEFAULT (CURRENT_TIMESTAMP),
    update_at TEXT DEFAULT (CURRENT_TIMESTAMP)
);

