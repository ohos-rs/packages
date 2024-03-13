# `@ohos-rs/pinyin`

Fastest Chinese words to pinyin library for OpenHarmony.

> 由于现行鸿蒙NDK中`napi_typeof`方法对于空参处理问题会导致默认参数不填参数报错，所以请填写对应的参数。   
> 等待官方问题修复即可

## Install

use`ohpm` to install package.

```shell
ohpm install @ohos-rs/pinyin
```

## 用法

### 同步

```ts
import { pinyin } from '@napi-rs/pinyin'

console.log(pinyin('中心')) // [ [ 'zhōng' ], [ 'xīn' ] ]
```

### 异步

```ts
import { asyncPinyin } from '@napi-rs/pinyin'

asyncPinyin('中心').then(console.log.bind(console)) // [ [ 'zhōng' ], [ 'xīn' ] ]
```

### 参数

- **input** `<string>`

  需要转拼音的中文字符串

- **options?** `<Options>`

  转拼音参数

    - **Options.heteronym?** `<boolean>`

      是否处理多音字， 默认 `false`。如果为 `true`，返回类型为 `string[][]/Promise<string[][]>`, 如果为 `false` 返回类型为 `string[]/Promise<string[]>`

    - **Options.style?** `<PINYIN_STYLE>`

      拼音风格，默认为 `PINYIN_STYLE.WithTone`
      可选值为:

        - `Plain` 普通风格，不带声调

        - `WithTone` 带声调的风格

        - `WithToneNum` 声调在各个拼音之后，使用数字 1-4 表示的风格

        - `WithToneNumEnd` 声调在拼音最后，使用数字 1-4 表示的风格

        - `FirstLetter` 首字母风格

    - **Options.segment?** `<boolean>`

      是否开启分词。输入有多音字时，开启分词可以获得更准确结果。
