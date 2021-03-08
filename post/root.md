# Build your own: linker

By: [Andrew Halle](https://github.com/andrewhalle)
Repo: [byo-linker](https://github.com/andrewhalle/byo-linker)

Part of [build-your-own](https://github.com/andrewhalle/build-your-own)

## Background

In this series of posts, I'd like to build up a simple but functional linker in
Rust, mostly just as an exercise to see what goes into a real linker. Some
high-level goals/requirements for this linker, it needs to:

  * produce an executable from object files produced from programs written in C
  * work on Linux only
  * support static and shared libraries

In particular this means that I do not concern myself with name-mangling or
with object file formats other than the ELF object format.

But what is a linker? A linker is a link in the compiler toolchain that
combines separately compiled object files and resolves undefined references
between them. As a motivating example, consider a simple C program with two files.

```main.c
void greet(void);

int main() {
  greet();
}
```
```greet.c
#include <stdio.h>

void greet(void) {
  printf("hello world!\n");
}
```

These files can be compiled separately, but before the program can be run, the
_declaration_ of `greet` contained in `main.c` must be connected with the
_definition_ of `greet` contained in `greet.c`. Additionally, the C standard
library, which contains a definition for `printf` must be included as well.
This is the job of the linker. Graphically:

![linkers graphically](http://placekitten.com/200/300 "the linking process")

## Better resources

I'm only a tinkerer, not an authority. Here are some resources I looked at (or
made a note to look at after I finished, so as not to make it too easy on
myself) while writing this series of posts. Any errors or misunderstandings are
my own.

1. System V ABI
  * [OSDev Wiki](https://wiki.osdev.org/System_V_ABI)
  * [System V ABI - Older Base Document](http://www.sco.com/developers/devspecs/gabi41.pdf)
  * [System V ABI - Latest Base Document](http://www.sco.com/developers/gabi/latest/contents.html)
2. [Linkers - Ian Lance Taylor](https://www.airs.com/blog/archives/38)
3. [Linkers and Loaders - John R. Levine](https://www.goodreads.com/book/show/1103509.Linkers_and_Loaders)

## Posts

Without further ado, here are the posts.

1. [Parsing ELF files](https://andrewhalle.github.io/build-your-own/linker/1)
2. [Serializing ELF files](https://andrewhalle.github.io/build-your-own/linker/2)
3. [Simple linking - creating a relocatable file](https://andrewhalle.github.io/build-your-own/linker/3)
4. [Intermediate linking - including static libraries](https://andrewhalle.github.io/build-your-own/linker/4)
5. [Advanced linking - including shared libraries](https://andrewhalle.github.io/build-your-own/linker/5)
6. [Putting it all together - creating an executable](https://andrewhalle.github.io/build-your-own/linker/6)
