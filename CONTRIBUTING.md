# 项目贡献指南

## 代码规范
### 通用规则
1. **缩进**：4 个空格的Tab缩进。
```rust
// 正确示例
fn example() {
	println!("使用 4 空格缩进");
}
```
2. **命名约定**：
    - 变量/函数：`snake_case`
    - 类型：`PascalCase`
    - 常量：`SCREAMING_SNAKE_CASE`
```rust
// 正确示例
fn example_function() {
    let example_variable: i32 = 42;
    const EXAMPLE_CONSTANT: i32 = 42;
    struct ExampleStruct;
}
```

## 提交规范
### 提交消息规范
```
类型(范围): 简明描述

详细描述（可选）

解决 #issueID(可选)
```
**类型**：
- `Feat`: 新功能
- `Fix`: 错误修复
- `Docs`: 文档更新
- `Style`: 代码样式调整
- `Refactor`: 代码重构
- `Perf`: 性能优化
- `Test`: 测试相关
- `Chore`: 构建/配置变更
- `CI`: 持续集成相关
- `Revert`: 回滚提交
- `WIP`: 工作中（未完成）
**示例**：
```
Feat(Authentication): 修复认证API存在的问题

- 修复签名异常
- 添加用户认证状态管理
- 修改登录页面UI不符合规范的设计

解决 #114
```
### 分支规范
在开发新功能或修复问题时，请创建一个新的分支，并在完成后合并到主分支。
```bash
# 创建新分支
git checkout -b <类型>/<简短描述>-<issue号> # 例如: Feat/dark-mode-424

# 开发并提交更改
git add .
git commit -m "<提交消息, 参考上文>"

# 接下来，发送Pull Request
```
## 构建与发布
### 本地构建
```bash
# 生产构建
cargo build --release
```

### 发布流程
1. 更新版本号：
```bash
cargo bump [major|minor|patch]
```

2. 创建发布分支：
```bash
git checkout -b release/vX.Y.Z
```

3. 更新变更日志：
```markdown
## [X.Y.Z] - YYYY-MM-DD

### 新增
- 功能 A (#123)
- 功能 B (#124)

### 修复
- 问题 C (#125)
```

4. 提交并推送：
```bash
git commit -m "chore(release): vX.Y.Z"
git push origin release/vX.Y.Z
```
5. 创建 Pull Request 等待审核合并

感谢您的贡献！🎉

> 存在疑问？通过Issue 或 Email 与我们联系。
> Email: [bilirumble@outlook.com](mailto:bilirumble@outlook.com)
