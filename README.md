### todo

This was the first thing I created in rust.

By default, a global todo file will be created and accessed in `~/.config/todo` 

A seperate todo file can be created in each directory for individual things or projects


#### commands
* todo        => list todos
* todo new    => create todo in current directory
* todo rm x   => remove todo in xth index (1 indexed)
* todo rm all => remove todo in current directory
* todo rm x-y => remove all todos in range x-y

### installing

```
cargo build --release
sudo cp target/release/todo /usr/local/bin

```
