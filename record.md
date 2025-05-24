# 记录一些开发中容易忘记的事

# Poise

- `#[poise::command]`注释的方法中，对于参数使用`#[autocomplete = "autocomplete_func"]`来自定义补全函数，函数的结构为
  <br>

```rust
use futures::{Stream, StreamExt};
async fn autocomplete_activity_type(
    _ctx: PoiseContext<'_>,
    partial: &str,
) -> impl Stream<Item=String> {}
```

<br>

- ``#[description = "选择一个用户"]``表示为一个参数的注释，会在Discord bot中显示出来。

- 对于添加了`#[poise::command]`方法的注释也是机器人命令的描述。
- ```rust 
  #[poise::command(
  slash_command,
  required_permissions = "ADMINISTRATOR"
  )]
  async fn my_command(ctx: PoiseContext<'_>) -> Result<(), Error> {
      // ...
  } 
  ```
  `required_permissions = "ADMINISTRATOR"`描述了这个命令的权限要求。

