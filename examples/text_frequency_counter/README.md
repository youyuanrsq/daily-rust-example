# 文本频率统计器

## 任务描述

实现一个简单的文本频率统计器，要求：

1. 从一个字符串中统计每个单词出现的频率
2. 使用 Rust 的 HashMap 数据结构来存储结果
3. 将单词转换为小写进行统计
4. 最后按照频率降序输出前 N 个最常见的单词（N 由用户指定）

示例输入：
"The quick brown fox jumps over the lazy dog. The dog sleeps."
N = 3

预期输出：
the: 2
dog: 2
quick: 1
</task_description>

<learning_objectives>

1. 掌握 Rust 中 HashMap 的基本使用方法和所有权概念
2. 学习 Rust 字符串操作和迭代器的使用
3. 练习 Rust 中的排序和切片操作
</learning_objectives>

<hints>
1. 可以使用 split_whitespace() 方法将字符串分割成单词，使用 to_lowercase() 转换大小写
2. 考虑使用 HashMap 的 entry() API 来优雅地处理词频统计，这比 Python 的 dict.get() 更符合 Rust 风格
</hints>

这个任务适合作为 Rust 入门练习，因为：

- 它涉及常见的数据结构（HashMap）
- 包含字符串处理，这是 Rust 和 Python 有明显区别的地方
- 需要使用迭代器和排序，这些是 Rust 中的重要概念
- 难度适中，可以在 30-60 分钟内完成
- 有实际应用价值
