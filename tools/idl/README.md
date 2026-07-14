# IV8 IDL Toolchain

v0.8.18 IDL 预处理工具链 — 从 W3C Webref 和 Chromium 源码提取 Web IDL 接口定义，
产出 `unified_ir.json` 供 v0.8.19 Rust 代码生成器使用。

## 环境要求

- Node.js >= 18.x (LTS)，推荐 >= 20.x
- npm >= 9.x

## 安装

```bash
cd tools/idl
npm install
```

## 运行

```bash
node generate-ir.js
```

输出在 `output/unified_ir.json`。

## Go Gate 验证

```bash
node validate.js --go-gate
```

当前状态：**OVERALL: PASS**（13/13）。

## 管线脚本

| 脚本 | 功能 |
|---|---|
| `fetch-webref.js` | 从 @webref/idl 拉取并解析全部 325 个 W3C Web IDL 规范 |
| `normalize-ast.js` | 将 webidl2 AST 转换为 IV8 内部归一化格式 |
| `merge-tool.js` | 合并 partial interface、展开 mixin、处理 includes、Kahn 拓扑排序 |
| `type-mapper.js` | IDL 类型映射覆盖率统计 |
| `validate.js` | 13 项 Go Gate 验证器 |
| `generate-ir.js` | 主管线编排器（一键运行全流程） |
| `parse-chromium-idl.js` | 使用 webidl2 解析 Chromium .idl 文件 |

## 数据源

| 来源 | 位置 | 接口数 |
|---|---|---|
| W3C Webref | @webref/idl npm 包 | ~1123 interfaces |
| Chrome Extensions | chrome-manual/*.ir.json（手工编写） | ~161 interfaces |
| **总计** | | **~1284 interfaces** |

## 目录结构

```
tools/idl/
├── chrome-manual/          # 手工编写的 Chrome 扩展 IR
│   ├── chrome-extensions.ir.json
│   └── chrome-stubs.ir.json
├── output/                 # 管线产出
│   └── unified_ir.json
├── tmp/                    # 临时文件（不提交）
├── package.json
├── generate-ir.js          # 主管线入口
├── fetch-webref.js
├── normalize-ast.js
├── merge-tool.js
├── type-mapper.js
├── validate.js
└── parse-chromium-idl.js
```

## unified_ir.json Schema

```json
{
  "schema_version": "unified-ir.v0.1",
  "metadata": {
    "generated_at": "ISO 8601 timestamp",
    "webref_version": "from @webref/idl/package.json",
    "webidl2_version": "from webidl2/package.json",
    "total_interfaces": 1284,
    "...": "..."
  },
  "definitions": [
    {
      "kind": "interface|dictionary|enum|typedef|callback|callback_interface|namespace",
      "name": "InterfaceName",
      "source": "w3c|chromium|manual",
      "inheritance": "ParentInterface|null",
      "ext_attrs": [],
      "members": [
        {
          "kind": "attribute|operation|const|constructor",
          "name": "memberName",
          "type": { "kind": "name|generic|union|primitive", "name": "...", "nullable": false },
          "...": "..."
        }
      ]
    }
  ]
}
```

## Known limitations / troubleshooting

- `tmp/` is gitignored intermediate output (`tools/idl/.gitignore`). Safe to delete entirely; next `node generate-ir.js` recreates it.
- `tmp/first_run.json` can duplicate large IR blobs relative to `output/unified_ir.json`; not required after a successful run.
- `parse-chromium-idl.js` is not wired into `generate-ir.js`; Chrome extras currently come from `chrome-manual/*.ir.json`.
- Pipeline has no stage tests and no incremental rebuild cache.
- Metadata versions are resolved at generate time from installed `node_modules` (`@webref/idl`, `webidl2`); pin via `package.json` + lockfile.

## 下游版本

- v0.8.19：Rust 代码生成器（从 unified_ir.json 生成 iv8-surface crate）
- v0.8.20：Feature Flag 架构 + 初始化链替换
- v0.8.21+：深桩实现
