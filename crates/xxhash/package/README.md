# `@ohos-rs/xxhash`

[`xxhash-rust`](https://github.com/DoumanAsh/xxhash-rust) binding for OpenHarmony.

> 1. 由于现行NDK中`napi_typeof`方法对于空参处理问题会导致默认参数不填参数报错，所以请填写对应的参数。
> 2. @ohos.buffer表现跟Buffer不一致，优先建议使用string。
>
> 等待官方问题修复即可

## Install

use`ohpm` to install package.

```shell
ohpm install @ohos-rs/xxhash
```

## API

```ts
export type BufferLike =
  | Buffer
  | string
  | Uint8Array
  | ArrayBuffer
  | SharedArrayBuffer
  | ReadonlyArray<number>
  | number[]

export function xxh32(input: BufferLike, seed?: number): number
export function xxh64(input: BufferLike, seed?: BigInt): BigInt

export class Xxh32 {
  constructor(seed?: number)
  update(input: BufferLike): this
  digest(): number
  reset(): void
}

export class Xxh64 {
  constructor(seed?: BigInt)
  update(input: BufferLike): this
  digest(): BigInt
  reset(): void
}

export class Xxh3 {
  static withSeed(seed?: BigInt): Xxh3
  static withSecret(secret: BufferLike): Xxh3
  private constructor() {}
  update(input: BufferLike): this
  digest(): BigInt
  reset(): void
}

export const xxh3: {
  xxh64: (input: BufferLike, seed?: BigInt) => BigInt
  xxh64WithSecret: (input: BufferLike, secret: BufferLike) => BigInt
  xxh128: (input: BufferLike, seed?: BigInt) => BigInt
  xxh128WithSecret: (input: BufferLike, secret: BufferLike) => BigInt
  Xxh3: typeof Xxh3
}
```
