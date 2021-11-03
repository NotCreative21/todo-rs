## todo

This tiny program is one of the first things I've coded in rust that I actually use. Only tested to work on linux because that is what I [use](https://github.com/NotCreative21/dotfiles).

To list out todos simply type `todo` in the terminal, to add a todo type `todo arg`, to remove a todo type `todo rm x` where x is the number which corresponds with the todo you'd like to remove.

### installing

Edit the variable `file` on line 31 of main.rs to match where you'd like the todo file to be and what you'd like it to be named. (In this case the file name is set to .todo)

Next you need to save and build, to build simply run `cargo build --release`. 

Finally, copy the todo binary from ./target/release to a folder that is in your path.
