# CCBot

这是一个由Rust编写而成的Discord机器人，使用了[serenity](https://github.com/serenity-rs/serenity)
以及它的姊妹库[songbird](https://github.com/serenity-rs/songbird)和[poise](https://github.com/serenity-rs/poise)。

旨在使用最爆炸的语言，最混乱的库，最💩的代码，编写最简单的Discord机器人。

- [serenity](https://github.com/serenity-rs/serenity)机器人主库
- [songbird](https://github.com/serenity-rs/songbird)语音服务的依赖库
- [poise](https://github.com/serenity-rs/poise)命令程序提供库

## 使用

**语音服务**：

- `/join`:加入语音频道
- `/leave`:离开语音频道
- `/play_music <url>`:播放音频
- `/stop`: 停止播放

**通用命令**

- `ping`: 查看用户创建时间
- `set_status`:设置机器人状态
- `clear <amount>`:清除<amount>聊天记录

对于`/play_music`命令，支持Youtube和Bilibili的音频播放，理论上支持youtube，但由于yt-dlp的限制，可能会出现一些问题。
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

[x] 为机器人添加消息请求时的等待状态显示  
[ ] 为机器人添加私聊功能  
[x] 调查stop为什么会有一个发送。因为这个命令没有参数，所以有一个发送。  
[x] 添加消息清除功能，例如`/clear 100`为清理历史100条消息。  
[ ] 编写readme和部署指南  
[ ] 编写频道撤回功能，例如`withdraw <#1375147894847307807> up`开启后将维持住最后一条消息，和禁言功能一样。
`withdraw <#1375147894847307807> down`关闭撤回功能。该功能依赖于serde数据保存，需要首先完善配置读写。  


