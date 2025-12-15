# Data Model: Admin Backend API with SQLite Logging

## Entities

- **User**
  - Fields: id, name, realname, password (hashed), cellphone, enable, departmentId, roleId, createAt, updateAt.
  - Constraints: name unique; cellphone format; enable is boolean/int flag.
  - Relations: belongs to Department; belongs to Role.

- **Department**
  - Fields: id, name, parentId, leader, createAt, updateAt.
  - Constraints: parentId optional/self-referencing; name unique per parent.
  - Relations: optional parent Department; has many Users.

- **Role**
  - Fields: id, name, intro, createAt, updateAt.
  - Constraints: name unique.
  - Relations: many-to-many with Menu via RoleMenu; assigned to Users.

- **Menu**
  - Fields: id, name, type, url, icon, sort, parentId, createAt, updateAt.
  - Constraints: type in {1,2,3}; sort numeric; parentId optional; name unique per parent.
  - Relations: self-referencing tree; many-to-many with Role via RoleMenu.

- **RoleMenu** (join)
  - Fields: roleId, menuId.
  - Constraints: composite PK (roleId, menuId).

- **Goods**
  - Fields: id, name, oldPrice, newPrice, desc, status, imgUrl, inventoryCount, saleCount, favorCount, address, createAt, updateAt.
  - Constraints: name required; numeric fields non-negative.
  - Relations: belongs to Category (optional).

- **Category**
  - Fields: id, name, parentId, createAt, updateAt.
  - Constraints: name unique per parent; parentId optional/self-referencing.
  - Relations: self-referencing tree; has many Goods.

- **Story**
  - Fields: id, title, content, createAt, updateAt.
  - Constraints: title required; content required.

- **MetricsItem (derived)**
  - Fields: name/value or address/count depending on endpoint.
  - Purpose: returned by analytics endpoints; not persisted separately.

- **AuthToken (runtime)**
  - Fields: token string, expiry.
  - Purpose: response payload and validation input; not persisted.

## Validation Rules

- Required fields per spec payloads must be present; type-check all numbers/strings.
- Pagination: offset >= 0, size > 0 and bounded (e.g., <= 100).
- Referential integrity: parentId and foreign keys must reference existing rows.
- Enum/type fields (menu.type, enable) validated against allowed set.
- Uniqueness: user.name, role.name, menu (name per parent), category (name per parent).

## Notes

- All timestamps stored in ISO-like text or numeric; align with SQLite schema.
- Migrations must create indexes on foreign keys and common filters (name, parentId).

