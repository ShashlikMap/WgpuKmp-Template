# WgpuKmp-Template
WGPU Rust and Kotlin Multiplatform application template with a shared Compose UI.

A motivation is to provide easy-to-setup mobile app project template with Rust WGPU backend as GPU graphics layer and Compose Multiplatform UI as a frontend part.

It's been achieved by using a combination of the following libraries/approaches:
- Kotlin/Compose Multiplatfom for a shared mobile Compose UI.
- [WGPU](https://github.com/gfx-rs/wgpu) for a cross-platform, Rust low-level graphics API.
- [uniffi-rs](https://github.com/mozilla/uniffi-rs) and [gobley](https://github.com/gobley/gobley) to connect Rust and KMP modules and provide a simple way to implememt/support API between languages/platforms.
- Partly borrowed implementation from [wgpu-in-app](https://github.com/jinleili/wgpu-in-app/) for a native GPU sufraces.

You can refer to [shashlik-map](https://github.com/ShashlikMap/shashlik-map) to have a look at more advanced example.

<img width="300" alt="Screenshot 2025-12-29 at 15 17 53" src="https://github.com/user-attachments/assets/66041510-a982-4288-ad06-9eb4e293205a" />
<img width="100" alt="Screenshot_20251229_151727" src="https://github.com/user-attachments/assets/68df0c9d-fcb3-4388-af4b-5ed4a57c05bd" />
<img width="100" alt="Simulator Screenshot - iPhone 17 Pro - 2025-12-29 at 15 16 01" src="https://github.com/user-attachments/assets/b4aac47f-f35c-4b5a-b0ac-a9ef95e457b2" />

## How to use template and run on macOs
Create and clone a new repository

<img width="142" height="41" alt="Screenshot 2025-12-29 at 15 57 27" src="https://github.com/user-attachments/assets/d1aa054c-1896-490b-ae28-714115f0479d" />

### winit Rust
```
cargo run --bin winit-run --release
```
P.S. Press "I" to increment counter.

### Android
- Open `kmp` folder as a project in Android Studio(wait for indexing/importing completed, may take a while)
- XCode might be needed to be installed(gradle will be compiling iOS as well)
- Make sure `composeApp` is selected
- Run `composeApp` as usual

_Note: initial importing may take up to 5-6 minutes, be patient_:)

### iOS
- The Android project from `kmp` folder has to be opened in AS at least once(gradle will create some extra files required to build/run iOS app)
- Open `kmp/iosApp` folder as a project in XCode
- Run as usual on iOS Simulator or real device

## Roadmap
- Simplify an internal configuration even more
- Add a Compose Desktop

## Extra
If you're looking for a pure Android WebGPU template(using androidx.webgpu) you can find it [here](https://github.com/ShashlikMap/webgpu-kt-template)
