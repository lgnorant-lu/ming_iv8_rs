# Docs 目录结构规范

> Created: 2026-06-18
> Status: accepted
> Scope: `docs/` 下所有目录和文件的组织规则

## 目录结构

```
docs/
  README.md                ← 文档总入口 (Quick Start + 目录地图)
  GUIDE.md                 ← 用户 API 指南
  PROGRESS.md              ← 里程碑进度表

  conventions/             ← 项目级规范文档
    README.md              ← 规范索引 + 层级
    naming-conventions.md  ← 命名规范 (模块/类/函数/测试/文档)
    testing-conventions.md ← 测试规范 (层/断言/harness/覆盖)
    docs-conventions.md    ← 本文档 (docs/ 自身结构规范)

  roadmap/                 ← 路线规划 (按版本组织)
    v0.8/                  ← v0.8 阶段所有文档
      shared/              ← 跨轨道: 能力矩阵/执行阶段/版本边界/决策登记/执行协议/治理
      environment/         ← 补環境轨道 (v0.8.0-v0.8.16)
      native-substrate/    ← 原生基板轨道 (v0.8.17-v0.8.53)
      bundles/             ← 多打包轨道 (v0.8.52-v0.8.53)
      analysis/            ← 分析/反混淆轨道 (capability specs)
    v0.9/                  ← v0.9 阶段 (future)

  specs/                   ← 能力规范 (跨版本复用)
    deobfuscation-pipeline.md
    vm-analysis-pipeline.md
    dispatch-generalization.md
    ...

  todo/                    ← 跨模块 TODO 追踪
    README.md
    TODO-META.md
    TODO-*.md

  quality-harness/         ← 质量门禁
    HARNESS-CHARTER.md
    H01-crypto-detection.md

  _legacy/                 ← 历史归档 (只读)
    README.md
    early-design/
    early-research/
    root-superseded/
    adr/                   ← 已解决 ADR
    archive/               ← v0.6 审计
    baseline/              ← v0.6 基准
```

## 文件命名规范

### 版本文档

```
v<MAJOR>.<MINOR>.<PATCH>-<role>.md

例:
  v0.8.52-scope.md              版本 scope 定义
  v0.8.52-foundation-audit.md   前置审计
  v0.8.52-negative-gate-plan.md 负向门禁
  v0.8.52-implementation-task-plan.md 实现任务
  v0.8.52-acceptance.md         验收标准

Capability specs:
  v0.8.2-cff-detection-spec.md  绑定单一版本的能力规范
  v0.7-dispatch-generalization.md
```

### 跨版本规范

```
<capability>-spec.md           不绑定特定版本的通用规范

例:
  deobfuscation-pipeline.md     跨 v0.8.0-v0.8.2
  vm-analysis-pipeline.md       跨 v0.8.0-v0.8.2
  api-contract.md               跨 v0.6.1+
```

### 永久档/跨周期

```
保留原名, 不绑定版本号:

  naming-conventions.md         项目级规范, 不随版本迁移
  RESEARCH_REFERENCES.md         参考资料索引
  CAPABILITY_INDEX.md            能力矩阵
  EXECUTION_ORDER.md             执行阶段
  VERSION_SCOPE_MAP.md           版本边界
```

## 活跃 vs 旧物

| 状态 | 目录 | 规则 |
|---|---|---|
| **活跃** | `conventions/`, `todo/`, `roadmap/v0.8/`, `quality-harness/`, `specs/` | 可修改、更新 |
| **退役** | `_legacy/` | **只读** — 不再修改，仅保留历史参考 |
| **旧物归档** | 完成使命的文档 → `_legacy/` 对应子目录 | 不删除，移入归档 |

归档条件：
- 对应的版本阶段已完成并 tagged
- 内容已被更新版本替代
- 6 个月以上无引用

## 交叉引用规范

```markdown
项目内引用使用 repo-relative 路径:

[PROGRESS.md](docs/PROGRESS.md)
[todo/README.md](docs/todo/README.md)
```

不依赖外部链接（`docs/design/` → 已移入 `_legacy/early-design/`）。

## 新建文件 Checklist

- [ ] 文件名符合命名规范
- [ ] 放对目录 (版本 doc → roadmap/v0.X/, 规范 → specs/ 或 conventions/)
- [ ] 头部元数据块完整 (Created / Status / Scope / Parent)
- [ ] 在对应 README.md 中添加条目
- [ ] 交叉引用使用正确路径
