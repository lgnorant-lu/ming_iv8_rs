# Quality Harness Charter (元规范)

> 本文档是 iv8-rs 所有"质量 harness"的元规范（宪法）。
> 凡是特定领域、有明确质量需求的场景，都按本宪法建立专属 harness，
> 围绕其规范推进开发。
> 第一个实例：H01 Crypto Detection（见 `H01-crypto-detection.md`）。
> 最后更新：2026-06-01

---

## 0. 什么是 Quality Harness

Quality Harness（质量评估框架）= **可计算的质量门禁**。
它把某个特定领域的"是否达标"从主观判断变成**工具算出的客观数字 + 预设阈值的对照**。

行业对位：SonarSource Quality Gate、EleutherAI lm-evaluation-harness、
Definition of Done。本宪法是这些成熟实践在 iv8-rs 的本地化落地。

**它解决的问题**（来自真实教训）：
开发反复陷入"声称完成 → 实测崩溃 → 打补丁 → 再声称完成"的环路。
根因不是某条实现错，而是**缺乏客观、量化、带阈值的评估规范和强制工作流**——
"100%" 是嘴说的，不是工具算的；只测证实不测证伪。

---

## 1. 十条核心原则（所有 harness 必须遵守）

| # | 原则 | 含义 | 反面教训 |
|---|---|---|---|
| P1 | **可计算性** | 每个 PASS/FAIL 由代码算出，不接受人工声称 | 嘴说"100% 覆盖"→ 一测崩 |
| P2 | **证伪优先** | 必须有反例/负向测试，不只正向 | 只测"喂 crypto 能检测"，漏测"喂普通数据不误报"→ 1250 个假阳性 |
| P3 | **阈值显式化** | 阈值集中一处（THRESHOLDS），改阈值需在 commit 写理由 | 隐式标准 = 无标准 |
| P4 | **指标分类全过** | 指标分类别，**全类别 PASS 才算达标** | 单点验证给虚假安全感 |
| P5 | **确定性** | 固定随机种子，同输入同输出 | 不可复现 = 不可信 |
| P6 | **自审性** | harness 自身可能有 bug，必须可审计、可暴露 | SHA-384 substring 误配导致 harness 自己误报 |
| P7 | **单命令单退出码** | `python harness.py` → exit 0/1，CI 可挂 | 散落脚本无法自动化门禁 |
| P8 | **诚实局限** | 显式记录未覆盖部分，不装作全覆盖 | 假装全覆盖 = 埋雷 |
| P9 | **门禁绑定** | 通过 steering 强制加载，非自觉遵守 | 靠自觉 = 必然遗忘 |
| P10 | **金标准数据** | 测试数据来自权威来源，可追溯 | 自编数据 = 自欺 |

---

## 2. 每个 Harness 的标准三件套

一个完整的 harness 实例必须有且仅有这三个组成部分：

| 组件 | 形式 | 职责 | 是否提交 git |
|---|---|---|---|
| **实现** (impl) | `scripts/evaluate_<domain>.py` | orchestrator，跑全部类别，输出 scorecard + 退出码 | 是 |
| **规范** (spec) | `docs/quality-harness/H<NN>-<domain>.md` | 指标定义、阈值、金标准来源、工作流、已知局限、当前基线 | 是 |
| **门禁** (gate) | `.kiro/steering/<domain>-quality-gate.md` | 文件触发自动加载的强制工作流条款 | 本地（.kiro 被 gitignore） |

实现脚本可调用若干子验证脚本，但**对外只暴露一个 orchestrator 入口**（P7）。

---

## 3. 评估类别模板（A–E）

新 harness 按需选用以下类别，但 **A、C 为强制**（数据正确 + 不误报是底线）：

| 类别 | 名称 | 回答的问题 | 典型指标 | 强制 |
|---|---|---|---|---|
| A | Data Integrity | 基础数据对不对？ | error 数 = 0 | **是** |
| B | Recall / Capability | 该做的能做到吗？ | recall_pct | 视领域 |
| C | False Positive / Safety | 不该触发的不会触发吗？ | fp_samples = 0 | **是** |
| D | Coverage | 测试覆盖全吗？ | coverage_pct + 负向守护数 | 视领域 |
| E | Robustness | 扰动下还稳吗？ | determinism + noise tolerance | 视领域 |

每个类别必须有：明确指标定义 + 数值阈值 + 程序化生成的样本（不手工挑选）。

---

## 4. Harness 生命周期（Define → Test → Diagnose → Fix）

```
1. Define   : 在 spec 中定义类别、指标、阈值、金标准来源
2. Implement: 写 evaluate_<domain>.py，THRESHOLDS 集中定义
3. Test     : 跑 harness，得到客观 scorecard
4. Diagnose : 若 FAIL，定位是「被测对象缺陷」还是「harness 自身 bug」(P6)
5. Fix      : 修到 PASS。禁止靠降阈值变绿 (P3)
6. Gate     : 写 steering 门禁，绑定文件触发
7. Baseline : 在 spec 记录当前基线数字 + 日期
```

任何后续改动相关文件 → 回到步骤 3 重跑，基线必须维持 PASS。

---

## 5. 命名与编号约定

- harness 实例编号：`H01`、`H02`...（按创建顺序，永不复用）
- spec 文件：`docs/quality-harness/H<NN>-<domain>.md`
- impl 脚本：`scripts/evaluate_<domain>.py`
- gate 文件：`.kiro/steering/<domain>-quality-gate.md`
- 退出码约定：`0` = 全 PASS；非 `0` = 至少一类 FAIL

---

## 6. 硬性工作流（对所有 harness 生效）

1. **禁止口头声称"通过/完成/100%覆盖"**。唯一合法证据 = 对应
   `evaluate_<domain>.py` 打印 `OVERALL: PASS` 且退出码 0。引用工具数字。
2. 改动 harness 覆盖范围内的文件后，**必须重跑 harness 并贴 scorecard**。
3. **任一类别 FAIL → 禁止 commit、禁止声称完成**。
4. **改阈值必须在 commit body 写明理由**，不得为变绿而降阈值。
5. **harness 自身 bug 也是缺陷**：误报 FAIL 先修 harness，不得绕过判定 (P6)。
6. 新增被测能力时，同一改动内补齐：实现 + 测试 + 重跑 harness。

---

## 7. Harness 注册表

| 编号 | 领域 | spec | impl | gate | 状态 |
|---|---|---|---|---|---|
| H01 | Crypto Detection | `H01-crypto-detection.md` | `scripts/evaluate_detection.py` | `crypto-detection-quality-gate.md` | 基线 PASS |
| H02 | Env Consistency | `H02-env-consistency.md` | `scripts/evaluate_env_consistency.py` | `env-consistency-quality-gate.md` | candidate (Phase 4 合并入 H04) |
| H03 | Surface Accuracy | `H03-surface-accuracy.md` | `scripts/evaluate_surface_accuracy.py` | `surface-accuracy-quality-gate.md` | candidate (Phase 4 合并入 H04) |
| H04 | Surface Integrity Matrix | `H04-surface-integrity-matrix.md` | `scripts/evaluate_surface_integrity.py` | `surface-integrity-quality-gate.md` | candidate (Phase 1 待实施) |
| H05 | Getter Return Value Audit | `H05-getter-return-value-audit.md` | `scripts/evaluate_h05_getter.py` (H05a), `scripts/evaluate_h05b_setter.py` (H05b), `scripts/evaluate_h05c_method.py` (H05c), `scripts/evaluate_h05d_constructor.py` (H05d), `scripts/evaluate_h05e_exception.py` (H05e), `scripts/evaluate_h05f_tostring.py` (H05f) | (pending) | candidate (H05a 1012/1063, H05b 49/50, H05c 59/60, H05d 35/35, H05e 17/37+20, H05f 1270/1284 OVERALL PASS) |
| H06 | Cross-Context Consistency | `H06-cross-context-consistency.md` | `scripts/evaluate_h06_window_iframe.py` | (pending) | candidate (H06a 43/43 PASS, H06b deferred) |

> H04 是 H02/H03 的超集（D-102），Phase 4 完成后 H02/H03 标记 DEPRECATED。

> 后续如有新需求场景（如：环境指纹一致性、TDC 字段正确率、trace 解析鲁棒性），
> 评估其是否够格立 harness，够格则按本宪法新增 H02、H03...，并登记于此表。

---

## 8. 何时该立一个新 Harness（准入判断）

不是所有东西都要 harness。满足以下**全部**条件才立：

- 该领域有**明确的对错标准**（可量化，非品味问题）
- 该领域**反复出错或回归风险高**（值得固化门禁）
- 存在**可程序化生成的正例与反例**
- 有**权威金标准来源**可对照（P10）

否则用普通单元测试即可，不必动用 harness 这套重型机制。
