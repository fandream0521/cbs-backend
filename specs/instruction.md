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

-数据库链接和 metadata 存储在 sqlite 数据库中，放在./.cms_backend/cms_backend.db中
-后端 API 需要支持 cors， 允许所有 origin 。接口地址位于./spec/api.md
