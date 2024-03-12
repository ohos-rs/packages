# `@ohos-rs/bcrypt`

🚀 Fastest bcrypt in OpenHarmony.

> 1. 由于现行NDK中`napi_typeof`方法对于空参处理问题会导致默认参数不填参数报错，所以请填写对应的参数。
> 2. @ohos.buffer表现跟Buffer不一致，优先建议使用string。
>
> 等待官方问题修复即可

## Install

use`ohpm` to install package.

```shell
ohpm install @ohos-rs/bcrypt
```

## API

```ts
export const DEFAULT_COST: 12

export function hashSync(password: string | Buffer, round?: number): string
export function hash(password: string | Buffer, round?: number): Promise<string>
export function verifySync(password: string | Buffer, hash: string | Buffer): boolean
export function verify(password: string | Buffer, hash: string | Buffer): Promise<boolean>
/**
 * The same with `verifySync`
 */
export function compareSync(password: string | Buffer, hash: string | Buffer): boolean
/**
 * The same with `verify`
 */
export function compare(password: string | Buffer, hash: string | Buffer): Promise<boolean>

export type Version = '2a' | '2x' | '2y' | '2b'
/**
 * @param version default '2b'
 */
export function genSaltSync(round: number, version?: Version): string
/**
 * @param version default '2b'
 */
export function genSalt(round: number, version?: Version): Promise<string>
```
