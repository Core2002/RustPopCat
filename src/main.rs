extern crate winreg;
use winreg::enums::*;
use winreg::RegKey;
use std::fs;
use resource::resource;
use std::io;

fn main() {
    let mut buf = String::new();
    
    let cat_ico_full = resource!("cat_full.dll");
    let cat_ico_empty = resource!("cat_empty.dll");

    let file_cat_empty = "C:\\cat_empty.dll";
    let file_cat_full = "C:\\cat_full.dll";

    if fs::write(file_cat_empty,cat_ico_empty).is_err() {
        println!("请右键 -> 管理员身份运行");
        io::stdin().read_line(&mut buf).unwrap();
        panic!();
    }
    
    fs::write(file_cat_full, cat_ico_full).unwrap();

    let reg_key = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\CLSID\\{645FF040-5081-101B-9F08-00AA002F954E}\\DefaultIcon";

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let recyclebin_open = hkcu.open_subkey(reg_key).unwrap();
    let recyclebin_create = hkcu.create_subkey(reg_key).unwrap();

    println!("URL: https://github.com/Core2002/RustPopCat    By: 钟小白Core");
    println!("1 - 回收站变波波猫  \t  2 - 变回原来的样子");
    println!("\n请输入操作序号：");
    io::stdin().read_line(&mut buf).unwrap();
    match buf.as_mut_str().trim() {
        "1" => {
            recyclebin_create.0.set_value("empty", &format!("{}{}",file_cat_empty,",0")).unwrap();
            recyclebin_create.0.set_value("full", &format!("{}{}",file_cat_full,",0")).unwrap();
            println!("回收站变波波猫了");
        }
        "2" => {
            let default_dll = "%SystemRoot%\\System32\\imageres.dll";
            recyclebin_create.0.set_value("empty", &format!("{}{}",default_dll,",-55")).unwrap();
            recyclebin_create.0.set_value("full", &format!("{}{}",default_dll,",-54")).unwrap();
            println!("变回原来的样子了");
        }
        _ => {
            println!("输入错误，再来一次叭");
        }
    }
    
    let pop_empty:String = recyclebin_open.get_value("empty").unwrap();
    let pop_full:String = recyclebin_open.get_value("full").unwrap();
    println!("empty = {} , full = {}", pop_empty, pop_full);
    println!("友情提示：假如没反应就打开关闭然后多刷新几次");
    
    io::stdin().read_line(&mut buf).unwrap();
}

// HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\CLSID\{645FF040-5081-101B-9F08-00AA002F954E}\DefaultIcon
// empty = %SystemRoot%\System32\imageres.dll,-55 
//  full = %SystemRoot%\System32\imageres.dll,-54
