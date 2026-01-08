# Saudade Blog (Refactored)

<p align="center"><strong>一个风格可爱的个人主题博客 (Rust 重构版)</strong></p>

<p align="center">本仓库为 <strong>Saudade Blog</strong> 的全栈仓库。原 Memory Blog 前端经过适配，后端已使用 <strong>Rust (Axum + SeaORM)</strong> 完全重构，提供更高性能与更轻量的部署体验。</p>

## :sparkles: 项目特性

- **高性能后端**: 使用 Rust (Axum) 重写，极低的内存占用与极快的响应速度。
- **现代化 ORM**: 集成 SeaORM，类型安全，支持 MySQL/PostgreSQL/SQLite。
- **清新前端**: 基于 React 18 + Vite + Sass 的可爱风格 UI。
- **全栈类型安全**: 前后端分离，结构清晰。

## 📂 项目结构

```text
Saudade-Blog/
|-- src/                # 前端 React 源代码
|   |-- components      # 组件库 (ChatBox, etc.)
|   |-- pages           # 页面试图
|   |-- store           # Redux 状态管理
|   |-- assets          # 静态资源
|-- src-tauri/          # (可选) Tauri 桌面端配置
|-- Cargo.toml          # Rust 后端依赖配置
|-- src/main.rs         # (假设) Rust 后端入口
|-- entity/             # SeaORM 实体定义
|-- migration/          # 数据库迁移文件
|-- package.json        # 前端依赖配置
|-- README.md           # 说明文档
```

## :wrench: 技术栈

### Frontend
- **Framework**: React 18, TypeScript
- **Build Tool**: Vite 5
- **Styling**: Sass, Framer Motion
- **State**: Redux Toolkit
- **Network**: Axios

### Backend (New!)
- **Language**: Rust
- **Web Framework**: Axum 0.7
- **ORM**: SeaORM (Async & Dynamic)
- **Database**: MySQL / PostgreSQL
- **Runtime**: Tokio

## 🚀 快速开始

### 1. 环境准备
确保已安装：
- Node.js >= 18
- Rust (Cargo)
- MySQL 或 PostgreSQL 数据库实例

### 2. 后端启动 (Rust)

配置数据库链接 (在 `.env` 或环境变量中):
```bash
DATABASE_URL=mysql://user:password@localhost:3306/saudade_db
```

运行后端:
```bash
cargo run --release
```

### 3. 前端启动

```bash
# 安装依赖
npm install

# 开发模式
npm run dev

# 构建生产环境代码
npm run build
```

前端配置位于 `.env`:
```properties
VITE_HTTP_BASEURL = 'http://127.0.0.1:3000'  # 指向 Rust 后端端口
```

## ☀️ Docker 部署

构建并运行完整的 Saudade 服务：

```bash
# 示例构建命令 (需配合 Dockerfile)
docker build -t saudade-blog .
docker run -d -p 80:80 -e DATABASE_URL=... saudade-blog
```

## :heart: 鸣谢

- 原作设计: [Memory-Blog](https://github.com/LinMoQC/Memory-Blog)
- 维护者: Saudade Team
