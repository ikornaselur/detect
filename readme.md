# Detect

Traverse up diretories to look for files/folders by name.

An example, with the folder structure

```
./
├── bar/
├── baz/
├── file
└── foo/
   ├── norf/
   └── qux/
      └── file
```

running `detect file` from `./foo/qux` would return `./foo/qux/file` while running it in `./foo/norf` would return `./file`.

`detect` will try to traverse all the way up to `/`
