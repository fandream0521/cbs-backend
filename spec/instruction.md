# Instructions

## constitution
- 使用 Rust
- 要有严格的类型标注
- 所有后端生成的 json 数据，使用 camelCase 格式。
- Rust web使用 axum， 序列化使用 serde，错误处理使用 anyhow，thiserror， 设计转换的数据结构 优先实现系统trait， 比如 From，TryFrom，TryInto， FromStr，Into等
- 所有依赖使用 2025年12月15日 时 最新的版本
- Rust 不能使用unwrap或者expect处理异常， 优先使用？进行传播，然后统一处理
- 每一个接口都需要有完善的unit test
- 每一阶段都需要使用 cargo check 处理警告和错误

## 基础思路
这是一个后台管理系统，根据给定的api接口去实现所有的接口

基本想法：

- 所有涉及的接口已给定，位于 ./spec/api.md 中
- 数据库使用sqlite进行存储
- 要有充分的日志记录，可以通过日志追踪请求的完整流程
- 所有的sql操作 需要通过日志的形式打印出sql语句，sql的参数以及当前sql执行的结果数量

后端使用 Rust 来实现


数据库链接和 metadata 存储在 sqlite 数据库中，放在~/.db_query/db_query.db中

后端 API 需要支持 cors， 允许所有 origin 。

大致API 如下：

```bash
# 获取所有已存储的数据库
GET /api/v1/dbs
# 添加一个数据库
PUT /api/v1/dbs/{name}

{
    "url": "postgres://postgres:postgres@localhost:5432/postgres"
}

# 获取一个数据库的 metadata
GET /api/vi/dbs/{name}

# 查询某个数据库的信息
POST /api/vi/dbs/{name}/query
{
    "sql": "SELECT * FROM users"
}

# 根据自然语言生成sql
POST /api/v1/dbs/{name}/query/natural
{
    "prompt": "查询用户表的所有信息"
}
```

仔细阅读当前文件夹下的代码，然后运行后端和前端，根据./fixtures/test.rest 用 curl 测试后端已实现的路由，确保所有的unit test 都通过

postgres://postgres:postgres@localhost:5432/chat
## 前端风格
