# 贡献指南 / Contributing Guide

> 本文档固化 iv8-rs 项目的提交规范、代码风格约定。所有提交必须遵守。

## 一、提交规范（Commit Convention）

本项目采用 [Conventional Commits 1.0.0](https://www.conventionalcommits.org/) 规范。

### 1.1 格式

```
<type>[optional scope]: <subject>

[optional body]

[optional footer]
```

### 1.2 type 取值

| type       | 含义                                       | 示例                                   |
|------------|--------------------------------------------|----------------------------------------|
| `feat`     | 新功能                                     | `feat(dom): implement EventTarget`     |
| `fix`      | bug 修复                                   | `fix(crypto): correct PBKDF2 iteration` |
| `refactor` | 重构（不改功能、不修 bug）                  | `refactor(events): extract dispatch path` |
| `perf`     | 性能优化                                   | `perf(dom): cache id index`            |
| `test`     | 测试相关（新增/修改测试）                   | `test(compat): add 198 fixtures`       |
| `docs`     | 文档变更                                   | `docs: update PROGRESS for v0.1`       |
| `chore`    | 构建/工具/依赖等杂项                       | `chore: add .gitignore`                |
| `ci`       | CI 配置变更                                | `ci: add macOS arm64 wheel build`      |
| `build`    | 构建系统/外部依赖变更                      | `build: bump v8 to 147`                |
| `style`    | 代码风格调整（不改逻辑，仅格式化）         | `style: rustfmt all crates`            |
| `revert`   | 回滚某次提交                               | `revert: feat(dom): implement X`       |

### 1.3 scope 使用规则（策略 C）

**必须带 scope 的 type：**
- `feat` / `fix` / `refactor` / `perf` / `test`

**不带 scope 的 type（仅作用于项目根/全局配置时）：**
- `chore` / `docs` / `ci` / `build` / `style`

注意：`docs` 如果是模块级文档变更（如 `crates/iv8-core/README.md`），仍可加 scope：
- `docs(core): add module-level documentation`

### 1.4 scope 取值（固定列表）

| scope        | 范围                                     |
|--------------|------------------------------------------|
| `core`       | crates/iv8-core 通用层（kernel/state/error/convert/config 等）|
| `undetect`   | crates/iv8-undetect（反检测，含 native_env.rs）|
| `py`         | crates/iv8-py + python/iv8_rs            |
| `dom`        | crates/iv8-core/src/dom/                 |
| `events`     | crates/iv8-core/src/events/              |
| `crypto`     | crates/iv8-core/src/crypto/              |
| `canvas`     | crates/iv8-core/src/canvas/              |
| `network`    | crates/iv8-core/src/network/             |
| `inspector`  | crates/iv8-core/src/inspector/           |
| `shims`      | crates/iv8-core/src/shims/（含 navigator/document/storage 等）|
| `surface`    | crates/iv8-surface + tools/iv8-surface-codegen |
| `compat`     | tests/compat/ 差异测试                   |

如果一次提交跨多个 scope，选择**主要影响的 scope**，在 body 中说明其他变更。

### 1.5 subject 规则（标题行）

- **语言**：英文
- **长度**：≤ 72 字符（推荐 ≤ 50 字符）
- **时态**：祈使句现在时（"add"、"fix"，不是 "added"、"fixes"）
- **首字母**：小写
- **结尾**：不加句号
- **禁止 emoji**：标题行严禁出现 emoji

### 1.6 body 规则（详细描述）

- **语言**：中文为主，专业术语保留英文（如 EventTarget / NodeId / cppgc）
- **格式**：每行 ≤ 72 字符
- **内容三段式**：
  1. 实施内容：做了什么、实现思路、影响范围
  2. 本提交不授权：scope 边界、deferred 项
  3. 已执行审阅：验证命令及结果（test/check/clippy 通过数）
- **禁止 emoji**：body 中严禁出现 emoji（包括 `[OK]` 等替代标记仅在状态文档中使用，不在 commit message 中使用）
- **链接 REQ ID**：如果与某条需求相关，标注 `Refs: REQ-DOM-009` 或 `Closes: REQ-CORE-005`

### 1.7 footer 规则（可选）

- `Refs: REQ-DOM-009` — 引用需求
- `Closes: #issue` — 关闭 issue（远程仓库后使用）
- `BREAKING CHANGE: <description>` — 不兼容变更说明（必须大写）
- `Co-authored-by: <name> <email>` — 协作者

### 1.8 完整示例

#### 示例 1：feat 带 scope + 完整 body

```
feat(dom): implement EventTarget three-phase dispatch

实现 DOM 事件的三阶段派发模型（capture -> target -> bubble）：

- 添加 EventListenerRegistry，按 NodeId + event_type 索引监听器
- 实现 capture/bubble 阶段的路径遍历，path 跳过 target 节点
- 修复 Phase 1 错误包含 target 导致监听器触发两次的 bug
- once 选项支持：触发后自动移除监听器
- stopPropagation / preventDefault 通过 Cell 标志位传递

性能：单次 dispatch 平均 ~3 微秒（criterion bench）。

Refs: REQ-DOM-009, REQ-DOM-010
```

#### 示例 2：fix 简洁版

```
fix(dom): correct getElementById RefCell borrow lifetime

drop(doc) 释放的是 &Document 引用而非 Ref guard，导致后续
node_to_v8_object 调用时再次 borrow 产生 panic。修复为先在
borrow 块内获取 node_id，释放后再调用。

Refs: Task 102
```

#### 示例 3：chore 不带 scope

```
chore: configure workspace and build tooling

- Cargo workspace 三 crate 结构（iv8-core / iv8-undetect / iv8-py）
- pyproject.toml 配置 maturin 构建
- rust-toolchain.toml 锁定 Rust 1.80+
- rustfmt.toml 统一代码风格
- .gitignore 排除 .kiro/ / .venv/ / target/ 等
```

#### 示例 4：refactor 跨模块

```
refactor(core): unify safe_callback macro across all V8 callbacks

之前各模块的 V8 callback 都是手写 catch_unwind 包装，重复代码
多且容易遗漏。统一通过 safe_callback! macro 包裹：

- 所有 panic 转为 JS Error 抛回 V8
- tracing::error 记录 panic 详情
- profile panic = "unwind" 保证 catch_unwind 生效

影响 dom/events/crypto/canvas/network 全部回调函数。

Refs: REQ-CORE-013, REQ-ENG-004
```

---

## 二、代码风格规范

### 2.1 通用规则

- **禁止 emoji**：所有源代码、文档、commit message、注释中严禁出现 emoji
  - 状态标记使用文本：`[OK]` / `[FAIL]` / `[WARN]` / `[INFO]` / `[TODO]` / `[DONE]` / `[BLOCKED]`
  - 调研文档中的状态标记同样使用文本
- **行尾**：LF（Unix 风格），不使用 CRLF
- **缩进**：根据语言惯例（Rust/Python 4 空格，YAML/TOML 2 空格）
- **行宽**：建议 ≤ 100 字符，硬限制 120 字符

### 2.2 Rust

- 用 `cargo fmt` 自动格式化（rustfmt.toml 已配置）
- 用 `cargo clippy` 检查 lint
- 业务代码禁止 `unwrap()` / `expect()` / `panic!()`，例外用 `// SAFETY: <理由>` 注释豁免
- V8 callback 必须用 `safe_callback!` macro 包裹

### 2.3 Python

- PEP 8 风格
- type hints 全覆盖（公开 API）
- 测试用 pytest

### 2.4 Markdown

- 标题用 `#` 不用下划线
- 列表用 `-` 不用 `*`
- 表格列对齐
- 中英文混排时英文与中文之间加空格（`Rust 1.80` 不是 `Rust1.80`）

---

## 三、Git 工作流

### 3.1 分支策略（v0.1 阶段）

- 当前阶段（自用项目，单人开发）：直接在 `main` 分支开发
- 后续如有协作需求：feature branch + PR

### 3.2 提交频率

- **小步提交**：每个有意义的功能切片一个 commit
- **避免**：一次性提交几百个文件的"超级 commit"
- **避免**：commit message 写"WIP"、"update"、"fix things" 等无意义内容

### 3.3 每个 commit 的最低要求

- 编译通过（至少 `cargo check` 通过）
- 不破坏已通过的测试（除非 commit 本身就是修复测试）
- 提交前用 `cargo fmt` 格式化
- 提交前检查 `git status` 确认意图

### 3.4 修复历史的规则

- **未推送**的本地 commit 可以用 `git commit --amend` / `git rebase -i` 修改
- **已推送**的 commit 不要 force push（除非自用项目且明确知道后果）

### 3.5 本地里程碑 tag 规则

v0.8.12 之后的 `v0.8.x` tag 默认是 local milestone tag，而不是
package release tag，除非发布文档明确说明 package metadata 已同步。

- 里程碑 tag 必须使用 annotated tag。
- tag message 必须写明 `not a package release` 或等价说明。
- 不因里程碑 tag 修改 `Cargo.toml`、`pyproject.toml`、lock metadata。
- 不 push tag、不创建 GitHub release、不发布构建产物，除非用户显式授权。
- 更新 `docs/roadmap/post-v0.6/v0.8-release-and-tag-governance-closeout.md`
  后再打里程碑 tag。

---

## 四、开发流程

### 4.1 新功能开发流程

1. 在 `.kiro/specs/iv8-clone/tasks.md` 中找到对应任务（如 Task XX）
2. 实现功能，保证编译和测试通过
3. 写单元测试（Rust）/ pytest（Python）
4. 更新相关文档（`docs/PROGRESS.md` 等）
5. 提交：`feat(scope): description`，body 中 `Refs: Task XX`

### 4.2 Bug 修复流程

1. 写一个能复现 bug 的测试用例
2. 修复代码使测试通过
3. 提交：`fix(scope): description`，body 说明根因 + 修复方式

### 4.3 重构流程

1. 确保现有测试覆盖被重构的代码
2. 重构，保证测试仍通过
3. 提交：`refactor(scope): description`

---

## 五、强制检查清单（提交前必做）

```
[ ] 代码编译通过（cargo check / cargo build）
[ ] 测试全部通过（cargo test / pytest）
[ ] 代码已格式化（cargo fmt）
[ ] clippy 无新增 warning（cargo clippy）
[ ] 没有 emoji（源码、文档、commit message）
[ ] commit message 符合 Conventional Commits 规范
[ ] subject 行 <= 72 字符，全英文
[ ] body 符合中英混排约定
[ ] git status 清洁（无意外文件）
```

---

## 六、Release Commits（发布相关特殊 commit）

发布流程中有两种特殊 commit，不属于日常开发但必须遵守规范：

### 6.1 Release 准备 commit

```
chore: prepare vX.Y.Z release

更新 CHANGELOG.md 归档 [Unreleased] -> [X.Y.Z]。
更新 docs/PROGRESS.md 时间戳与最终状态。
```

- type: `chore`，不带 scope（项目根级操作）
- 此 commit 之后立即打 tag

### 6.2 Version bump commit

```
chore: bump version to X.Y.Z-dev

Cargo.toml workspace.package.version: "X.Y.Z" -> "X.(Y+1).0-dev"
开始下一个开发周期。
```

- type: `chore`，不带 scope
- 此 commit 在 tag 之后

### 6.3 Release tag message

Tag message 不是 commit message，但遵循相同审美：
- subject: `release: vX.Y.Z -- <英文短描述>`
- body: 中文摘要 + 英文 highlights + known limitations + stats
- 详见 RELEASING.md §4

---

## 七、Git 操作安全（D-094）

以下操作为**破坏性操作**，未经用户明确确认不得执行：

| 操作 | 风险 |
|------|------|
| `git reset --hard` | 丢弃未提交改动且移动分支指针，丢失提交 |
| `git rebase` | 重写历史，可能静默丢弃提交 |
| `git checkout -- .` / `git restore .` | 丢弃工作区所有改动 |

### 规则

1. 实验性代码改动用 `git stash`（可恢复）或 `git checkout -- <具体文件>`（窄范围），**不用** `git reset --hard`。
2. 如需 `git reset` 或 `git rebase`，须向用户说明确切命令和原因，等待确认。
3. 任何历史重写操作后，立即运行 `git reflog -10` 和 `cargo test --workspace` 验证提交和改动未丢失。
4. rebase 出问题时 `git rebase --abort` 是安全的，但仍需 `git reflog` 验证。

> 事故记录：v0.8.78 会话中 `git reset` 将分支指针从 v0.8.77 tag 移回 v0.8.76 tag，丢失 39 个提交。通过 `git reset --hard 64e52a3` 恢复。

---

## 八、参考

- [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/)
- [Semantic Versioning 2.0.0](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- 项目章程：`.kiro/steering/iv8-clone-charter.md`
