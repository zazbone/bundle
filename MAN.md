# MAN

## Bundle(1) - manipulate bundles of files and folders with metadata

### SYNOPSIS

`bundle` \[options\] command \[arguments\]

### DESCRIPTION

Bundle is a command-line tool that allows the manipulation of "bundles" of files and folders with metadata, providing a way to manage permanent files with unique names based on their characteristics.

### OPTIONS

The following options are available:

- `-h`, `--help`: Display the help message and exit.
- `-v`, `--verbose`: Display log INFO, WARNING and ERROR in stdout.
- `-V`: Display the version number and exit.

### COMMANDS

The following commands are available:

- `init`: Initializes a new bundle in the current directory (must be empty !). The command also generates a JSON configuration file in the bundle directory which will hold the bundle's files and metadata.

- `reserve [name] [metadata]` Reserves a new path name in the bundle with a unique ID based on the given metadata (hash of name + meta). And return it.

- `get [name] [metadata]`: Retrieves the path name of a file in the bundle that matches the specified metadata. If multiple files match the metadata, only the first one found will be returned.

- `delete [name] [metadata]`: Deletes a file in the bundle that matches the specified metadata. If multiple files match the metadata, throw an exception.

### ARGUMENTS

The following arguments can be used with the `reserve`, `get`, and `delete` commands:

- `name`: A string containing the name of the file to be reserved, retrieved, or deleted. The only non optionnal argument.

- `metadata`: A string containing key-value pairs of metadata attributes that describe the file to be reserved, retrieved, or deleted. The format of the metadata string is `key1=value1;key2=value2;...`.

### EXAMPLES

Initialize a new bundle:

```shell
$ bundle init
```

Reserve a path name in the bundle:

```shell
$ bundle reserve --name fit_results.csv --metadata "fit=response_model;iteration=n4"
/home/path/to/bundle/fit_results4853704354194759796.csv
```

get a previously reserved file in the bundle:
```shell
$ bundle get --name fit_results.csv --metadata "fit=response_model;iteration=n4"
/home/path/to/bundle/fit_results4853704354194759796.csv
```

Delete a file in the bundle:
```shell
$ bundle delete --name fit_results.csv --metadata "fit=response_model;iteration=n4"
```

### AUTHOR

Bundle was written by zazbone <coczaz@gmail.com>.

### REPORTING BUGS

Report bugs to the [GitHub issues page][GIT_ISSUES].

### COPYRIGHT

This program is free software: you can redistribute it and/or modify it under the terms of the MIT License. See the `LICENSE.txt` file for details.

### SEE ALSO
For more information, see the `README.md` file or visit the [Bundle GitHub repository][GIT].



[GIT]: https://github.com/zazbone/bundle
[GIT_ISSUES]: https://github.com/zazbone/bundle/issues
