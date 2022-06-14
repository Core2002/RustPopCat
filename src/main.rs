extern crate winreg;
use winreg::enums::*;
use winreg::RegKey;
use resource::resource;
use std::fs;
use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let mut buf = String::new();
    
    let cat_ico_full = resource!("cat_full.dll");
    let cat_ico_empty = resource!("cat_empty.dll");

    let file_cat_empty = "C:\\cat_empty.dll";
    let file_cat_full = "C:\\cat_full.dll";

    if fs::write(file_cat_empty,cat_ico_empty).is_err() {
        println!("请右键 -> 管理员身份运行 (按 Enter 退出)");
        _ = io::stdin().read_line(&mut buf);
        std::process::exit(-1);
    }
    
    _ = fs::write(file_cat_full, cat_ico_full);

    let reg_key = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\CLSID\\{645FF040-5081-101B-9F08-00AA002F954E}\\DefaultIcon";

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let recyclebin_open = hkcu.open_subkey(reg_key).unwrap();
    let recyclebin_create = hkcu.create_subkey(reg_key).unwrap();

    println!("URL: https://github.com/Core2002/RustPopCat    By: 钟小白Core");
    println!("1 - 回收站变波波猫  \t  2 - 变回原来的样子");
    println!("\n请输入操作序号：");

    loop {
        io::stdin().read_line(&mut buf).unwrap();
        match buf.as_mut_str().trim() {
            "1" => {
                recyclebin_create.0.set_value("empty", &format!("{}{}",file_cat_empty,",0")).unwrap();
                recyclebin_create.0.set_value("full", &format!("{}{}",file_cat_full,",0")).unwrap();
                println!("回收站变波波猫了");
                break;
            },
            "2" => {
                let default_dll = "%SystemRoot%\\System32\\imageres.dll";
                recyclebin_create.0.set_value("empty", &format!("{}{}",default_dll,",-55")).unwrap();
                recyclebin_create.0.set_value("full", &format!("{}{}",default_dll,",-54")).unwrap();

                fs::remove_file(file_cat_empty).expect(&format!("欸? 删除失败了? 看来需要你自己删除 {} 了呢", file_cat_empty));
                fs::remove_file(file_cat_full).expect(&format!("欸? 删除失败了? 看来需要你自己删除 {} 了呢", file_cat_full));
                println!("回收站变回原来的样子了");
                break;
            },
            _ => {
                println!("输入错误，再来一次叭");
            }
        }
    }
    
    let pop_empty:String = recyclebin_open.get_value("empty").unwrap();
    let pop_full:String = recyclebin_open.get_value("full").unwrap();
    println!("empty = {} , full = {}", pop_empty, pop_full);
    println!("按 Enter 以重启资源管理器并退出");
    _ = io::stdin().read_line(&mut buf);

    let mut cmd = Command::new("cmd.exe")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .expect("无法启动 cmd.exe 以执行指令... 你可以运行 'taskkill -f -im explorer.exe && explorer.exe' 手动重启");

    let mut stdin = cmd.stdin.take().expect("failed to get stdin");

    stdin.write_all(b"taskkill -f -im explorer.exe & explorer.exe\n").expect("怪死了, 没办法执行指令");
    stdin.write_all(b"exit\n").expect("怪死了, 还没办法执行");
    
    _ = cmd.wait();
}

// HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\CLSID\{645FF040-5081-101B-9F08-00AA002F954E}\DefaultIcon
// empty = %SystemRoot%\System32\imageres.dll,-55 
//  full = %SystemRoot%\System32\imageres.dll,-54
