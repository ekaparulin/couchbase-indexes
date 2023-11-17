# couchbase-indexes
Couchbase index definition generator

Parses JSON output of `SELECT * from system:indexes;` to N1QL "CREATE INDEX" statements

## Build

`cargo build`

## Run

```
./target/debug/gen-indexes -h

Couchbase index definition generator 
Parses JSON output of `SELECT * from system:indexes;`
to N1QL statements

USAGE:
    gen-indexes [OPTIONS] <FILE>

ARGS:
    <FILE>    

OPTIONS:
    -b, --bucket <BUCKET>    Filter by bucket
    -d, --defer-build        Defer build
    -h, --help               Print help information
    -n, --if-not-exists      Add IF NOT EXISTS option
    -v, --verbose            Enabled verbose output
```

E.g:

- Run `SELECT * from system:indexes;` on Couchbase
- Save output as JSON
- Run `./target/debug/gen-indexes -d -n -b BUCKET > indexes.n1ql`