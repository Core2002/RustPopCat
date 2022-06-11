extern crate winreg;
use winreg::enums::*;
use winreg::RegKey;
use std::fs;
use resource::resource;
use std::io;

fn main() {
    let cat_ico_full = resource!("cat_full.dll");
    let cat_ico_empty = resource!("cat_empty.dll");

    let file_cat_empty = "D:\\cat_empty.dll";
    let file_cat_full = "D:\\cat_full.dll";

    fs::write(file_cat_empty,cat_ico_empty).unwrap();
    fs::write(file_cat_full, cat_ico_full).unwrap();

    let reg_key = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\CLSID\\{645FF040-5081-101B-9F08-00AA002F954E}\\DefaultIcon";

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let recyclebin_open = hkcu.open_subkey(reg_key).unwrap();
    let recyclebin_create = hkcu.create_subkey(reg_key).unwrap();
    
    let pop_empty:String = recyclebin_open.get_value("empty").unwrap();
    let pop_full:String = recyclebin_open.get_value("full").unwrap();

    recyclebin_create.0.set_value("empty", format!("{}{}",file_cat_empty,",0")).unwrap();
    recyclebin_create.0.set_value("full", format!("{}{}",file_cat_full,",0")).unwrap();

    println!("empty = {} \n full = {}", pop_empty, pop_full);
    let mut buf = String::new();
    io::stdin().read_line(&mut buf);
}

// HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\CLSID\{645FF040-5081-101B-9F08-00AA002F954E}\DefaultIcon
// empty = %SystemRoot%\System32\imageres.dll,-55 
//  full = %SystemRoot%\System32\imageres.dll,-54
