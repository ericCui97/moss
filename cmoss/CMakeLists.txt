cmake_minimum_required(VERSION 3.10)
project(cmoss)

# 添加可执行文件
add_executable(
    cmoss
main.c
chunk.c
memory.c
debug.c
)

# 设置C标准
set(CMAKE_C_STANDARD 11)

# 可选：如果需要特定的编译选项，可以添加以下行
# set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wall -Wextra -pedantic")

# 如果有需要的话，你可以添加其他配置，比如设置输出目录等
# set(EXECUTABLE_OUTPUT_PATH ${CMAKE_SOURCE_DIR}/bin)
