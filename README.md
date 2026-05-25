# GH-Terminal

国赫电台设备控制终端（Tauri + Svelte），通过串口与电台设备通信，并在桌面端提供参数/状态/表头等信息展示与控制。

## 目录结构

- `gh-terminal/`：Tauri（Rust）桌面端主程序、串口连接与命令调用
- `frontend/`：Svelte + Vite 前端界面
- `gh-protocol/`：设备通信协议编解码与命令定义（Rust crate）
- `doc/`：协议说明文档

## 环境要求

- Node.js（建议 18+ / 20+）
- Rust（edition 2021，对应 stable 工具链）
- Tauri CLI v2：`cargo install tauri-cli`
- Windows：需要 MSVC Build Tools（含 C++ 构建工具），并确保 WebView2 可用

## 开发运行（推荐）

1) 安装前端依赖：

```bash
cd frontend
npm install
```

2) 启动 Tauri 开发模式（会自动启动前端 dev server）：

```bash
cd gh-terminal
cargo tauri dev
```

## 构建打包

```bash
cd gh-terminal
cargo tauri build
```

产物位置与平台相关，Tauri 会在构建输出中提示具体路径。

## 协议文档

- `doc/国赫电台设备控制协议.md`

> 说明：如果你在终端里看到中文乱码，通常是控制台编码（Code Page）或文件编码显示方式导致；在编辑器（VS Code 等）中打开会更直观。

## 常见问题

- 串口占用：请先关闭其它占用串口的软件（串口调试助手等）。
- Windows 串口权限/驱动：确认设备驱动已安装、设备管理器中可见对应 COM 口。
- 运行日志：Rust 端使用 `tracing`，默认 INFO 级别（见 `gh-terminal/src/main.rs`）。
