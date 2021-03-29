# Build your own: linker

A very simple and hacky linker built to better understand linker internals.
Does only one linking task: combining relocatable object files into a single
relocatable object file. All of the following are true:

* only works on 64-bit Linux
* only tested with extremely simple cases of linking two object files, more
  simple files ought to work, but any non-trival code would likely run into
  unimplemented parts of the ELF file spec
* always writes out the result to `output.o`
* errors not handled gracefully

Here are some resources I looked at when implementing my linker, by people who
actually know what they're talking about:

1. System V ABI
  * [OSDev Wiki](https://wiki.osdev.org/System_V_ABI)
  * [System V ABI - Older Base Document](http://www.sco.com/developers/devspecs/gabi41.pdf)
  * [System V ABI - Latest Base Document](http://www.sco.com/developers/gabi/latest/contents.html)
2. [Linkers - Ian Lance Taylor](https://www.airs.com/blog/archives/38)
3. [Linkers and Loaders - John R. Levine](https://www.goodreads.com/book/show/1103509.Linkers_and_Loaders)
4. [The missing link](https://www.cl.cam.ac.uk/~pes20/rems/papers/oopsla-elf-linking-2016.pdf)
5. [Toolchains.net](https://www.toolchains.net/)

With that out of the way, here's an overview of interesting bits of the project.

## Overview

This linker implements the following workflow (see test files in
`test-files/01-combine-objects` and `test-files/02-combine-objects`)

```bash
$ clang -c -o main.o main.c
$ clang -c -o greet.o greet.c
$ ld -r -o combined.o greet.o main.o # <-- linker invocation
$ clang -o main combined.o
```

It combines input object files into a new relocatable object file. The result
is then linked into an executable (with `ld` again but used through the `clang`
compiler driver so I don't have to type out all the required options)

With my linker

```bash
$ clang -c -o main.o main.c
$ clang -c -o greet.o greet.c
$ cargo run -- *.o
$ clang -o main output.o
```

This linker takes approximately the following actions:

```
1. parse ELF files into an in-memory representation
2. for each ELF file being linked
  a. merge like sections (e.g. merge `.text` to `.text`)
  b. resolve undefined symbols
3. write out the result
```

The way this linker accomplishes these tasks is:

```
1. use `nom` to parse ELF files
2.
  a. match sections by name, if a matching name is found, concatentate them (respecting alignment)
  b. put all symbols in a hashmap, if a symbol with the same name is found, and
     one is undefined, merge them (if two defined symbols have the same name,
     panic)
3. write out the result with `byteorder`
```

Some interesting takeaways I had from each section are below

## Parsing ELF files

I stumbled on a pattern that I quite like while parsing ELF files. I was faced
with the following problem: ELF files have a particular representation on disk,
but that representation is not easy to link. In particular, I ran into a
problem with handling symbol/section names.

ELF files store all strings in special string table sections, and metadata
tracks which string table holds symbol names and which string table stores
section names. When trying to link ELF files in this format, it became onerous
to try to track which string table an index pointed into, and re-write these
indices while merging sections. I decided that it would be easier to copy the
string name into my in-memory representation of the symbol/section.

To accomplish this, I kept two versions of each of the data structures defined
for ELF files, a _raw_ version and a regular version. My `nom` parser would
parse a `*Raw` struct, and then I implemented `From<ElfFile64Raw>` for
`ElfFile64` to build the in-memory version and do the tasks like building the
symbols and sections with string names, and putting relocations in a vector
owned by the section it relocates.

## Merging sections

Linkers don't have a complex job, they don't need to understand the bytes
they're linking together (unless they're doing link-time optimization!) so
merging sections is little more than concatenating slices of bytes together.
The one interesting part of this is that some sections have an alignment
requirement (`.text` requires an 8-byte alignment for example) so when
concatenating these sections, occasionally some padding is required.

Where this really starts to add complexity is when re-writing relocation
references.  Relocations (calculations to fill in missing addresses that the
compiler can't possibly know, for example, a jump address) reference a symbol
and an offset.  When merging sections, careful attention has to be paid to make
sure that symbols and relocations continue to refer to the correct thing. I
didn't find a particularly elegant way to do this, I mostly used a lot of
`HashMap`s to provide a mapping between old and new indices (I'm certain better
ways to do this exist)

## Writing out the result

There's at least at least one interesting thing to say about this. ELF files
store string data in string table sections (which are C strings concatenated
together, with a `\0` byte at the beginning and end, so that every index into
this array of bytes is a valid C string) and use metadata to describe where the
section headers and symbol names are located.

One interesting thing that I found is that `clang` will emit all names (both
section and symbol) in one string table `.strtab`, and `ld` will emit two
string tables, `.strtab` for the symbol names, and `.shstrtab` for the section
names. So, for example, if you put a single object file through `ld` to make
another relocatable file, you will end up with a different result (a new
`.shstrtab` will be created). This validates for me my choice to copy string
data into my in-memory representation of symbols and sections, because I
suspect `ld` is doing the same, else this splitting would be very difficult. My
linker mirrors `ld`s behavior in this regard.
