# CCBot

这是一个由Rust编写而成的Discord机器人，使用了[serenity](https://github.com/serenity-rs/serenity)
以及它的姊妹库[songbird](https://github.com/serenity-rs/songbird)和[poise](https://github.com/serenity-rs/poise)。

旨在使用最爆炸的语言，最混乱的库，最💩的代码，编写最简单的Discord机器人。

- [serenity](https://github.com/serenity-rs/serenity)机器人主库
- [songbird](https://github.com/serenity-rs/songbird)语音服务的依赖库
- [poise](https://github.com/serenity-rs/poise)命令程序提供库

## 使用

**语音服务**：

- `/music join <channel>`:加入语音频道
- `/music leave <channel>`:离开语音频道
- `/music play <url>`:使用[yt-dlp](https://github.com/yt-dlp/yt-dlp)作为支持。`[关键词|AV|BV]定位B站资源|直接链接]`
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

对于`/music play`命令，支持Youtube和Bilibili的音频播放，理论上支持youtube，但由于yt-dlp的限制，可能会出现一些问题。

对于ai对话，需要@机器人，机器人会向上回溯50条消息，并过滤出你与机器人的消息进行对话。

## 安装&运行

该程序依赖于[yt-dlp](https://github.com/yt-dlp/yt-dlp)
，如何安装请参考[install](https://github.com/yt-dlp/yt-dlp?tab=readme-ov-file#installation)

rust编译问题请参考rust给出的报错，安装对应的依赖包即可。

从[action](https://github.com/ChengCY-2254/discord_hub_bot/actions)中下载，并将其放到你的部署路径。
第一次可直接运行`./cc-bot`，会自动创建配置文件夹`config`，并在配置文件夹中创建`ai-config.json`和`.env`文件。
需要修改其中的.env文件，填入你的Discord Bot Token和ai-config.json中的配置信息。
然后运行`./cc-bot`即可。

## 功能列表

- [x] 为机器人添加消息请求时的等待状态显示
- [x] 为机器人添加私聊功能
- [x] 添加消息清除功能，例如`/clear 100`为清理历史100条消息。
- [x] 编写readme和部署指南
- [x] 编写频道撤回功能。
- [x] 添加版本号，使用`build.rs`来记录git的id并从env中获取版本号。
- [x] 精简配置文件
- [x] 给受到聊天管控的区域添加管控公告。
- [x] 原神，启动！2025/05/24
- [x] 未添加到语音频道的时候使用播放音乐给出正确的错误提示。
- [x] 如果是dm频道，那么就只需要删除自己的消息即可。
- [x] 为/switch_prompt 添加仅管理员可用，而不是只能在频道中使用。,
- [x] /music play_for_bilibili [keyword]参数支持AV&BV号，这个需要在注释中给出。,
- [x] 私聊不用@,
- [x] 合并两个音乐播放命令，如果是HTTP开头，就使用new，如果是文本，那就默认去哔哩哔哩搜索。
- [ ] 记录当前加入的语音频道在机器人重启后自动加入。