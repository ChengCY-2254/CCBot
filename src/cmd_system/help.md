**音乐服务**：

- `/music join <channel>`:加入语音频道
- `/music leave <channel>`:离开语音频道
- `/music play <url>`:使用[yt-dlp](https://github.com/yt-dlp/yt-dlp) 作为支持。`[关键词|AV|BV]定位B站资源|直接链接]`
- `/music stop`: 停止播放

**通用命令**

- `ping`: 查看用户创建时间
- `status`:设置机器人状态
- `clear <amount>`:清除<amount>聊天记录
- `switch_prompt <file_name>`:设置ai的系统提示词，提示词存放路径在`config/`下，扩展名为`.md`
- `send_message <channel> <message>`: 发送消息到指定频道

**管理命令**

- `/withdraw add <channel>`:将一个频道添加到撤回列表
- `/withdraw remove <channel>`:将一个频道从撤回列表移除
- `/withdraw list `:查看当前禁言列表

更多请查看[CC-Bot](https://github.com/ChengCY-2254/CCBot)