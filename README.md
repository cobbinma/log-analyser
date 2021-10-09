<div align="center">
<h1 align="center">log-analyser</h1>
 <strong>
   cli tool to analyse logs
 </strong>
</div>

## install

[rust](https://www.rust-lang.org/tools/install) is required to install log-analyser

install log-analyser using the command:

```shell
cargo install --path .
```

## run

run the help command to see options:

```shell
log-analyser --help
```

example:

```shell
log-analyser -i input.log
```

output:

```shell
+------+----------------+-----------------+
| Type | Total Messages | Total Byte Size |
+=========================================+
| A    | 1              | 25              |
|------+----------------+-----------------|
| B    | 2              | 73              |
+------+----------------+-----------------+
```

## development

### run

```shell
cargo run -- --help
```

### test

```shell
cargo test
```