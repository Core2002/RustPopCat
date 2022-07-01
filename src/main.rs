extern crate winreg;
use winreg::enums::*;
use winreg::RegKey;
use resource::resource;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut buf = String::new();

    if !check_permissions_and_release_files() {
        if args.len() > 1 {
            println!("错误：需要管理员权限");
        }else {
            println!("请右键 -> 管理员身份运行 (按 Enter 退出)");
            _ = io::stdin().read_line(&mut buf);
        }
        std::process::exit(-1);
    }

    if args.len() > 1 {
        pop_cat(&args[1]);
        return;
    }

    println!("URL: https://github.com/Core2002/RustPopCat    By: 钟小白Core");
    println!("1 - 回收站变波波猫  \t  2 - 变回原来的样子");
    println!("\n请输入操作序号：");

    loop {
        io::stdin().read_line(&mut buf).unwrap();
        match buf.as_mut_str().trim() {
            "1" => {
                pop_cat("1");
                break;
            },
            "2" => {
                pop_cat("0");
                break;
            },
            _ => {
                println!("输入错误，再来一次叭");
            }
        }
    }

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


static FILE_CAT_EMPTY: &str = "C:\\cat_empty.dll";
static FILE_CAT_FULL: &str = "C:\\cat_full.dll";

fn check_permissions_and_release_files() -> bool {
    let cat_ico_full = resource!("cat_full.dll");
    let cat_ico_empty = resource!("cat_empty.dll");

    if fs::write(FILE_CAT_EMPTY,cat_ico_empty).is_err() {
        return false;
    }
    _ = fs::write(FILE_CAT_FULL, cat_ico_full);

    return true;
}

fn pop_cat(operate: &str) {
    let reg_key = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\CLSID\\{645FF040-5081-101B-9F08-00AA002F954E}\\DefaultIcon";

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let recyclebin_create = hkcu.create_subkey(reg_key).unwrap();

    match operate.trim() {
        "1" => {
            recyclebin_create.0.set_value("empty", &format!("{}{}",FILE_CAT_EMPTY,",0")).unwrap();
            recyclebin_create.0.set_value("full", &format!("{}{}",FILE_CAT_FULL,",0")).unwrap();
            println!("回收站变波波猫了");
        },
        "0" => {
            let default_dll = "%SystemRoot%\\System32\\imageres.dll";
            recyclebin_create.0.set_value("empty", &format!("{}{}",default_dll,",-55")).unwrap();
            recyclebin_create.0.set_value("full", &format!("{}{}",default_dll,",-54")).unwrap();

            fs::remove_file(FILE_CAT_EMPTY).expect(&format!("欸? 删除失败了? 看来需要你自己删除 {} 了呢\n", FILE_CAT_EMPTY));
            fs::remove_file(FILE_CAT_FULL).expect(&format!("欸? 删除失败了? 看来需要你自己删除 {} 了呢\n", FILE_CAT_FULL));
            println!("回收站变回原来的样子了");
        },
        _ => {
            println!("帮助 - 命令行参数如下\n1 => 变波波猫\n0 => 还原默认");
        }
    }
}

// HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\CLSID\{645FF040-5081-101B-9F08-00AA002F954E}\DefaultIcon
// empty = %SystemRoot%\System32\imageres.dll,-55 
//  full = %SystemRoot%\System32\imageres.dll,-54
