# tuilibre (图书管理器)

English Documentation | [中文文档 / Chinese Documentation](README-zh.md)

一个用于管理 calibre 数字图书库的 TUI（终端用户界面）工具。

## 描述

tuilibre 是一个快速、键盘驱动的终端应用程序，用于浏览和管理 calibre 数字图书库。它提供了一个简单而强大的界面，用于搜索、查看和组织您的数字图书收藏。

## 功能特性（MVP）

- 连接 calibre 数据库
- 键盘导航浏览图书列表
- 查看详细图书信息
- 简单搜索功能
- 跨平台支持（Linux、macOS、Windows）

## 系统要求

- Rust 1.70+
- SQLite（用于 calibre 数据库访问）

## 安装

### 从源代码构建

```bash
git clone https://github.com/yourusername/tuilibre.git
cd tuilibre
cargo build --release
```

### 使用方法

使用您的 calibre 图书馆路径运行 tuilibre：

```bash
# 使用当前目录
tuilibre

# 指定图书馆路径
tuilibre /path/to/calibre/library
```

## 控制键

### 普通模式
- `Enter` 或 `→`：查看图书详情
- `Esc` 或 `←`：返回图书馆选择
- `↑/↓` 或 `j/k`：导航图书列表
- `/`：进入搜索模式
- `q`：退出应用程序

### 搜索模式
- `Enter` 或 `→`：执行搜索并查看结果
- `Esc` 或 `←`：返回普通模式（清除搜索）
- 字符键：输入搜索查询
- `Backspace`：删除最后一个字符

### 详情模式
- `Enter` 或 `→`：使用系统默认应用程序打开图书
- `Esc` 或 `←`：返回上一模式
- `q`：退出应用程序

### 图书馆选择模式
- `Enter` 或 `→`：选择图书馆
- `Esc` 或 `←`：退出应用程序
- `↑/↓` 或 `j/k`：导航图书馆列表
- `/`：进入图书馆搜索模式



## 许可证

本项目使用 MIT 许可证授权 ([LICENSE-MIT](LICENSE-MIT) 或 http://opensource.org/licenses/MIT)。

## 贡献

欢迎贡献！请随时提交 Pull Request。

## 免责声明

**AI 辅助**: 本软件在人工智能辅助下编写。虽然使用了AI工具来加速开发，但最终代码已经过人工开发者审查、测试和批准。

**无担保**: 本软件"按现状"提供，不提供任何形式的担保，无论是明示还是暗示的，包括但不限于对适销性、特定用途适用性和非侵权性的担保。