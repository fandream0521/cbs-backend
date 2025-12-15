-- Performance indexes for common queries

-- Foreign key indexes for joins and lookups
CREATE INDEX IF NOT EXISTS idx_users_department_id ON users(department_id);
CREATE INDEX IF NOT EXISTS idx_users_role_id ON users(role_id);
CREATE INDEX IF NOT EXISTS idx_users_enable ON users(enable);
CREATE INDEX IF NOT EXISTS idx_departments_parent_id ON departments(parent_id);
CREATE INDEX IF NOT EXISTS idx_menus_parent_id ON menus(parent_id);
CREATE INDEX IF NOT EXISTS idx_menus_sort ON menus(sort);
CREATE INDEX IF NOT EXISTS idx_categories_parent_id ON categories(parent_id);
CREATE INDEX IF NOT EXISTS idx_role_menus_role_id ON role_menus(role_id);
CREATE INDEX IF NOT EXISTS idx_role_menus_menu_id ON role_menus(menu_id);

-- Search indexes for LIKE queries (SQLite FTS not used, but indexes help with prefix searches)
-- Note: Full-text LIKE queries benefit less from indexes, but these help with exact matches
CREATE INDEX IF NOT EXISTS idx_users_name ON users(name);
CREATE INDEX IF NOT EXISTS idx_departments_name ON departments(name);
CREATE INDEX IF NOT EXISTS idx_roles_name ON roles(name);
CREATE INDEX IF NOT EXISTS idx_menus_name ON menus(name);
CREATE INDEX IF NOT EXISTS idx_categories_name ON categories(name);
CREATE INDEX IF NOT EXISTS idx_goods_name ON goods(name);

-- Metrics query indexes
CREATE INDEX IF NOT EXISTS idx_goods_sale_count ON goods(sale_count DESC);
CREATE INDEX IF NOT EXISTS idx_goods_address ON goods(address) WHERE address IS NOT NULL AND address != '';
CREATE INDEX IF NOT EXISTS idx_goods_favor_count ON goods(favor_count DESC);

-- Timestamp indexes for sorting
CREATE INDEX IF NOT EXISTS idx_stories_create_at ON stories(create_at DESC);
