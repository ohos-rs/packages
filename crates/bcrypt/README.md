# `@ohos-rs/bcrypt`

🚀 Fastest bcrypt in OpenHarmony.

## Install

use`ohpm` to install package.

```shell
ohpm install @ohos-rs/bcrypt
```

## API

```ts
export const DEFAULT_COST: number = 12;

export function genSalt(round: number, version?: '2a' | '2x' | '2y' | '2b', signal?: AbortSignal): Promise<string>

export function genSaltSync(round: number, version?: '2a' | '2x' | '2y' | '2b'): string

export function hash(input: Uint8Array | string, cost?: number | undefined | null, salt?: string | Uint8Array | undefined | null, version?: '2a' | '2x' | '2y' | '2b', signal?: AbortSignal | undefined | null): Promise<string>

export function hashSync(input: string | Uint8Array, cost?: number | undefined | null, salt?: string | Uint8Array | undefined | null, version?: '2a' | '2x' | '2y' | '2b'): string

export function verify(password: Uint8Array | string, hash: Uint8Array | string, signal?: AbortSignal | undefined | null): Promise<boolean>

export function verifySync(input: string | Uint8Array, hash: string | Uint8Array): boolean
```


## Abort

If you want to abort the task and you can use `@ohos-rs/abort-controller`

```ts
import { AbortController } from '@ohos-rs/abort-controller'

const controller = new AbortController();

genSalt(10, undefined, controller.signal).then((res) => {
  console.log(res);
}).catch((err) => {
  console.log(err);
})

controller.abort();
```
