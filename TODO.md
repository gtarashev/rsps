# TODO
A todo list for what else needs to be done (will add more whenever I think of them and word them appropriately). Most likely, when the list is complete, then the shell will be complete as well.

*The list is not ordered*

[ ] Separate builtin and other commands
    - store all builtin functions (probably enum holding function pointers)
    - make the `which` command work correctly
[ ] Create a parser for the commands
    - can store and expand variables
    - parser returns error for incorrectly formatted statements (commands not run)
[ ] Buffer streams
    - can use tab completion
    - can redirect streams from/into eachother/files
[ ] Docummentation
    - add `rustdoc` to functions, enums, structs, etc.
[ ] Tests
