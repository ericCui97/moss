#pragma once

#include "chunk.h"
/**
 * 把一个chunk中的指令打印出来
 * @param chunk chunk指针
 * @param name chunk的名字
 */
void disassemble_chunk(Chunk* chunk,const char* name);
int disassemble_instruction(Chunk* chunk,int  offset);