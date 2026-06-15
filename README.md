# Studio Display Brightness Controller

用于在 Windows 上控制 Apple Studio Display 亮度的 Tauri 2 + Rust 桌面应用。

当前 MVP 提供主窗口亮度滑杆、常用亮度预设、Studio Display 设备枚举和系统托盘预设入口。当前切片只写入亮度，不读取显示器真实当前亮度。

## 开发

- `npm install`
- `npm run build`
- `npm run tauri dev`

## 打包

- `npm run tauri build`

Windows 默认生成 NSIS 安装包：

- `src-tauri/target/release/bundle/nsis/StudioDisplayBrightnessController_0.1.0_x64-setup.exe`

## 验证

- `npm run build`
- `npm run test:slider`
- `Push-Location src-tauri; cargo test --lib; cargo check; cargo clippy --lib -- -D warnings; Pop-Location`

## 许可证

本项目使用 MIT License，详见 `LICENSE`。
