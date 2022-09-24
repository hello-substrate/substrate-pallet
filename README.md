# offchain-worker 练习

## ppt

[ppt](./offchain-worker文档.pptx)
[code](https://github.com/miketang84/oneblockplus_offchain_lesson)

## 知识点

### Offchain Features 三大组件

- https://docs.substrate.io/fundamentals/offchain-operations/
- Offchain Worker
- Offchain Storage
- Offchain Indexing

### 补充说明

- Offchain Worker可以直接读链上的数据（状态），但不能写
- 链上代码可以向 Offchain Storage中写数据，但不能读
- 外层 node 和 rpc 可以直接读链上存储的数据，和Offchain Storage中存储的数据
    - api.rpc.offchain.localStorageGet
- 外层 node 和 rpc 可以直接向 Offchain Storage 中写数据，但不能直接向链上存储写数据（必须通过发交易更改链上状态）
    - api.rpc.offchain.localStorageSet

## offchain_worker 打印日志

- 运行: `make 1`
- 给 pallet 添加 offchain worker hook
- 在 offchain worker 中打印信息，观察日志出现的时机

## offchain_worker 的生命周期

### 理解 offchain worker 的执行时机

- 运行: `make 2`
- 同时打开多个hooks，on_initialize, on_finalize, on_idle
- 在各个hooks中打印信息，观察日志出现的时机，理解 offchain worker 的执行时机

### 观察 offchain worker的跨块执行效果

- `make 3`
- 在offchain worker中sleep一段时间，观察 offchain worker的跨块执行效果

## Local Storage 存储

> - Offchain Worker 可直接读写Local Storage
> - 链上代码可通过 Offchain Indexing 功能直接向Local Storage写入数据，但是不能读
> - 可用于Offchain Worker tasks之间的通信和协调，注意由于可能同时存在多个Offchain worker，因此可能对存储的访问需要lock

- `make 4`
- 设计为在奇数块向 Local Storage 写数据，偶数块读取数据，并检查
- 可以学到：如何获取链下随机数，如何对BlockNumer类型进行数学运算，如何获取链下时间，如何生成存储key，如何写链下存储，如何读链下存储，如何清理存储key

## mutate方法对数据进行原子更改

- `make 5`
- 在示例4的基础上，使用mutate方法对数据进行原子更改
- 可以学到：新的原子操作修改方法（不再使用之前手动锁的方式），学习配套的错误处理模式



