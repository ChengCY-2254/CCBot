# CCBot 使用手册

## 📦 命令分类

### 🎵 音乐服务

| 命令                  | 功能说明   | 使用方法                   | 参数说明                     |
|---------------------|--------|------------------------|--------------------------|
| `/music join`       | 加入语音频道 | `/music join #频道名`     | `#频道名`：目标语音频道            |
| `/music leave`      | 离开语音频道 | `/music leave`         | 无                        |
| `/music play`       | 播放音乐   | `/music play [关键词/链接]` | `[关键词/链接]`：支持B站/BV号/直接链接 |
| `/music stop`       | 停止播放   | `/music stop`          | 无                        |
| `/music pause`      | 暂停播放   | `/music pause`         | 无                        |
| `/music continue`   | 继续播放   | `/music continue`      | 无                        |
| `/music set_volume` | 调整音量   | `/music set_volume 50` | `50`：音量值（1-100）          |

> 🎵 播放音乐时会自动识别B站链接或关键词搜索，支持[yt-dlp](https://github.com/yt-dlp/yt-dlp)支持的所有平台

---

### 🧰 通用命令

| 命令                | 功能说明     | 使用方法                    | 特殊说明                                      |
|-------------------|----------|-------------------------|-------------------------------------------|
| `/ping`           | 查看用户创建时间 | `/ping @用户名`            | 可单独使用查看机器人延迟                              |
| `/status`         | 设置机器人状态  | `/status playing 正在玩原神` | 支持类型：playing/listening/watching/streaming |
| `/clear`&`!clear` | 清除聊天记录   | `/clear 50` `!clear 50` | 需要「管理消息」权限                                |
| `/register`       | 注册应用命令   | `@机器人 register`         | 仅限机器人主人使用                                 |

---

### 🛡️ 管理命令

#### 🗑️ 频道撤回管理

| 子命令                | 功能说明   | 使用方法                     | 
|--------------------|--------|--------------------------|
| `/withdraw add`    | 添加撤回频道 | `/withdraw add #公告频道`    |
| `/withdraw remove` | 移除撤回频道 | `/withdraw remove #公告频道` |
| `/withdraw list`   | 查看撤回列表 | `/withdraw list`         |

> ⚠️ 该功能开启后会自动删除指定频道所有消息

#### 📨 消息管理

| 命令               | 功能说明    | 使用方法                       |
|------------------|---------|----------------------------|
| `/send_message`  | 跨频道发消息  | `/send_message #测试频道 "你好"` |
| `/switch_prompt` | 切换AI提示词 | `/switch_prompt 提示词文件.md`  |

> 📁 提示词文件需存放在`config/`目录下，格式为`.md`

---