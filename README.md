# shishixianshi

一个用 `Tauri 2 + TypeScript + Rust` 搭的极简桌面浮窗原型，用来在 macOS 和 Windows 右上角持续显示 `Claude` 与 `Codex` 的剩余额度。

## 当前状态

当前仓库已经包含：

- 极简 HUD 浮窗前端
- 顶部右侧自动定位逻辑
- 托盘菜单骨架
- Rust 侧 mock 数据接口
- 适合后续替换成真实 provider 的目录结构

当前仓库还没有完成：

- Claude 真实额度接入
- Codex 真实额度接入
- Keychain / Credential Manager 凭据管理
- 开机启动

## 技术栈

- `Tauri 2`
- `TypeScript`
- `Vite`
- `Rust`

## 本地开发

### 1. 安装前端依赖

```bash
npm install
```

### 2. 安装 Rust

按 Tauri 官方前置要求，macOS / Linux 可使用：

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Windows 可用：

```powershell
winget install --id Rustlang.Rustup
```

### 3. 开发运行

```bash
npm run tauri dev
```

如果你只是先看前端样式，也可以：

```bash
npm run dev
```

浏览器模式下会自动退回本地 mock 数据。

## 计划中的真实数据源

- `Claude`: 复用本机授权状态，优先走已有 CLI / OAuth 登录态
- `Codex`: 复用本机 `codex login` 后的本地授权状态

## 目录

```text
.
├── index.html
├── package.json
├── src/
│   ├── main.ts
│   ├── overlay/
│   │   ├── OverlayApp.ts
│   │   ├── overlay.css
│   │   └── types.ts
│   └── providers/
│       ├── bridge.ts
│       ├── mock.ts
│       └── provider.ts
└── src-tauri/
    ├── Cargo.toml
    ├── build.rs
    ├── tauri.conf.json
    ├── capabilities/
    │   └── default.json
    └── src/
        ├── commands.rs
        ├── lib.rs
        ├── main.rs
        └── providers/
            ├── mock.rs
            ├── mod.rs
            └── types.rs
```
