use moss::interpreter::Interpreter;
use moss::parser::Parser;
use moss::scanner::Scanner;
use uuid::Uuid;
use std::{fs, io::Write, process::exit};

use crate::common::cli::run_file_via_command;
#[allow(dead_code)]
pub fn run_file(path: &str) {
    let mut interpreter = Interpreter::new();
    match fs::read_to_string(path) {
        Ok(contents) => {
            run(&mut interpreter, &contents).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
            exit(74);
        }
    }
}

pub fn run(interpreter: &mut Interpreter, source: &str) -> Result<(), String> {
    let scan = &mut Scanner::new(source);
    let tokens = scan.scan_tokens()?;
    let parser = Parser::new(tokens);
    let stmts = parser.parse()?;
    interpreter.interpret(&stmts)?;
    Ok(())
}


pub fn run_test_file(path:&str)->Result<(),String>{
    // run_file("tests/common/file.moss");
    // read file
    let fs_string = fs::read_to_string(path).unwrap();
    // 读取 第一行以---test开头的行 ---test tc1 tc1 是测试编号
    let mut test_name = String::new();
    // 读取 内容 直到 ---expect
    let mut source = String::new();
    let mut expect = Vec::<String>::new();
    let mut is_expect = false;
    for line in fs_string.lines(){
        if line.starts_with("---test"){
            test_name = line.split_whitespace().last().unwrap().to_string();
            continue;
        }
        if line.starts_with("---expect"){
            is_expect = true;
            continue;
        }
        if is_expect{
            expect.push(line.to_string());
        }else{
            source.push_str(line);
            source.push('\n');
        }
    }

    // 创建一个临时文件，将source写入
    let uuid: Uuid = Uuid::new_v4();
    let temp_file: fs::File = fs::File::create(format!("debug_file/{}.moss",uuid)).unwrap();
    let mut writer: std::io::BufWriter<fs::File> = std::io::BufWriter::new(temp_file);
    writer.write_all(source.as_bytes()).unwrap();

    println!("running test-------- {} --------test case: {}",path,test_name);
    let result_line: Vec<String> = run_file_via_command(format!("debug_file/{}.moss",uuid).as_str()).unwrap();
    println!("result: {:?}",result_line.len());
    // 比对
    for (i, line) in result_line.iter().enumerate(){
        if line != &expect[i]{
            println!("Error: test case {} failed, expect: {}, got: {}",test_name,expect[i],line);
            return Err("test failed".to_string());
        }
    }
    if result_line.len() != expect.len(){
        println!("Error: test case {} failed, expect: {:?}, got: {:?}",test_name,expect,result_line);
        return Err("test failed".to_string());
    }
    println!("test case {} passed",test_name);


    // delete temp file
    fs::remove_file(format!("debug_file/{}.moss",uuid)).unwrap();

    Ok(())
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_run_test_file(){
//         assert!(run_test_file("test_file/test_for.moss").is_ok());
//         // 断言 run_test_file 不会抛出异常
        
//     }
// }