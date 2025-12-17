# 坦克大战 - Tank Tank Tank

一个使用 Rust 和 Bevy 游戏引擎开发的经典坦克大战游戏。

## 游戏特性

- **经典玩法**：控制坦克击败敌人，保卫阵地
- **智能 AI**：敌方坦克具有巡逻、追击和攻击行为
- **波次系统**：逐渐增加的难度和敌人数量
- **完整的游戏机制**：生命系统、得分系统、碰撞检测
- **可破坏环境**：包含可破坏和不可破坏的障碍物

## 游戏操作

### 移动控制
- **WASD** 或 **方向键**：控制坦克移动
- **鼠标**：控制坦克炮塔旋转方向

### 射击
- **空格键** 或 **鼠标左键**：发射子弹

### 其他
- **ESC**：暂停/继续游戏

## 运行游戏

### 前置要求
- Rust 1.70 或更高版本
- Cargo

### 安装和运行

#### 1. 准备资源文件

**字体文件（必需）：**

游戏使用 Adobe 思源黑体（Source Han Sans）显示中文文字。请先下载字体：

1. 访问 [Source Han Sans 官方发布页](https://github.com/adobe-fonts/source-han-sans/releases)
2. 下载 `SourceHanSansCN-Regular.otf` 文件
3. 将文件放置在 `assets/fonts/` 目录下

详细说明请查看 `assets/fonts/README.md`

**坦克贴图（可选）：**

游戏支持使用自定义贴图来显示坦克。如果不提供贴图，游戏将使用纯色方块：

1. 准备两个 PNG 图片文件：
   - `player_tank.png` (40x40 像素) - 玩家坦克
   - `enemy_tank.png` (35x35 像素) - 敌人坦克
2. 将文件放置在 `assets/textures/` 目录下

详细说明请查看 `assets/textures/README.md`

#### 2. 编译和运行

```bash
# 克隆仓库
git clone https://github.com/Qinka/tank-tank-tank.git
cd tank-tank-tank

# 运行游戏
cargo run --release
```

开发模式运行（编译更快但性能较低）：
```bash
cargo run
```

## 游戏机制

### 玩家
- 初始 3 条生命
- 生命值耗尽后失去一条生命
- 所有生命耗尽后游戏结束

### 敌人
- 敌人会在地图边缘生成
- 具有三种 AI 模式：
  - **巡逻模式**：在地图上随机移动
  - **追击模式**：发现玩家后进行追击
  - **攻击模式**：接近玩家后开火射击

### 得分
- 击毁每个敌方坦克获得 100 分
- 分数越高，难度越大

### 波次系统
- 每波敌人数量逐渐增加
- 消灭所有敌人后进入下一波

### 地图元素
- **灰色墙壁**：不可破坏的障碍物
- **棕色墙壁**：可以被子弹摧毁
- **地图边界**：限制移动范围

## 项目结构

```
tank-tank-tank/
├── Cargo.toml              # 项目配置和依赖
├── README.md               # 项目说明
└── src/
    ├── main.rs             # 程序入口
    ├── lib.rs              # 库入口
    ├── components.rs       # ECS 组件定义
    ├── constants.rs        # 游戏常量配置
    ├── resources.rs        # 全局资源
    ├── states.rs           # 游戏状态机
    └── systems/            # 游戏系统
        ├── mod.rs          # 系统模块
        ├── setup.rs        # 初始化系统
        ├── player.rs       # 玩家控制系统
        ├── enemy.rs        # 敌人 AI 系统
        ├── combat.rs       # 战斗系统
        ├── collision.rs    # 碰撞检测系统
        └── ui.rs           # UI 系统
```

## 技术栈

- **Rust** - 系统编程语言
- **Bevy 0.14** - 数据驱动的游戏引擎
- **ECS 架构** - Entity Component System 设计模式

## 开发说明

### 修改游戏参数

可以在 `src/constants.rs` 文件中调整各种游戏参数：
- 窗口大小
- 坦克速度和尺寸
- 子弹速度和伤害
- AI 检测范围
- 生成间隔

### 编译优化

项目已配置开发环境优化，在 debug 模式下也能获得较好的性能：
```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 贡献

欢迎提交 Issue 和 Pull Request！

## 致谢

- 使用 [Bevy](https://bevyengine.org/) 游戏引擎
- 灵感来自经典的坦克大战游戏