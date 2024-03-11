# `@ohos-rs/crc32`

Fastest `crc32` implement in `OpenHarmony`

> 1. 由于现行NDK中`napi_typeof`方法对于空参处理问题会导致默认参数不填参数报错，所以请填写对应的参数。
> 2. @ohos.buffer表现跟Buffer不一致，优先建议使用string。
>
> 等待官方问题修复即可

## Install

use`ohpm` to install package.

```shell
ohpm install @ohos-rs/crc32
```

## API

> The type of **input initial crc** and **output crc number** is `u32`

```ts
export function crc32(input: string | buffer.Buffer, crc?: number): number
export function crc32c(input: string | buffer.Buffer, crc?: number): number
```

## Usage

```ts
import { crc32 } from '@ohos-rs/crc32';

crc32("crc32c - test",0);
// 2608757237
```
