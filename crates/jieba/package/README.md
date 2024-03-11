# `@ohos-rs/jieba`

[jieba-rs](https://github.com/messense/jieba-rs) binding to HarmonyOS

> 由于现行鸿蒙NDK中`napi_typeof`方法对于空参处理问题会导致默认参数不填参数报错，所以请填写对应的参数。   
> 等待官方问题修复即可

## Install

use`ohpm` to install package.

```shell
ohpm install @ohrs-rs/jieba
```

## Usage

```javascript
import jieba from '@ohos-rs/jieba';

jieba.load()
// loadDict(fs.readFileSync(...))
// loadTFIDFDict(fs.readFileSync(...))

jieba.cut('我们中出了一个叛徒', false)

// ["我们", "中", "出", "了", "一个", "叛徒"]
```

```javascript
import jieba from '@ohos-rs/jieba';

jieba.load()

jieba.extract(
    '今天纽约的天气真好啊，京华大酒店的张尧经理吃了一只北京烤鸭。后天纽约的天气不好，昨天纽约的天气也不好，北京烤鸭真好吃',
    3,
)

// [
//   { keyword: '北京烤鸭', weight: 1.3904870323222223 },
//   { keyword: '纽约', weight: 1.121759684755 },
//   { keyword: '天气', weight: 1.0766573240983333 }
// ]
```
