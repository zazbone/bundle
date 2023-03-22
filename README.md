# Bundle

## Introduction

Bundle is a small library and command-line application that allows for the manipulation of "bundles" of files and folders with a set of attributes, or metadata, rather than a traditional file system. It is similar to the mktemp Linux command-line tool, but with permanent files instead of temporary ones. Bundle adds a hash or a unique ID to the file names to ensure that they remain distinct and returns this ID at file creation. It can also search for an existing file with the same characteristics later.

It is currently in the early stages of development, with planned features including:

- [ ] Initializing a bundle folder 
    - [x] JSON configuration file
    - [ ] Shell environment script generation
- [ ] Reserving a path name in the bundle
- [ ] Secure access to namespace and metadata
- [ ] Retrieving a path name that matches given metadata
- [ ] Deleting a path name that matches given metadata
- [ ] Interfacing with other languages;
    - [ ] C abi
    - [ ] C++ 
    - [ ] Python
- [ ] MAKE IT SECURE
- [ ] Asynchronous server version ??


Bundle aims to stay minimalistic, with advanced features such as querying metadata and listing files left to the responsibility of extensions or other software.

## Goals

Bundle aims to solve a common problem encountered during data analysis: the accumulation of multiple input and output files during model development. Currently, this problem is solved by creating files with verbose names to differentiate them. Bundle offers a potential solution where all differences between file parameters can be stored as metadata and queried later.

We hope that Bundle will simplify the process of managing multiple files in a project and make it easier to keep track of important information about each file.


*Thanks chatGPT*