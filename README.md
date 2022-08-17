# 大作业：Wordle

2022 年夏季学期《程序设计训练》 Rust 课堂大作业（一）。

## 作业要求

具体要求请查看[作业文档](https://lab.cs.tsinghua.edu.cn/rust/projects/wordle/)。

## Honor Code

请在 `HONOR-CODE.md` 中填入你完成作业时参考的内容，包括：

* 开源代码仓库（直接使用 `crate` 除外）
* 查阅的博客、教程、问答网站的网页链接
* 与同学进行的交流

## 自动测试

本作业的基础要求部分使用 Cargo 进行自动化测试，运行 `cargo test [--release] -- --test-threads=1` 即可运行测试。其中 `[--release]` 的意思是可以有也可以没有，例如 `cargo test -- --test-threads=1` 表示在 debug 模式下进行单线程测试，而 `cargo test --release -- --test-threads=1` 表示在 release 模式下进行单线程此时。

如果某个测试点运行失败，将会打印 `case [name] incorrect` 的提示（可能会有额外的 `timeout` 提示，可以忽略）。你可以在 `tests/data` 目录下查看测试用例的内容，还可以使用以下命令手工测试：

```bash
cp tests/cases/[case_name].before.json tests/data/[case_name].run.json # 复制游戏初始状态文件（如果需要）
cargo run [--release] -- [options] < test/cases/[case_name].in > test/cases/[case_name].out # 运行程序
diff test/cases/[case_name].ans test/cases/[case_name].out # 比较输出
jd -set tests/data/[case_name].after.json tests/data/[case_name].run.json # 比较游戏状态文件（如果需要）
```

其中 `[options]` 是游戏使用的命令行参数，`[case_name]` 是测试用例的名称。`jq` 工具可以使用各类包管理器（如 `apt` 或 `brew`）安装。

项目配置了持续集成（CI）用于帮助你测试。在推送你的改动后，可以在 GitLab 网页上查看 CI 结果和日志。

## 其他说明

* `src/builtin_words` 是内嵌于程序中的单词列表，`FINAL` 为所有答案词，`ACCEPTABLE` 为所有候选词。
