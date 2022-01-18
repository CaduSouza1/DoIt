# DoIt
A command-line based todo-list app implemented using the rust language

# Installation
If you already have a Rust environment in your machine, you can install this program using cargo:
```
$ cargo install doit
```

# Usage
To add a list:
```
$ doit add list -l <List name>
```

This command Will create a list with the desired name and save it to the tasks.json file
```
$ doit add task -t <Task title> -l <List name>
```
This will add the task to that specified list and save it to the tasks.json file.

## Defaults
For now, there is no configurable defaults for task information, only for filepaths. In the near future, a way to add and change defaults will be added, either in the form of a json file or a source code file compiled directly in the program 

# Commands
- add - alias: a
- show - alias: s
- remove - alias: r
- complete - alias: c

## Add
### Available subcommands for add
- task
```
$ doit add task -t <Task Title> -d <Task Description> -l <List where the task will be put> -i <Index where the task will be placed>
```
- list
```
$ doit add list -l <List name> -i <Index where the list will be placed>
```

## Show
### Available subcommands for show
- list
```
$ doit show list -l <List name> -i <Index where the list will be deleted>
```
- all
```
$ doit show all
```

## Remove
### Available subcommands for remove
- task
    - All the options are used to filter out similar tasks
    - The index option will always take precedence over the title and description options
```
$ doit remove task -t <Task Title> -d <Task Description> -l <List where the task will be deleted> -i <Index where the task will be deleted>
```
- list
```
$ doit remove list -l <List name> -i <Index where the list will be deleted>
```


## Complete
### Available subcommands for 
- task
    - All the options are used to filter out similar tasks
    - The index option will always take precedence over the title and description options
```
$ doit complete task -t <Task Title> -d <Task Description> -l <List where the task will be completed> -i <Index where the task will be completed> 
```
- list
```
$ doit complete list -l <List name> -i <Index where the list will be completed>
```


# Command-line options
- -r, --read-file: sets the file where the tasks will be read from
- -w, --write-file: sets the file where the tasks will be wrote to
- -f, --file: sets the file where the tasks will be both read and wrote to. This options does not override the other options

