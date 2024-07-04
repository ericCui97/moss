# cmoss

moss lang implemetation by c.

## get started

start:

```bash
 # 切到cmoss目录下
 cd cmoss
 # 编译
 make
 # 运行
 ./cmoss

 # 清理编译产物
 make clean

```

## todo

1. line info in chunk struct waste too much memory.most of instructions has same line number. [run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding)
2. better debug info.

## design

### chunk

代表一个块 chunk，内部有指令序列 code，和值序列 valueArray

```asciidoc
+------------------+
| int count        | 4 bytes
+------------------+
| int capacity     | 4 bytes
+------------------+
| uint8_t*code    | 8 bytes (64-bit system)
+------------------+
| int* lines       | 8 bytes (64-bit system)
+------------------+
| ValueArray       | 24 bytes
|   int capacity   | 4 bytes
|   int count      | 4 bytes
|   Value* values  | 8 bytes (64-bit system)
+------------------+
```

### valueArray

value 是 double类型，八个字节。

```asciidoc
+------------------+
| int capacity     | 4 bytes
+------------------+
| int count        | 4 bytes
+------------------+
| Value* values    | 8 bytes (64-bit system)
+------------------+
```

### VM

好的,根据您提供的新的结构体定义,我可以继续绘制内存布局图。

``` asciidoc
+------------------+
| Chunk*chunk     | 8 bytes (64-bit system)
+------------------+
| uint8_t* ip      | 8 bytes (64-bit system)
+------------------+
| Value stack      | 2048 bytes (256 *8 bytes)
+------------------+
| Value* stackTop  | 8 bytes (64-bit system)
+------------------+

```

总共占用 2072 bytes 的内存空间。

其中:

Chunk*chunk 是一个指向 Chunk 结构体的指针。
uint8_t* ip 是一个指向无符号 8 位整数的指针,用作指令指针。
Value stack[STACK_MAX] 是一个大小为 256 的 Value 类型数组,用作栈。
Value* stackTop 是一个指向 Value 类型的指针,用作栈顶指针。
Value 类型是 double 类型,占用 8 bytes 的内存空间。

整个 VM 结构体总共占用 2072 bytes 的内存空间。
