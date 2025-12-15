mall_cms_v2 接口文档
=====================

> 由 `spec/postman.json` 整理。`{{baseURL}}`/`{{baseURL2}}` 为变量，请按环境替换。除登录外，其余接口均使用 Bearer Token（`Authorization: Bearer {{token}}`）。

## 统一响应结构, data类型为泛型 T, 可能是任何类型，比如对象或数组
```json
{
  "code": 200,
  "message": "成功",
  "data": T
}
```

用户登录
--------

### 用户登录
- 方法：`POST`
- 路径：`{{baseURL}}/login`
- 请求参数（JSON）：
  ```json
  {
    "name": "coderwhy",
    "password": "123456"
  }
  ```
- 响应结构示例：
  ```json
  {
    "code": 0,
    "message": "查询成功",
    "data": {
      "token": "Bearer xxxxxx",
      "user": {
        "id": 1,
        "name": "coderwhy",
        "realname": "coderwhy",
        "cellphone": "13333333333",
        "enable": 1,
        "createAt": "2021-08-19T08:00:00.000Z",
        "updateAt": "2021-08-19T08:00:00.000Z",
        "role": {
          "id": 1,
          "name": "超级管理员",
          "intro": "所有权限",
          "createAt": "2021-08-19T08:00:00.000Z",
          "updateAt": "2021-08-19T08:00:00.000Z",
        },
        "department": {
          "id": 1,
          "name": "研发部",
          "parentId": null,
          "leader": "coderwhy",
          "createAt": "2021-08-19T08:00:00.000Z",
          "updateAt": "2021-08-19T08:00:00.000Z",
        }
      }
    }
  }
  ```

### 验证登录
- 方法：`GET`
- 路径：`{{baseURL}}/test`
- 认证：Bearer `{{token}}`
- 请求参数：无
- 响应结构示例（用于校验 token 有效性）：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": {
      "valid": true
    }
  }
  ```

系统管理
--------

### 用户管理
- 创建用户 — `POST {{baseURL}}/users`
  ```json
  {
    "name": "james",
    "realname": "詹姆斯",
    "password": "123456",
    "cellphone": 13322223338,
    "departmentId": 1,
    "roleId": 1
  }
  ```
- 删除用户 — `DELETE {{baseURL}}/users/{id}`（示例 `4`）
- 修改用户 — `PATCH {{baseURL}}/users/{id}`（示例 `3`）
  ```json
  {
    "password": "12345",
    "cellphone": "15566668888"
  }
  ```
- 查询某个用户 — `GET {{baseURL}}/users/{id}`（示例 `1`）
- 查询用户列表 — `POST {{baseURL}}/users/list`
  ```json
  {
    "offset": 0,
    "size": 10,
    "name": "w",
    "cellphone": 4
  }
  ```
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": {
      "list": [
        {
          "id": 1,
          "name": "coderwhy",
          "realname": "coderwhy",
          "cellphone": "13333333333",
          "enable": 1,
          "departmentId": 1,
          "roleId": 1,
          "createAt": "2021-08-19T08:00:00.000Z",
          "updateAt": "2021-08-19T08:00:00.000Z"
        }
      ],
      "totalCount": 1
    }
  }
  ```

### 部门管理
- 创建部门 — `POST {{baseURL}}/department`
  ```json
  {
    "name": "人事部",
    "parentId": 1,
    "leader": "coderwhy"
  }
  ```
- 删除部门 — `DELETE {{baseURL}}/department/{id}`（示例 `11`）
- 更新部门 — `PATCH {{baseURL}}/department/{id}`（示例 `4`）
  ```json
  {
    "leader": "lily",
    "parentId": 2
  }
  ```
- 获取某个部门 — `GET {{baseURL}}/department/{id}`（示例 `4`）
- 获取部门列表 — `POST {{baseURL}}/department/list`
  ```json
  {
    "offset": 0,
    "size": 2
  }
  ```
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": {
      "list": [
        {
          "id": 4,
          "name": "研发部",
          "parentId": 1,
          "leader": "coderwhy",
          "createAt": "2021-08-19T08:00:00.000Z",
          "updateAt": "2021-08-19T08:00:00.000Z"
        }
      ],
      "totalCount": 1
    }
  }
  ```

### 角色管理
- 创建角色 — `POST {{baseURL}}/role`
  ```json
  {
    "name": "运营2",
    "intro": "日常管理2",
    "menuList": [1, 9, 38, 41, 2, 3, 4, 25, 5, 6, 7, 8, 17, 18, 19, 20, 21, 22, 23, 24, 15, 16, 30, 31, 32, 33, 34, 35, 36, 37, 26, 27, 28, 29, 39, 40, 42, 43]
  }
  ```
- 删除角色 — `DELETE {{baseURL}}/role/{id}`（示例 `2`）
- 更新角色 — `PATCH {{baseURL}}/role/{id}`（示例 `3`）
  ```json
  {
    "intro": "日常事务"
  }
  ```
- 获取某个角色 — `GET {{baseURL}}/role/{id}`（示例 `1`）
- 获取角色列表 — `POST {{baseURL}}/role/list`
  ```json
  {
    "offset": 0,
    "size": 100
  }
  ```
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": {
      "list": [
        {
          "id": 1,
          "name": "超级管理员",
          "intro": "所有权限",
          "createAt": "2021-08-19T08:00:00.000Z",
          "updateAt": "2021-08-19T08:00:00.000Z",
          "menuList": [1, 9, 38]
        }
      ],
      "totalCount": 1
    }
  }
  ```

### 菜单管理
- 创建菜单 — `POST {{baseURL}}/menu`
  ```json
  {
    "name": "商品信息",
    "type": 2,
    "url": "/product/goods",
    "sort": 104,
    "parentId": 9
  }
  ```
- 删除菜单 — `DELETE {{baseURL}}/menu/{id}`（示例 `43`）
- 修改菜单 — `PATCH {{baseURL}}/menu/{id}`（示例 `43`）
  ```json
  {
    "name": "test测试",
    "type": 2,
    "url": "/demo",
    "icon": "el-icon-menu",
    "sort": 120,
    "parentId": 1
  }
  ```
- 查询某个菜单 — `GET {{baseURL}}/menu/{id}`（示例 `1`）
- 查询菜单列表 — `POST {{baseURL}}/menu/list`
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": [
      {
        "id": 1,
        "name": "系统管理",
        "type": 1,
        "url": "/main/system",
        "icon": "el-icon-setting",
        "sort": 1,
        "parentId": null,
        "createAt": "2021-08-19T08:00:00.000Z",
        "updateAt": "2021-08-19T08:00:00.000Z",
        "children": [
          {
            "id": 9,
            "name": "用户管理",
            "type": 2,
            "url": "/main/system/user",
            "icon": "el-icon-user",
            "sort": 10,
            "parentId": 1,
            "createAt": "2021-08-19T08:00:00.000Z",
            "updateAt": "2021-08-19T08:00:00.000Z",
            "children": [
              {
                "id": 38,
                "name": "查询用户",
                "type": 3,
                "url": null,
                "icon": null,
                "sort": 38,
                "parentId": 9,
                "createAt": "2021-08-19T08:00:00.000Z",
                "updateAt": "2021-08-19T08:00:00.000Z"
              }
            ]
          }
        ]
      }
    ]
  }
  ```

商品管理
--------

### 商品信息
- 创建商品 — `PATCH {{baseURL}}/goods`
  ```json
  {
    "name": "aaa",
    "oldPrice": 100,
    "newPrice": 88,
    "desc": "cba",
    "status": 1,
    "imgUrl": "www.itsiyuan.com/abc.png",
    "inventoryCount": 100,
    "saleCount": 100,
    "favorCount": 199,
    "address": "北京"
  }
  ```
- 删除商品 — `DELETE {{baseURL}}/goods/{id}`（示例 `282`）
- 修改商品 — `PATCH {{baseURL}}/goods/{id}`（示例 `1`）
  ```json
  {
    "name": "code",
    "newPrice": 90
  }
  ```
- 获取单个商品 — `GET {{baseURL}}/goods/{id}`（示例 `1`）
- 获取商品列表 — `POST {{baseURL}}/goods/list`
- 请求体：无（空对象）
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": {
      "list": [
        {
          "id": 1,
          "name": "aaa",
          "oldPrice": 100,
          "newPrice": 88,
          "status": 1,
          "imgUrl": "www.itsiyuan.com/abc.png",
          "inventoryCount": 100,
          "saleCount": 100,
          "favorCount": 199,
          "address": "北京",
          "createAt": "2021-08-19T08:00:00.000Z",
          "updateAt": "2021-08-19T08:00:00.000Z"
        }
      ],
      "totalCount": 1
    }
  }
  ```

### 商品分类
- 创建类别 — `POST {{baseURL}}/category`
  ```json
  {
    "name": "床上用品"
  }
  ```
- 删除类别 — `DELETE {{baseURL}}/category/{id}`（示例 `1`）
- 更新类别 — `PATCH {{baseURL}}/category/{id}`（示例 `1`）
  ```json
  {
    "name": "生活用品"
  }
  ```
- 获取单个类别 — `GET {{baseURL}}/category/{id}`（示例 `1`）
- 获取类别列表 — `POST {{baseURL}}/category/list`
  ```json
  {
    "offset": 0,
    "size": 3,
    "name": "子"
  }
  ```
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": {
      "list": [
        {
          "id": 1,
          "name": "床上用品",
          "parentId": null,
          "createAt": "2021-08-19T08:00:00.000Z",
          "updateAt": "2021-08-19T08:00:00.000Z"
        }
      ],
      "totalCount": 1
    }
  }
  ```

故事杂谈
--------

- 你的故事 — `POST {{baseURL}}/story`
  ```json
  {
    "title": "我与地坛",
    "content": "宇宙以其不息的欲望将一个歌舞炼为永恒。这欲望有怎样一个人间的姓名，大可忽略不计。"
  }
  ```
- 故事列表 — `POST {{baseURL}}/story/list`
- 请求体：无（空对象）
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": {
      "list": [
        {
          "id": 1,
          "title": "我与地坛",
          "content": "宇宙以其不息的欲望将一个歌舞炼为永恒。这欲望有怎样一个人间的姓名，大可忽略不计。",
          "createAt": "2021-08-19T08:00:00.000Z",
          "updateAt": "2021-08-19T08:00:00.000Z"
        }
      ],
      "totalCount": 1
    }
  }
  ```

高级查询
--------

- 查询完整菜单树 — `POST {{baseURL}}/menu/list`
- 请求体：无（空对象）
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": [
      {
        "id": 1,
        "name": "系统管理",
        "type": 1,
        "url": "/main/system",
        "icon": "el-icon-setting",
        "sort": 1,
        "parentId": null,
        "createAt": "2021-08-19T08:00:00.000Z",
        "updateAt": "2021-08-19T08:00:00.000Z",
        "children": [
          {
            "id": 9,
            "name": "用户管理",
            "type": 2,
            "url": "/main/system/user",
            "icon": "el-icon-user",
            "sort": 10,
            "parentId": 1,
            "createAt": "2021-08-19T08:00:00.000Z",
            "updateAt": "2021-08-19T08:00:00.000Z",
            "children": [
              {
                "id": 38,
                "name": "查询用户",
                "type": 3,
                "url": null,
                "icon": null,
                "sort": 38,
                "parentId": 9,
                "createAt": "2021-08-19T08:00:00.000Z",
                "updateAt": "2021-08-19T08:00:00.000Z"
              }
            ]
          }
        ]
      }
    ]
  }
  ```
- 查询角色菜单树 — `GET {{baseURL}}/role/{roleId}/menu`
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": [
      {
        "id": 1,
        "name": "系统管理",
        "type": 1,
        "url": "/main/system",
        "icon": "el-icon-setting",
        "sort": 1,
        "parentId": null,
        "children": [
          {
            "id": 9,
            "name": "用户管理",
            "type": 2,
            "url": "/main/system/user",
            "icon": "el-icon-user",
            "sort": 10,
            "parentId": 1,
            "children": [
              { "id": 38, "name": "查询用户", "type": 3, "permission": "system:users:list", "parentId": 9 }
            ]
          }
        ]
      }
    ]
  }
  ```
- 查询角色菜单 ids — `GET {{baseURL}}/role/{roleId}/menuIds`
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": {
      "menuIds": [1, 9, 38]
    }
  }
  ```
- 给角色分配权限 — `POST {{baseURL}}/role/assign`
  ```json
  {
    "roleId": 1,
    "menuList": [1, 9, 38, 41, 2, 3, 4, 25, 5, 6, 7, 8, 17, 18, 19, 20, 21, 22, 23, 24, 15, 16, 30, 31, 32, 33, 34, 35, 36, 37, 26, 27, 28, 29, 39, 40, 42, 43]
  }
  ```
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": {
      "menuIds": [1, 9, 38, 41],
      "tree": [
        {
          "id": 1,
          "name": "系统管理",
          "children": [
            { "id": 9, "name": "用户管理" }
          ]
        }
      ]
    }
  }
  ```

图表数据
--------

- 每个分类商品的个数 — `GET {{baseURL}}/goods/category/count`
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": [
      { "name": "数码", "value": 120 },
      { "name": "家居", "value": 80 }
    ]
  }
  ```
- 每个分类商品的销量 — `GET {{baseURL2}}/goods/category/sale`
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": [
      { "name": "数码", "value": 5600 },
      { "name": "家居", "value": 3200 }
    ]
  }
  ```
- 每个分类商品的收藏 — `GET {{baseURL2}}/goods/category/favor`
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": [
      { "name": "数码", "value": 230 },
      { "name": "家居", "value": 110 }
    ]
  }
  ```
- 销量前10的商品数量 — `GET {{baseURL2}}/goods/sale/top10`
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": [
      { "name": "MacBook Pro", "value": 120 },
      { "name": "iPhone 13", "value": 98 }
    ]
  }
  ```
- 不同城市的销量数据 — `GET {{baseURL2}}/goods/address/sale`
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": [
      { "address": "北京", "count": 320 },
      { "address": "上海", "count": 280 }
    ]
  }
  ```
- 商品数据统计的数量 — `GET {{baseURL}}/goods/amount/list`
- 请求参数：均为无 body，使用路径/查询参数
- 响应结构示例：
  ```json
  {
    "code": 200,
    "message": "成功",
    "data": [
      { "name": "数码", "value": 120 },
      { "name": "家居", "value": 80 }
    ]
  }
  ```

