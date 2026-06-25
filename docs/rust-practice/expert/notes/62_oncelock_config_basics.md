# 62 OnceLock Config Basics - 笔记

## 本题会用到的前置知识点

- `HashMap` 基础。
- `Result` / `Option` 错误处理。
- `static` 的基础概念。

## 核心结论

- 全局状态优先做成“初始化一次 + 后续只读”。
- `OnceLock` 比 `static mut` 安全得多。
- 第二次初始化应明确返回错误，不要静默覆盖。

## 核心示例

```rust
use std::collections::HashMap;
use std::sync::OnceLock;

static CONFIG: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

fn init_config() -> Result<(), &'static str> {
	let mut map = HashMap::new();
	map.insert("mode", "dev");
	map.insert("region", "local");
	CONFIG.set(map).map_err(|_| "config already initialized")
}

fn get_config(key: &str) -> Option<&'static str> {
	CONFIG.get().and_then(|m| m.get(key).copied())
}
```

理解重点：`set` 只会成功一次，读取走 `get`，全程不需要 `static mut`。

## 常见坑

- 继续使用 `static mut`。
- 不区分“未初始化/初始化失败/key 不存在”。
- 把过多业务逻辑塞进初始化函数。

## 回看问题

- 这份配置是否真的必须全局？
- 初始化后是否保持只读？
