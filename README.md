# sqlite-base64

A base64 encoder/decoder for SQLite, written in Rust. See [`sqlite-loadable-rs`](https://github.com/asg017/sqlite-loadable-rs), the framework that makes this extension possible.
md).

## WORK IN PROGRESS

This extension isn't 100% complete yet, but hoping to release soon! Once it's ready, you'll be able to do things like:

```sql
select base64_decode('YWxleA=='); -- 'alex'
select base64_encode('angel'); -- 'YW5nZWw='
```

In the meantime, checkout the very-new and more offical [base64.c](https://github.com/sqlite/sqlite/blob/master/ext/misc/base64.c) SQLite extension.
