# Feature Specification: Admin Backend API with SQLite Logging

**Feature Branch**: `001-admin-api-sqlite`  
**Created**: 2025-12-15  
**Status**: Draft  
**Input**: 用户需求：后台管理系统，按 `specs/api.md` 提供的接口实现全部 API；数据存储使用 SQLite；需要完整日志，可追踪请求全流程；所有 SQL 操作需记录语句、参数及返回结果数量。

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
  
  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - 登录与系统管理 CRUD 可用 (Priority: P1)

管理员可登录后台并管理系统核心资源（用户、部门、角色、菜单），满足鉴权、分页查询与单项 CRUD。

**Why this priority**: 无登录与系统资源管理则无法使用后台，属于根基能力。

**Independent Test**: 使用登录获取 Bearer Token 后，对用户/部门/角色/菜单的创建、查询、更新、删除与列表查询可独立验证且不依赖其他故事。

**Acceptance Scenarios**:

1. **Given** 正确的用户名密码，**When** 调用 `/login`，**Then** 返回 token 与用户信息。
2. **Given** 合法 token，**When** 调用 `/users/list` 并传入分页条件，**Then** 返回列表与 totalCount。
3. **Given** 合法 token，**When** 创建/更新/删除 用户、部门、角色、菜单，**Then** 资源持久化且返回统一结构（code/message/data）。
4. **Given** token 失效或缺失，**When** 访问受保护接口，**Then** 返回鉴权失败响应且不触发数据写入。

---

### User Story 2 - 商品与分类管理 (Priority: P2)

运营人员可维护商品与分类信息（CRUD、分页查询、单项查询），保持库存、价格等字段准确。

**Why this priority**: 商品与分类是业务数据核心，直接影响前台展示与分析。

**Independent Test**: 在具备有效 token 情况下，对 goods/category 的新增、修改、删除、单查、列表查可独立验证，不依赖其他故事。

**Acceptance Scenarios**:

1. **Given** 商品创建请求体，**When** 调用 `/goods`（创建）或 `/goods/{id}`（更新/删除），**Then** 返回成功且数据持久化。
2. **Given** 分类创建请求体，**When** 调用 `/category` 相关 CRUD 接口，**Then** 返回成功且列表查询可见最新数据。
3. **Given** 无 body 的商品列表查询，**When** 调用 `/goods/list`，**Then** 返回列表和 totalCount，分页正确。

---

### User Story 3 - 故事发布与菜单/图表查询 (Priority: P3)

用户可发布故事内容，前台可获取菜单树与图表统计数据用于展示与分析。

**Why this priority**: 增强内容和可视化能力，提升使用体验但不阻塞核心 CRUD。

**Independent Test**: 在获取 token 后可独立测试故事发布/列表、菜单树查询、统计图表数据获取，与其他故事的持久化逻辑解耦。

**Acceptance Scenarios**:

1. **Given** 故事发布请求体，**When** 调用 `/story`，**Then** 返回 code=200 且数据可在 `/story/list` 中查询到。
2. **Given** 无请求体，**When** 调用菜单树与角色菜单接口，**Then** 返回完整树结构且字段齐全。
3. **Given** 无请求体，**When** 调用各类图表数据接口，**Then** 返回数据数组，字段满足示例结构。

---

[Add more user stories as needed, each with an assigned priority]

### Edge Cases

- 登录失败（用户名/密码错误、账户禁用）返回明确错误，不暴露系统信息。
- Token 过期或缺失时，所有受保护接口拒绝访问且不执行数据库操作。
- 分页参数越界（offset/size 非法或过大）返回校验错误，不降级为全量查询。
- 创建或更新资源时字段缺失/类型错误，返回校验错误并不写库。
- 资源不存在时的 GET/PATCH/DELETE 返回 404 类语义错误，不产生副作用。
- 重名/约束冲突（如角色/菜单名称唯一约束）返回冲突错误并记录原因。
- SQLite 锁冲突或执行异常时，保证事务回滚并记录错误日志与 SQL。
- 日志写入失败（磁盘/权限）不应导致接口 panic，应记录降级信息。

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
  Constitution alignment:
  - APIs/handlers must target Rust (axum + serde) with camelCase JSON contracts.
  - Error handling uses Result propagation (no unwrap/expect); note expected
    error cases.
  - Plan unit tests for every interface (success + failure) and ensure cargo
    check remains warning-free.
-->

### Functional Requirements

- **FR-001**: 提供登录接口 `/login`，验证用户名/密码，返回 token 与用户信息；提供 `/test` 校验 token 有效性。
- **FR-002**: 用户管理：支持用户的创建、更新、删除、单查以及列表查询（含分页与过滤），响应包含 totalCount。
- **FR-003**: 部门管理：支持部门的创建、更新、删除、单查、列表查询，确保 parentId 合法。
- **FR-004**: 角色管理：支持角色的创建、更新、删除、单查、列表查询，支持分配菜单权限并返回树形结构和 menuIds。
- **FR-005**: 菜单管理：支持菜单的创建、更新、删除、单查、列表查询与完整树获取，保留层级关系。
- **FR-006**: 商品管理：支持商品创建、更新、删除、单查、列表查询，字段覆盖价格、库存、销量、图片等示例字段。
- **FR-007**: 分类管理：支持分类创建、更新、删除、单查、列表查询，支持按名称过滤与分页。
- **FR-008**: 故事发布：支持故事创建与列表查询，返回字段与示例一致。
- **FR-009**: 图表数据：提供商品分类数量、销量、收藏、Top10、城市销量、商品统计数量等查询接口，返回数组结构按示例字段输出。
- **FR-010**: 所有接口遵循统一响应包装（code/message/data），code 与 message 填充与示例一致或更具体的错误信息。
- **FR-011**: 需要完整日志链路：每个请求包含可追踪的请求 ID，记录入口请求、鉴权结果、处理耗时、响应状态。
- **FR-012**: 所有 SQL 操作必须记录 SQL 语句、参数、影响/返回的记录数量；错误时记录错误详情并与请求 ID 关联。
- **FR-013**: 持久化使用 SQLite，支持迁移/初始化脚本以确保表结构满足所有字段与关系。
- **FR-014**: 输入校验必需，拒绝缺失或类型错误字段；空 body 的接口按示例行为处理（例如列表查询可接受空对象）。
- **FR-015**: 权限控制：除登录外的接口均需 Bearer Token，非法或缺失 token 返回鉴权失败，避免数据泄露。
- **FR-016**: 所有接口必须有单元测试覆盖成功与失败路径，确保契约稳定。

### Key Entities *(include if feature involves data)*

- **User**: id, name, realname, cellphone, enable, departmentId, roleId, timestamps.
- **Department**: id, name, parentId, leader, timestamps.
- **Role**: id, name, intro, menuList/menuIds, timestamps.
- **Menu**: id, name, type, url, icon, sort, parentId, children (树结构), timestamps.
- **Goods**: id, name, oldPrice, newPrice, status, imgUrl, inventoryCount, saleCount, favorCount, address, timestamps.
- **Category**: id, name, parentId, timestamps.
- **Story**: id, title, content, timestamps.
- **MetricsItem**: name/value 或 address/count，用于图表统计。
- **AuthToken**: token 字符串及过期策略（用于鉴权流程）。

## Assumptions & Dependencies

- `specs/api.md` 中的接口定义为最终契约，未列出的接口不在范围内。
- 鉴权方式固定为登录获取 Bearer Token，暂无额外的角色/权限体系变更需求。
- SQLite 作为单节点存储，运行环境可读写数据库文件；高并发与分布式扩展不在本次范围。
- 日志落地介质可用（文件/控制台），日志保留策略由运营侧另行制定，不在本次实现范围。

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: 登录成功率 ≥ 99%，有效 token 校验通过率 ≥ 99%，失败场景返回明确错误码/消息。
- **SC-002**: 全部列出的 API 在集成测试中 CRUD/查询通过率 100%，响应结构字段与 `specs/api.md` 契约一致。
- **SC-003**: 100% SQL 操作生成包含语句、参数、影响行数/结果数量的日志，可通过请求 ID 关联重放。
- **SC-004**: 关键接口端到端平均响应时间在 500ms 内（本地/测试环境基准），95% 分位不超过 800ms。
- **SC-005**: 所有接口具备单元测试覆盖成功/失败路径；`cargo check` 与测试在 CI 本地均 0 警告/错误。
