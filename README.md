<h1 align="center">😽 Rust Pop Cat Recycle Bin</h1>
<h5 align="center">一只Rust写的用来把回收站变成pop猫(啵啵猫)的小工具</h5>

------

## 项目简介

RustPopCat 是一个使用 `Rust` 编写的简单工具，可以将 `Windows` 回收站图标更改为波波猫图标，并且可以恢复到默认状态。此工具需要管理员权限才能运行。

该项目采用了 `Tauri` + `Vue 3` + `TypeScript` 的现代架构，结合了 Tauri 的跨平台能力、Vue 3 的高性能和 TypeScript 的静态类型检查，提供了更好的开发体验和更高的代码质量。

## 功能

- 将回收站图标更改为波波猫图标。
- 恢复回收站图标为默认状态。
- 支持通过命令行参数直接操作。

## 使用方法

### 图形化操作

1. 运行程序
2. 根据提示单击 "更改图标" 或 "恢复图标" 按钮，来选择您的操作

### 命令行参数

- `rust_pop_cat.exe 1`：将回收站图标更改为波波猫图标。
- `rust_pop_cat.exe 0`：恢复回收站图标为默认状态。

## 依赖项

- `winreg`：用于操作 Windows 注册表。
- `resource`：用于加载资源文件。

## 构建方法

1. 确保已配置 `Rust`、`Tauri`、`Visual Studio C++ Build Tools` 工具链。
2. 克隆仓库：

    ```sh
    git clone https://github.com/Core2002/RustPopCat.git
    cd RustPopCat
    ```

3. 构建项目：

    ```sh
    yarn tauri build
    ```

4. 运行项目：

    ```sh
    src-tauri/target/release/rust_pop_cat.exe
    ```

## 注意事项

- 本工具需要管理员权限才能运行。请右键点击可执行文件并选择“管理员身份运行”。
- 操作完毕后若未自动刷新，请手动右键刷新桌面。
- 该程序为Windows程序，需要在Windows环境下构建。

## 许可证

本项目采用 `MIT` 许可证，详情见 [LICENSE](./LICENSE) 文件。
