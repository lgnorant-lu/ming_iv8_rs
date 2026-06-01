# H01 — Crypto Detection Quality Harness

> 第一个 quality harness 实例。遵循 `HARNESS-CHARTER.md` 元规范。
> 所有"达标"判断由 `scripts/evaluate_detection.py` 计算，对照下方阈值，
> **不接受任何主观"100%"声称**（宪法 P1）。
> 最后更新：2026-06-01

| 字段 | 值 |
|---|---|
| 编号 | H01 |
| 领域 | Crypto algorithm detection（trace 后处理识别加密算法） |
| 实现 | `scripts/evaluate_detection.py` |
| 门禁 | `.kiro/steering/crypto-detection-quality-gate.md` |
| 被测模块 | `python/iv8_rs/patterns.py`, `trace.py`, `crypto_*.json` |

---

## 1. 背景与动机

开发过程反复出现"声称完成 → 实测崩溃 → 打补丁 → 再声称完成"的环路。
最严重的一次：检测引擎对普通 VM dispatch 循环误报 1250 个 RIPEMD-160 + 25 个
SHA-3 常量——因为只测了证实（喂 crypto 能检测），从没测证伪（喂普通数据不误报）。

本 harness 把评估指标、阈值、工作流固化，消除经验主义。

---

## 2. 评估类别与指标

5 个类别（A–E），**总体 PASS 当且仅当全部 5 个类别 PASS**（宪法 P4）。

### A. 数据完整性 (Data Integrity) — 强制

| 指标 | 定义 | 阈值 |
|---|---|---|
| total_errors | 所有数据验证脚本报告的错误总数 | **= 0** |

验证脚本（单一整合脚本，按验证方法分 7 节）：
`scripts/verify_crypto_data_integrity.py`
- 第 1 节：hex/int 一致性（所有常量）
- 第 2 节：独立数学计算（SHA-256 K / MD5 T / CRC32 table / SM4 CK / Keccak RC）
- 第 3 节：权威参考表逐条对照（AES/SM4 S-box、Blowfish、DES、SHA-256 IV、sigma、FK）
- 第 4 节：结构/对抗性质（置换性、互逆性、Keccak bit 约束、XTEA/HMAC 算术）
- 第 5 节：外部交叉验证（hashlib/zlib/XTEA test vector）
- 第 6 节：跨库一致性（sequence vs constant、metadata）
- 第 7 节：完整性守护（hex/int 冲突、值非负）

> 历史：原为 6 个独立脚本（verify_crypto_data / verify_sequences_full /
> verify_final_comprehensive / verify_round4/5/6），存在重复检查，
> 已合并为单一脚本，去重后约 4149 项检查（原 4529 含重复）。

金标准来源（宪法 P10）：FIPS 180-4 / FIPS 197 / FIPS 46-3 / RFC 1319 /
RFC 8439 / GM/T 0002-2012 / GM/T 0004-2012 / Schneier Applied Cryptography。

### B. 召回率 (Recall) — 能否检测到真的

| 指标 | 定义 | 阈值 |
|---|---|---|
| recall_pct | L1/L2 可检测算法中实际检测到的比例 | **= 100%** |
| l3_fires_with_map | L3-only 算法在合成 opcode_map 下实际 fire 的数量 | **= 8 (全部)** |

- **L1-capable**（有常量）：喂理想常量，必须 `detect_constants` 检测到
- **L2-capable**（有序列）：喂理想序列，必须 `detect_sequences` 检测到
- **L3-only**（无固定常量）：构造合成 opcode_map + trace，必须 `detect_patterns` 真正 fire

L3-only 白名单（无固定常量，仅能经 Layer 3 结构检测）：
`RC4, IDEA, XOR_Cipher, WAKE, PBKDF2, HKDF, GOST_28147, SAFER`
- key-dependent table: RC4 / IDEA / XOR / WAKE
- HMAC 构造: PBKDF2 / HKDF
- impl-defined S-box: GOST
- exp/log table: SAFER

> **Layer 3 契约（关键）**：VM opcode 是 per-VM 任意数字，behavior_pattern 是
> 语义 token。`detect_patterns` 必须接收 `opcode_map`（VM 数字 opcode → 语义
> token，来自人工逆向标定或 Phase 2 差分推断）才能匹配。**无 map 时返回 []**，
> 不做数字 vs 字符串的无效比较，不伪造匹配。B 类不再仅检查"pattern 存在"
> （旧标准给假绿），而是构造合成 map 验证 L3 真能 fire。

> harness bug 警示（宪法 P6）：召回匹配必须用**精确 token 匹配**
> （algo 是 `/` 分隔字段中的完整一项），不能用 substring——
> 否则 "SHA-3" 会误配 "SHA-384"、"SHA-1" 误配 "RIPEMD-160"。

### C. 假阳性率 (False Positive Rate) — 强制，会否误报

| 指标 | 定义 | 阈值 |
|---|---|---|
| fp_samples | conf >= 0.5 时产生误检的反例场景数 | **= 0** |

反例场景（程序化生成，固定种子，非手工挑选）：
1. 普通 VM dispatch 循环（opcode 0-65）
2. 随机字节 0-255
3. 顺序数组索引 0-15
4. 1-64 随机排列（模拟置换表数据）
5. 真实非 crypto 值（屏幕尺寸/时间戳/HTTP 码/2 的幂）
6. 随机 32-bit 值（可能巧合命中常量）

### D. 测试覆盖 (Test Coverage)

| 指标 | 定义 | 阈值 |
|---|---|---|
| positive_coverage_pct | 有正例测试的算法比例 | **= 100%** |
| fp_guard_tests | TestFalsePositives 类的守护测试数 | **>= 5** |

### E. 鲁棒性 (Robustness)

| 指标 | 定义 | 阈值 |
|---|---|---|
| determinism | 同输入两次运行输出一致 | **= True** |
| noise_recall_pct | crypto 序列后接噪声仍能检测的比例 | **= 100%** |

---

## 3. 运行方式

```bash
# 完整评估（声称完成前必跑，宪法 P7）
python scripts/evaluate_detection.py
echo $?   # 0 = PASS, 非0 = FAIL

# 专项详细输出（诊断用）
python scripts/audit_false_positives.py   # C 类明细
python scripts/check_coverage.py          # B/D 类明细
```

---

## 4. 当前基线 (2026-06-01)

| 类别 | 指标 | 实测 | 阈值 | 状态 |
|---|---|---|---|---|
| A | total_errors | 0 (4149 checks) | 0 | PASS |
| B | recall_pct | 100% (43 算法) | 100% | PASS |
| B | l3_fires_with_map | 8/8 | 8 | PASS |
| C | fp_samples | 0 (6 场景) | 0 | PASS |
| D | positive_coverage_pct | 100% (51 算法) | 100% | PASS |
| D | fp_guard_tests | 6 | >=5 | PASS |
| E | determinism | True | True | PASS |
| E | noise_recall_pct | 100% (5/5) | 100% | PASS |

**OVERALL: PASS**

数据库规模：51 patterns / 216 constants / 24 sequences。
关联 pytest：`tests/test_crypto_detection.py`（104 tests）。

---

## 5. 已知局限（诚实记录，宪法 P8）

- **L3 需要 opcode_map**：8 个 L3-only 算法（RC4/IDEA/XOR/WAKE/PBKDF2/HKDF/
  GOST/SAFER）只能经 Layer 3 结构检测，而 Layer 3 必须由调用方提供 VM 的
  opcode→语义映射才能 fire。harness 已用合成 map 验证 8/8 全部 fire；但**真实
  使用时 map 来自人工逆向标定或 Phase 2 差分推断**——无 map 时这 8 个不可检测。
  这是设计契约（宪法 P8），非 bug。
- **真实 trace 召回 vs 理想输入召回**：B 类用理想合成输入测召回。真实 trace 的
  常量编码变体（已验证：decimal/hex/负数补码均命中）、序列交错（需显式 max_gap）、
  乱序访问（序列匹配固有失效，靠 L1 兜底）见 `audit_m27_realworld.py`。
- 序列匹配是动态 trace 值匹配，非 ghidra-findcrypt 的静态完整表扫描；
  trace 中表值不连续/不完整时召回会下降（已通过 E 类噪声测试部分覆盖）。
- 共享常量算法（TEA/XTEA/RC5/Serpent/SEED 共享 0x9E3779B9）只能标注歧义，
  无法单独确定 —— 数学事实，非缺陷。
