# Test Builder

Simple command-line utility to create a test directory which mirrors your source directory.  Obviously this should never be needed as everyone writes their tests first...

Hopefully a inital stepping stone, to building a background-task framework which can dynamically create test files as you go along.

**Note**: Currently a little hard-coded/optimised for python directories

## Usage

You can clone, build and run anyway you want (either local target, or symlink to your binaries)

The binary takes 2 optional cli args:

- Source path: `-s`, `--source` which corresponds to the source directory you want to traverse and mirror (defaults to `./src`)
- Test path: `-t`, `--test` which corresponds to the tests directory you want to build (defaults to `./tests`)

Input paths can be absolute or relative.

## Example

given a directory:

```bash
python-code
└── src
    ├── main.py
    └── module_1
        ├── __init__.py
        └── submodule1.py
```

Running:

```bash
test-builder -s ./src -t ./tests
```

Yields:

```bash
python-code
└── src
└── tests
    ├── test_main.py
    └── test_module_1
        ├── __init__.py
        └── test_submodule1.py
```