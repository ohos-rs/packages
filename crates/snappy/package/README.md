# `@ohos-rs/snappy`

Fastest Snappy compression library in `OpenHarmony`, powered by `ohos-rs` and `rust-snappy`.

> 1. 由于现行NDK中`napi_typeof`方法对于空参处理问题会导致默认参数不填参数报错，所以请填写对应的参数。
> 2. @ohos.buffer表现跟Buffer不一致，优先建议使用string。
>
> 等待官方问题修复即可

## Install

use`ohpm` to install package.

```shell
ohpm install @ohos-rs/snappy
```

## API

```ts
export function compressSync(
    input: buffer.Buffer | string | ArrayBuffer | Uint8Array,
): Buffer
export function compress(
    input: buffer.Buffer | string | ArrayBuffer | Uint8Array,
): Promise<Buffer>
export function uncompressSync(compressed: buffer.Buffer): buffer.Buffer
export function uncompress(compressed: buffer.Buffer): Promise<buffer.Buffer>
```

## Usage

```ts
import { compressSync } from '@ohos-rs/snappy';

const a = compressSync("hello world", { copyOutputData: false });
// Buffer.from([11, 40, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]))
```

