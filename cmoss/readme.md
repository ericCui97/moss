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
