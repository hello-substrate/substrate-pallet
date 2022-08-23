# 链下工作者
- 包含
  1. ocw 发送已签名的交易
  2. 发送未签名的交易
  3. 发送带有签名`payloads`的未签名交易
  4. http 读取和解析 json 数据

- 参考

https://mp.weixin.qq.com/s/qYwontALTt0hHsr5hKV9zg
https://github.com/JoshOrndorff/recipes/blob/master/text/off-chain-workers/transactions.md
https://docs.substrate.io/reference/how-to-guides/offchain-workers/offchain-transactions/

## 设置 offchainworker 中的签名账户用来提交交易(需要有余额来支付手续费哟)

- 示例账户(Alice可给转入金额)

```
Secret phrase:       boat stick victory lazy teach science summer outdoor sugar royal message great
  Network ID:        substrate
  Secret seed:       0xf97b00cba1d31684c599d7802d8337b158bd7d1f676da1573d2711ce87500a85
  Public key (hex):  0x6c3b2c14bab0c58bdff69a8c7169577c3ed0e480b1f99e34ea45aa84c43ece3a
  Account ID:        0x6c3b2c14bab0c58bdff69a8c7169577c3ed0e480b1f99e34ea45aa84c43ece3a
  Public key (SS58): 5EWcc8bAXM7ek6n1trvf1t8BqitrqxZfYfnjG8NKSrR3Su5C
  SS58 Address:      5EWcc8bAXM7ek6n1trvf1t8BqitrqxZfYfnjG8NKSrR3Su5C

```

进行insert key 设置 offchainworker中提交交易账户.需要密钥和私钥

### 第一种是使用polkadot-js-app

- 选择开发者->rpc calls -> author -> insertKey
- 参数

```
keyType= demo (自己设置的`pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");`)
suri= boat stick victory lazy teach science summer outdoor sugar royal message great
publicKey= 0x6c3b2c14bab0c58bdff69a8c7169577c3ed0e480b1f99e34ea45aa84c43ece3a
```

### 第二种是使用curl发送rpc请求

`curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d  '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["demo", "此处换成私钥", "此处换成公钥"] }'`