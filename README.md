# ohos_rust_napi_demo

使用`ohos-rs`编译`Rust`代码输出`OpenHarmony`库.

- [hos-rs](https://ohos.rs/)

# Rust 侧

## 安装`ohrs`插件

```shell
cargo install ohrs
```

## 初始化`Rust`工程

```shell
ohrs init rust_hello
```

## 编译产物

编译之前需要配置交叉编译环境

```shell
rustup target add aarch64-unknown-linux-ohos
rustup target add armv7-unknown-linux-ohos
rustup target add x86_64-unknown-linux-ohos
```

需要配置环境变量`OHOS_NDK_HOME`:

```shell
# https://ohos.rs/docs/basic/quick-start.html#%E5%AE%89%E8%A3%85
export OHOS_NDK_HOME=/Applications/DevEco-Studio.app/Contents/sdk/default/openharmony
```

构建:

```shell
ohrs build --release --arch aarch --arch x64 --arch arm --dist=dist

# https://ohos.rs/docs/cli/build.html
# 构建. 默认输出产物在 `./dist/` 目录下.
# dist
# ├── arm64-v8a
# │   └── librust_hello.so
# ├── armeabi-v7a
# │   └── librust_hello.so
# ├── index.d.ts
# └── x86_64
#     └── librust_hello.so
```

产物还少了一个`oh-package.json5`配置文件, 需要手动创建, 后面会介绍.

# ArkTS 侧

## 复制产物到`ArkTS`工程

将`Rust`产物`.so`文件复制到`ArkTS`工程`(entry)/libs`下(按照编译器架构分类存放).

## 创建`oh-package.json5`配置文件

将`Rust`产物`index.d.ts`文件复制到`ArkTS`工程`(entry)/src/main/cpp/types/(libRustHello)`下(任意路径都可以).

同时在此目录下创建`oh-package.json5`配置文件:

```json5
{
  "name": "libRustHello.so", // 任意名称都可以
  "types": "./index.d.ts",
  "version": "1.0.0",
  "description": "rust hello."
}
```

## 配置依赖

在`ArkTS`工程`(entry)/oh-package.json5`文件中加入依赖:

```json5
{
  "name": "entry",
  "version": "1.0.0",
  "description": "Please describe the basic information.",
  "main": "",
  "author": "",
  "license": "",
  // ...
  "dependencies": {
    // ...
    // The declared dependency name should match the name of the referenced SO library
    "libRustHello.so": "file:./src/main/cpp/types/libRustHello" // 这里`key`必须是`oh-package.json5`配置文件中的`name`.
  }
}
```

## 使用

导入对应的`librust_hello.so`文件即可获取到对应的API.

```typescript
import { xxx } from 'librust_hello.so'; // 这里必须要使用和`.so`文件一样的名字.
```

- [flutter_rust_ffi](https://github.com/angcyo/flutter_rust_ffi)
- [ohos_rust_napi_demo](https://github.com/angcyo/ohos_rust_napi_demo)

---
**群内有`各(pian)种(ni)各(jin)样(qun)`的大佬,等你来撩.**

# 联系作者

[点此QQ对话](http://wpa.qq.com/msgrd?v=3&uin=664738095&site=qq&menu=yes)  `该死的空格`    [点此快速加群](https://shang.qq.com/wpa/qunwpa?idkey=cbcf9a42faf2fe730b51004d33ac70863617e6999fce7daf43231f3cf2997460)

[开源地址](https://github.com/angcyo/DslTabLayout)

![扫码进群](https://raw.githubusercontent.com/angcyo/res/master/code/all_in1.jpg)

![给点鼓励](https://raw.githubusercontent.com/angcyo/res/master/code/all_in2.jpg)
