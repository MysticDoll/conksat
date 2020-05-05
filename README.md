# conksat

conksat means `Convert Key/value struct to Any Template`

## Build

```bash
$ git clone https://github.com/MysticDoll/conksat && cd conksat
$ cargo build --release
$ cp target/release/conksat /path/to/install_dir
```

## Usage
```
conksat 1.0
Mitsuru Takigahira <mysticdoll@mysticdoll.com>
Convert key/value struct to any template

USAGE:
    conksat [OPTIONS] --config <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>      Sets a custom config file
    -s, --source <source>    Sets the input file to use

```
### Prepare convert specs

Write specs like below

```yaml

target: "JSON"
vars:
  - name: hoge_fuga_value
    key: .hoge.fuga
  - name: hoge_foo_value
    key: .hoge.foo
template: |
  {
    "hoge_fuga": "{{ hoge_fuga_value }}"
    "hoge_foo": "{{ hoge_foo_value }}"
  }
```

- `target` means the format of input source. `conksat` interprets input as specified format. `JSON` or `YAML` is now supported.
- `vars` is the collection of variables. `conksat` gets the value correspond to `key` from input and store it.
- `template` is the template to output. `conksat` converts `{{ <name> }}` to the value of variable named `<name>`.

### execute

```bash
$ echo '{"hoge": {"fuga": 1, "foo": 2}}' | conksat -c ./spec.yaml
{
  "hoge_fuga": "1"
  "hoge_foo": "2"
}
```
