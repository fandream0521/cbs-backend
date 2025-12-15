# Tasks: Admin Backend API with SQLite Logging

**Input**: Design documents from `/specs/001-admin-api-sqlite/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Per constitution, every interface (endpoint/handler/service) MUST
include unit tests covering success and failure paths. Do not omit tests for
story tasks that touch an interface.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Initialize Rust project structure and workspace scaffolding in `src/` and `tests/`
- [X] T002 Add dependencies to `Cargo.toml` (axum, serde/serde_json, tokio, sqlx-sqlite, anyhow, thiserror, tower-http, tracing, tracing-subscriber)
- [X] T003 Configure toolchain and fmt/clippy settings; ensure `cargo fmt`/`cargo clippy` run clean
- [X] T004 [P] Create base module layout (`src/main.rs`, `src/api/`, `src/domain/`, `src/services/`, `src/infra/`, `src/auth/`)
- [X] T005 [P] Set up logging/tracing bootstrap with request ID support in `src/infra/logging.rs`
- [X] T006 [P] Configure CORS allow-all via tower-http layer in `src/infra/cors.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

-- [X] T007 Set up SQLite pool and migration runner at `./.cms_backend/cms_backend.db` in `src/infra/db.rs`
-- [X] T008 [P] Add migration files in `migrations/` for all core tables (users, departments, roles, menus, role_menus, goods, categories, stories)
- [X] T009 [P] Implement error type using thiserror/anyhow in `src/domain/error.rs`; ensure no unwrap/expect
- [X] T010 [P] Define shared response envelope and serde camelCase config in `src/api/mod.rs`
- [X] T011 [P] Configure router skeleton with health/CORS/logging middleware in `src/api/router.rs`
- [X] T012 Establish auth middleware for Bearer token validation in `src/auth/mod.rs`
- [X] T013 [P] Add test helpers (request builder, in-memory db or temp db, fixtures) in `tests/common/mod.rs`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - 登录与系统管理 CRUD 可用 (Priority: P1) 🎯 MVP

**Goal**: 登录获取 token，完成用户、部门、角色、菜单的 CRUD 与列表；鉴权覆盖所有受保护接口。

**Independent Test**: 登录获取 Bearer Token 后，独立验证用户/部门/角色/菜单的创建、更新、删除、单查、列表；无依赖其他故事。

### Tests for User Story 1

- [X] T014 [P] [US1] Unit tests for auth middleware and `/login` success/failure in `tests/unit/auth_tests.rs`
- [X] T015 [P] [US1] Unit tests for users/roles/departments/menus handlers (success/failure) in `tests/unit/system_handlers_tests.rs`
- [X] T016 [P] [US1] Integration test for login + protected route access in `tests/integration/auth_flow.rs`
- [X] T017 [P] [US1] Integration tests for system CRUD/list endpoints in `tests/integration/system_crud.rs`

### Implementation for User Story 1

- [X] T018 [P] [US1] Implement auth middleware + token validator in `src/auth/mod.rs`
- [X] T019 [P] [US1] Implement `/login` handler and service in `src/api/auth.rs` and `src/services/auth_service.rs`
 - [X] T020 [P] [US1] Implement `/test` token validation endpoint in `src/api/auth.rs`
- [X] T021 [P] [US1] Define User/Department/Role/Menu models and DTOs in `src/domain/system.rs`
- [X] T022 [P] [US1] Implement repositories for system entities in `src/infra/system_repo.rs` (users, departments, roles, menus, role_menus)
- [X] T023 [US1] Implement services for system entities in `src/services/system_service.rs`
 - [X] T024 [US1] Implement handlers/routes for system CRUD/list in `src/api/system.rs`; wire into router
 - [X] T025 [US1] Ensure logging of SQL (statement/params/rows) and request tracing in system flows
 - [X] T026 [US1] Add input validation and error mapping for system endpoints in `src/api/system.rs`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - 商品与分类管理 (Priority: P2)

**Goal**: 运营可维护商品与分类（CRUD、分页、单查），保持价格/库存等字段准确。

**Independent Test**: 具备有效 token 后，可独立验证 goods/category 的新增、修改、删除、单查、列表，不依赖其他故事。

### Tests for User Story 2

- [X] T027 [P] [US2] Unit tests for goods/category handlers (success/failure) in `tests/unit/goods_handlers_tests.rs`
- [X] T028 [P] [US2] Integration tests for goods/category CRUD/list in `tests/integration/goods_crud.rs`

### Implementation for User Story 2

- [X] T029 [P] [US2] Define Goods/Category models and DTOs in `src/domain/goods.rs`
- [X] T030 [P] [US2] Implement repositories for goods/category in `src/infra/goods_repo.rs`
- [X] T031 [US2] Implement services for goods/category in `src/services/goods_service.rs`
- [X] T032 [US2] Implement handlers/routes for goods/category CRUD/list in `src/api/goods.rs`; wire into router
- [X] T033 [US2] Ensure logging of SQL (statement/params/rows) and validation for goods/category endpoints

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - 故事发布与菜单/图表查询 (Priority: P3)

**Goal**: 支持故事发布与列表查询；菜单树、角色菜单、图表数据接口可用。

**Independent Test**: 具备 token 后，可独立测试故事发布/列表、菜单树查询、图表数据获取。

### Tests for User Story 3

- [X] T034 [P] [US3] Unit tests for story/menu tree/metrics handlers (success/failure) in `tests/unit/story_metrics_handlers_tests.rs`
- [X] T035 [P] [US3] Integration tests for story, menu tree, role menu, metrics endpoints in `tests/integration/story_metrics.rs`

### Implementation for User Story 3

- [X] T036 [P] [US3] Implement story repository/service/handlers in `src/infra/story_repo.rs`, `src/services/story_service.rs`, `src/api/story.rs`
- [X] T037 [P] [US3] Implement menu tree and role menu endpoints in `src/api/menu_tree.rs`
- [X] T038 [US3] Implement metrics endpoints (category count/sale/favor, top10, address sale, amount list) in `src/api/metrics.rs`
- [X] T039 [US3] Ensure logging and validation for story/metrics endpoints; reuse response envelope

**Checkpoint**: All user stories should now be independently functional

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T040 [P] Documentation updates (spec/quickstart) after implementation in `specs/001-admin-api-sqlite/`
- [X] T041 [P] Add/adjust migrations or indexes for performance in `migrations/`
- [X] T042 [P] Performance tuning and tracing spans around heavy queries in `src/`
- [X] T043 [P] Additional unit/integration tests for edge cases in `tests/`
- [X] T044 Security hardening review (auth, error messages, logging PII) across `src/`
- [X] T045 Run quickstart validation and end-to-end smoke in `specs/001-admin-api-sqlite/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

