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
  
- 淦，这个机器人返回消息的类型太多了，什么`CreateMessage`、`CreateReply`，我自己去看md文档吧。
- 我嘞个，我才知道message里的private代表的是私聊ORZ。还有什么dm频道，居然就是私聊频道，淦！
- `Unknown interaction`在命令中，这个错误代表没有申请延时操作。
  >为什么会发生 Unknown interaction？
  未在 3 秒内响应 (HTTP 200)
  >Discord 要求 必须在 3 秒内对交互请求返回初始响应（ACK 或实际回复），否则会标记为 Unknown interaction。
  解决方案：
  使用 deferReply（如果是长时间操作，先返回“机器人正在处理”）。
  如果无法在 3 秒内完成，先返回 ACK（type: 5 或 defer: true），再用 followUp 发送最终结果。
