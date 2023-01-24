# Implementing TodoMVC

This folder contains the source files for the [TodoMVC](https://todomvc.com/) example, you
can see a running example following [this link](https://todomvc.com/examples/react/#/).

To start the project locally, please run the following command:

```sh
trunk serve
```

This command is watching all the files in the project and will recompile whenever a file is saved.

## Extract components

- Create new todo input
- Todo list input

## Load a list of static todos from a state

You could use a state struct like the following one. It will be simpler to pass it down
to a child component.

```rust
struct TodoListState {
    todos: Vec<Todos>
}
```

