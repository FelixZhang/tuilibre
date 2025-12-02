# tuilibre - 数字书籍管理TUI工具 需求文档

## 项目概述

tuilibre是一个基于Rust开发的终端用户界面(TUI)工具，用于高效管理calibre数字图书馆。

**核心目标**
- 统一管理多个calibre数字图书馆
- 提供快速的键盘驱动操作体验
- 支持智能搜索和元数据管理
- 轻量级、跨平台的解决方案

## 核心功能

### 基础功能
- **多库管理**: 连接和切换多个calibre数据库
- **书籍浏览**: 列表和详情视图显示书籍信息
- **搜索过滤**: 支持标题、作者、标签等多字段搜索
- **元数据编辑**: 修改书籍基本信息和标签

### 高级功能
- **阅读管理**: 阅读状态跟踪和进度记录
- **批量操作**: 同时编辑多本书籍
- **数据导入导出**: 支持CSV/JSON格式
- **界面自定义**: 主题和快捷键配置

## 技术规格

### 技术栈
- **语言**: Rust (edition 2021)
- **UI**: ratatui TUI框架
- **数据库**: SQLite (rusqlite)
- **配置**: serde + toml

### 架构设计
```
UI Layer (TUI)
    ↓
Business Logic Layer
    ↓
Data Access Layer
    ↓
Database Layer (SQLite)
```

### 核心数据模型
```rust
pub struct Book {
    pub id: i32,
    pub title: String,
    pub authors: Vec<Author>,
    pub tags: Vec<Tag>,
    pub path: String,
    pub uuid: String,
    pub has_cover: bool,
    // ... 其他calibre字段
}

pub struct Library {
    pub name: String,
    pub path: PathBuf,
    pub book_count: i32,
}
```

## 开发计划

### 第一阶段 - MVP (4-6周)
- 基础项目结构和TUI框架
- 单库连接和书籍列表显示
- 基础键盘导航和搜索功能
- 书籍详情查看

### 第二阶段 - 功能增强 (6-8周)
- 多库管理和切换
- 高级搜索和过滤
- 元数据编辑功能
- 批量操作支持

### 第三阶段 - 完善优化 (4-6周)
- 阅读进度管理
- 数据导入导出
- 性能优化和测试
- 跨平台打包发布

## 性能要求
- UI响应时间 < 100ms
- 10万本书搜索 < 500ms
- 初始库加载 < 2秒
- 基础内存使用 < 50MB

## 约束条件

### 技术约束
- 必须使用Rust + TUI
- 兼容calibre 5.0+ SQLite数据库
- 支持Linux/macOS/Windows

### 用户约束
- 全键盘操作设计
- 10分钟内掌握基础操作
- 不损坏原始calibre数据
- 支持中文界面和书籍