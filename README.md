# Memory Blog (Rust Rewrite)

这是一个基于 **Rust** 重写的高性能博客后端系统，完全替代了原有的 Java Spring Boot 版本。本项目利用 Rust 的内存安全和并发特性，在保持与原有 **React 前端代码 (Admin Dashboard)** 100% 接口兼容的前提下，极大降低了资源占用。

## 🛠️ 技术栈 (Tech Stack)

| 组件 | 选型 | 说明 |
|------|------|------|
| **语言** | Rust (2021 Edition) | 系统级编程语言，零开销抽象 |
| **Web 框架** | [Axum](https://github.com/tokio-rs/axum) | 基于 Tokio 的现代 Web 框架 |
| **ORM** | [SeaORM](https://www.sea-ql.org/SeaORM/) | 异步动态 ORM，支持 MySQL |
| **运行时** | [Tokio](https://tokio.rs/) | Rust 下最流行的异步运行时 |
| **数据库** | MySQL 8.0 | 兼容原版 Memory Blog 数据结构，并进行了扩展 |
| **序列化** | Serde | 高效的 JSON 处理 |

## 📂 项目结构 (Structure)

```text
/opt/memory_blog_rust/
├── Cargo.toml          # 项目依赖管理
├── .env                # 环境变量配置 (数据库连接)
├── src/
│   ├── main.rs         # 程序入口，服务器配置
│   ├── utils.rs        # 通用工具 (统一 API 响应格式 ApiResponse)
│   ├── entity/         # 数据实体层 (扩充了字段以匹配前端 Interface)
│   │   ├── user.rs     # 用户
│   │   ├── note.rs     # 文章 (含 cover, status, description 等字段)
│   │   ├── category.rs # 分类 (含 icon, color, noteCount 等字段)
│   │   ├── tag_one.rs  # 一级标签
│   │   ├── tag_two.rs  # 二级标签 (嵌套结构)
│   │   ├── friend.rs   # 友链
│   │   └── ...
│   └── routes/         # API 路由与控制器 (Controller)
│       ├── notes.rs    # 文章管理 (支持批量删除、搜索、Top/Status 更新)
│       ├── categories.rs # 分类管理 (支持 Note 计数)
│       ├── tags.rs     # 标签管理 (树形结构组装)
│       ├── web_info.rs # 站点/个人/社交信息管理
│       └── mod.rs      # 路由注册中心
```

## 🚀 快速开始 (Getting Started)

### 1. 环境准备
确保已安装 [Rust](https://www.rust-lang.org/tools/install) 和连接可用的 MySQL 数据库。

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2.配置环境变量
在项目根目录创建 `.env` 文件：

```ini
# .env
DATABASE_URL=mysql://root:123456@localhost/memory_blog
RUST_LOG=debug
```
*请将 `root:123456` 替换为实际的数据库账号密码，`memory_blog` 替换为实际库名。*
*注意：Rust 版后端扩展了原有数据表字段（如 `note_tags`, `color`, `icon` 等），请确保数据库 Schema 已更新。*

### 3. 运行项目

开发模式：
```bash
cargo run
```

生产构建：
```bash
cargo build --release
./target/release/memory_blog_rust
```

服务器默认监听端口：`3000`

## 💻 前端项目 (Frontend)

项目包含一个配套的 **React Admin Dashboard** 前端，源码位于 `frontend`。

### 1. 技术栈
- **框架**: React 18 + Vite 5
- **UI 组件库**: Ant Design 5 + Material UI
- **数据状态**: Redux Toolkit
- **Markdown**: Bytemd

### 2. 运行前端
请确保后端服务已运行在 `3000` 端口。

```bash
cd frontend
npm install
npm run dev
```
前端默认运行在 `http://localhost:5173`，并已配置 Proxy 代理除 `/api` 开头的请求到 `http://localhost:3000`。

### 3. 构建部署
```bash
npm run build
```
构建产物位于 `dist/` 目录，可直接部署至 Nginx 或集成到 Rust Axum 的静态文件服务中。

## 🔌 API 接口全览 (API Endpoints)

本系统已针对 React 前端 `src/apis/*.tsx` 中的调用进行了全量适配。所有接口均返回统一格式：`{ code: 200, message: "...", data: ... }`。

### 🔓 公开接口 (Public)
| 模块 | 方法 | 路径 | 描述 |
|------|------|------|------|
| **Auth** | `POST` | `/api/login` | 管理员登录 |
| **Note** | `GET` | `/api/public/notes` | 获取文章列表 (含分类/标签信息) |
| **Note** | `POST` | `/api/public/notes/search` | 全文搜索 |
| **Note** | `GET` | `/api/public/notes/:id` | 文章详情 |
| **Cat** | `GET` | `/api/category`, `/api/public/category` | 分类列表 (含文章计数) |
| **Tag** | `GET` | `/api/tagone` | 一级标签列表 |
| **Tag** | `GET` | `/api/tagtwo` | 二级标签列表 (扁平化返回) |
| **Friend** | `GET` | `/api/friends` | 友链列表 |
| **Talk** | `GET` | `/api/talk` | 说说列表 |
| **User** | `GET` | `/api/public/user` | 全局管理员信息 (Avatar, Talk, BlogTitle) |
| **Social** | `GET` | `/api/public/social` | 社交媒体链接 |

### 🔒 管理接口 (Protected) - 需鉴权
| 模块 | 方法 | 路径 | 描述 | 前端对应方法 |
|------|------|------|------|--------------|
| **Note** | `POST` | `/api/protected/notes` | 创建文章 | `createNote` |
| **Note** | `POST` | `/api/protected/notes/:id` | 更新文章 | `updateNote` |
| **Note** | `DELETE` | `/api/protected/notes` | **批量**删除文章 (Body: `[id1, id2]`) | `delNote` / `delAllNotes` |
| **Cat** | `POST` | `/api/protected/category` | 创建分类 | `addCategory` |
| **Cat** | `POST` | `/api/protected/category/:id` | 更新分类 | `updateCategory` |
| **Cat** | `DELETE` | `/api/protected/category` | **批量**删除分类 (Body: `[id...]`) | `delCategory` |
| **Tag** | `POST` | `/api/protected/tagone` | 创建一级标签 | `addTagOne` |
| **Tag** | `POST` | `/api/protected/tagtwo` | 创建二级标签 | `addTagTwo` |
| **Tag** | `DELETE` | `/api/protected/tag` | **批量**删除标签 | `delTag` |
| **Talk** | `POST` | `/api/protected/talk` | 发布说说 | `addTalk` |
| **Web** | `PUT` | `/api/protected/social` | 更新社交信息 | `updateSocial` |

## ✅ 开发进度与适配说明

- [x] **字段对齐**: 已修正 `note_categories` -> `category_id`, `created_at` 格式, `isTop`, `cover` 等由于前后端命名不一致导致的问题。
- [x] **批量操作**: 删除接口已升级为接收 JSON 数组，支持前端的批量选择删除功能。
- [x] **数据聚合**: 分类列表 API 自动计算关联的文章数量 (`noteCount`)。
- [x] **路由兼容**: 同时兼容 `/api/public/...` 和 `/api/...` 等遗留路径别名。

## ⚠️ 迁移注意
如果使用的是旧版 SpringBoot 的数据库，请务必执行 SQL 脚本添加 Rust 版所需的新列（如 `icon`, `color`, `path_name` 等），否则 API 可能会报错。
