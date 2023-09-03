# rust leanring

修改 配置文件 ~./.cargo/config

```yaml
[source.crates-io]
replace-with = 'rsproxy-sparse'
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"
[net]
git-fetch-with-cli = true
```

rust book: https://doc.rust-lang.org/book/ 

rust zh book: https://kaisery.github.io/
