# 大作业：Wordle

2022 年夏季学期《程序设计训练》 Rust 课堂大作业（一）。

## 程序结构

```c
src
├── app
│   ├── colors.rs		// 颜色常量
│   ├── definition.rs	// 单词释义面板
│   ├── grid.rs			// 字母矩阵
│   ├── keyboard.rs		// 键盘组件以及输入监测
│   ├── letter.rs		// 字母组件
│   ├── metrics.rs		// 尺寸及布局相关常量
│   ├── settings.rs		// 设置面板
│   ├── stats.rs		// 统计数据面板
│   ├── utils.rs		// 工具函数
│   └── visuals.rs		// 视觉风格（明亮 / 暗黑模式）
├── app.rs				// GUI
├── args.rs				// 参数解析及校验
├── builtin_words.rs	// 预置词库
├── dict.rs				// 预置词典
├── game.rs				// 游戏逻辑
├── main.rs				// CLI
└── stats.rs			// 统计数据记录及存储
```

Crate 层级下的模块（除 `app` 外）为 CLI 及 GUI 所共用，`app` 及其子模块为 GUI 独有。

## 游戏主要功能说明

### CLI

直接启动程序将进入 CLI 交互模式。在此模式下将会首先要求玩家指定游戏的答案，然后开始一局 Wordle 游戏。

![CLI](images/cli.png)

交互模式下每次猜测后将会显示所有猜测的结果以及每个字母的状态。若输入 `HINT` 将能够获取一个提示：

![Hint](images/hint.png)

游戏结束后，将会得到单词的释义并询问是否进行下一局游戏。

![Definition](images/definition.png)

在 CLI 模式下，可以指定一些参数来自定义游戏体验。

![CLI Options](images/options.png)

| 参数                      | 子参数        | 作用                                               | 备注                                               |
| ------------------------- | ------------- | -------------------------------------------------- | -------------------------------------------------- |
| `--acceptable-set` / `-a` | 路径 `<FILE>` | 指定允许的猜测词库，每行一个 5 字母单词            |                                                    |
| `--config` / `-c`         | 路径 `<FILE>` | 指定默认配置文件，格式为 JSON                      | 命令行参数相较于配置文件有更高优先级               |
| `--day` / `-d`            | 整数 `<DAY>`  | 指定游戏天数，即种子与天数决定答案                 | 依赖于 `--random`；范围为 1 至答案词库的大小（含） |
| `--difficult` / `-D`      |               | 开启困难模式，每次猜测必须使用上一次猜测得到的提示 |                                                    |
| `--final-set` / `-f`      | 路径 `<FILE>` | 指定答案词库，每行一个 5 字母单词                  | 答案词库必须是猜测词库的子集                       |
| `--gui` / `-g`            |               | 启动 GUI                                           | 此时不再解析其他参数                               |
| `--help` / `-h`           |               | 显示帮助信息                                       |                                                    |
| `--random` / `-r`         |               | 随机抽取答案                                       | 与 `--word` 冲突                                   |
| `--seed` / `-s`           | 整数 `<SEED>` | 指定随机数种子                                     | 依赖于 `--random`                                  |
| `--state` / `-S`          | 路径 `<FILE>` | 开启游戏状态存储并制定存储路径                     |                                                    |
| `--stats` / `-t`          |               | 游戏结束后展示统计信息                             |                                                    |
| `--word` / `-w`           | 单词 `<WORD>` | 指定答案                                           | 与 `--random` 冲突；答案应在答案词库中             |

下面将展示一些命令行参数的功能以及对一些错误输入的检测。

![Difficult Mode](images/difficult.png)

![Statistics](images/stats.png)

### GUI

GUI 既支持本地运行，也能通过编译到 WebAssembly 的方式在 Web 上运行。目前本项目部署在[这里](https://abmfy.github.io/wordle/)。

![GUI](images/gui.png)

GUI 充分利用了回车键以及退格键进行信息展示与交互。例如，正常模式下，回车键不可被按下：

![Enter Disabled](images/enter_disabled.png)

只有在输入了 5 个字母并且单词在词库中时，回车键才会变为可用状态：

![Enter Enabled](images/enter_enabled.png)

游戏结束时，退格键将变为下一局按钮，且若没有猜对单词，回车键将会展示正确答案：

![Enter With Answer](images/answer.png)

设置面板提供了困难模式的切换以及随机种子、游戏天数的选取。值得一提的是，困难模式下 GUI 将转为深邃的暗黑模式：

![Dark Mode](images/dark.png)

在键盘区输入 `HINT`，回车键将变为提示按钮，点击即可获取提示：

![Hint Button](images/hint_button.png)

![Hint Shown](images/hint_got.png)

统计面板将会展示统计数据：

![Statistics Panel](images/stat_panel.png)

释义面板将在游戏结束后展示单词的释义：

![Definition Panel](images/def_panel.png)

另外，GUI 对手机进行了一定的适配，能够在手机上正常游玩。在手机上运行时，三个面板区域将收纳到一个折叠按钮下避免遮挡字母矩阵：

<img src="images/mobile.png" alt="Mobile Mode" style="width: 50%;" />

<img src="images/mobile_panel.png" alt="Mobile Mode Panel" style="width: 50%;" />

## 提高要求实现

### GUI

采用 [egui](https://crates.io/crates/egui) 库及 [eframe](https://crates.io/crates/eframe) 框架实现。字母矩阵以及键盘通过直接绘制图形并监听点击及键盘输入事件实现。动画效果通过 `egui::Contenxt::animate_value_with_time` 实现。暗黑模式效果通过切换全局 `Visuals` 并添加过渡动画实现。手机适配通过检测屏幕尺寸实现。

使用 [trunk](https://trunkrs.dev) 将编译完成的 WebAssembly 进行打包后，即可部署到服务器上。

### 提示

提示功能使用一种简单直接的方式实现：从词库中符合之前全部提示信息的词中抽取一个。由于在实际游玩中玩家很少在前期（第一或第二次猜测）立即使用提示，这样的提示效果不错。

### 单词释义

单词释义的实现工作主要集中于对词典的处理上。

首先从 macOS 的 Dictionary.app 中提取 New Oxford American Dictionary 的数据，使用 [JadedTuna/apple-dictionary](https://github.com/JadedTuna/apple-dictionary) 项目进行解析得到 [`dictionary.xml`](https://cloud.tsinghua.edu.cn/f/f165e853e90441a78f13/)（由于大小原因，此原始词典数据并没有附在仓库中）。此后通过 `dict_gen.py` 对词典进行预处理，包括提取答案词库中对应词条、将派生词映射到原始词、对少部分词典中不存在的词进行补充等生成 `assets/dict.json` 文件。

## 完成作业感想

完成本次作业的一周是忙碌而充实的。完成前期基础功能的过程中，我深感 Rust 虽然是一门相当年轻的语言，却有着繁荣的生态。`console`、`clap`、`rand` 以及 `serde_json` 等第三方库为基础功能的实现提供了莫大的助力，其中 `clap` 以及 `serde_json` 等库充分利用 Rust 的宏实现了方便快速的数据结构化，在保证静态性的同时提供了方便快捷的开发体验。Rust 强大的编译期检查也让大部分内存相关 bug 在编译器就得以发现，提高了开发效率。

开始实现提高部分时，我注意到 egui 能够同时编译到本地以及 WebAssembly，于是决定直接使用 egui 实现 GUI。刚开始的两三天，GUI 的开发进度其实是比较缓慢的，这是因为 egui 与大部分常见的 UI 框架不同，它是即时布局的，而且没有事件机制。这在带来了开发的快捷性的同时也带来了一个巨大的问题：它的布局能力相对较弱。因此在实现字母矩阵以及键盘时，我直接将它们按计算好的坐标绘制到屏幕上来解决布局问题。在探索了一天之后，我终于绘制了第一版的界面：

![First Edition of GUI](images/first_edition.png)

如果只是要做一个「能玩」的 GUI 的话，其实也许并不需要这么久。开发过程中的大部分时间其实都花在了一些对细节的追求上：

+ 字母状态改变时，希望能有一些动画效果。原版 Wordle 的翻转效果在 egui 框架下并不是那么容易实现，因此我用了简单的颜色渐变。这个渐变效果不仅应用于字母矩阵中的颜色，也在键盘区颜色改变时可以看到。
+ 为了体现一些「高级感」，困难模式下整个 UI 将转为暗黑模式：不仅是背景色变黑，而是整套配色都更换了。然而，egui 自带的视觉风格改变并没有过渡动画，这让我觉得非常生硬，因此花了一个下午给明亮/暗黑模式间的切换加了过渡动画。
+ 该在哪提示游戏结果？我想了很久这个问题，最后看向了回车和退格键。于是，它们开始逐渐承担越来越多奇怪的功能。在本项目中，回车键的功能包括：输入合法性提示、提交、答案显示、获取提示，退格键的功能包括：退格、下一局。
+ 窗口缩放得比较小的时候，字母矩阵会把键盘盖住，同时在手机上运行时，键盘会变得很大……于是做了一点尺寸自适应，让手机也能愉快地 Wordle。
+ 在手机上，左上角的三个面板若使用 Frame 会被字母矩阵挡住（因为是画上去的），若使用 Window 则会挡住字母矩阵。最后为手机和桌面端分开做了显示，手机端用一个折叠的 Window 装三个 CollapsingHeader，桌面端则直接使用三个 Frame。
+ 很多时候在答案揭晓后，你都会好奇：「Wow，这是个什么词？」于是加入了游戏结束后显示单词释义的功能。这个功能其实不太难，但是对词典的预处理非常繁琐。例如，有的词是别的词的派生词或者在词典中的原始词长得比较奇怪（éclat, ’twixt, quasi-），要把这种词映射到原始词才能在词典里找到释义。
+ 提示功能放在哪，加个按钮吗？但是那样会引导玩家更多地使用提示。注意到 HINT 一词只有四个字母，于是想到可以让检测到输入的词是 HINT 时，回车按钮变为提示按钮，这样既不影响正常猜测（虽然有个极特殊的情况是，猜测词是 HINTS），又让提示功能招之即来。这样还有一个问题：玩家怎么知道输入 HINT 会有提示呢？答案是单词释义面板。单词释义只有在游戏结束后才会出现，而提示功能只有在游戏进行时才会需要，于是在游戏进行时如果打开释义面板，就能看到键入 HINT 就能获得提示。

开发完成后，Rust 直接编译到 WebAssembly 的能力再次让我惊叹不已，将编译后的网页部署的过程也非常顺利，最后的成品总体上达到了令我自己满意的程度。在开发的过程中，我也学会了如何组织管理 Rust 项目。Rust 对我而言是一种重塑了整个编程思考方式的语言，期待接下来的 Rust 学习中能学到更多新知识！
