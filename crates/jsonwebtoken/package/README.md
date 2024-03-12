# `@ohos-rs/jsonwebtoken`

🚀 Fastest jsonwebtoken in OpenHarmony

> 由于现行鸿蒙NDK中`napi_typeof`方法对于空参处理问题会导致默认参数不填参数报错，所以请填写对应的参数。   
> 等待官方问题修复即可

## Install

use`ohpm` to install package.

```shell
ohpm install @ohos-rs/jsonwebtoken
```

## Usage

```javascript
  const iat = getUtcTimestamp()
const claims = {
    data: {
        id: 'f81d4fae-7dec-11d0-a765-00a0c91e6bf6',
        pr: 33,
        isM: true,
        set: ['KL', 'TV', 'JI'],
        nest: { id: 'poly' },
    },
    exp: iat + oneDayInSeconds,
    iat,
}
const secretKey = 'secret'
const resSync = signSync(claims, secretKey)
const resAsync = await sign(claims, secretKey)
```
