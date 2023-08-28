# linux-fs-tagger (tag)

linux-fs-tagger is currently a very minimal (alpha) command-line tool for
creating and managing tags for directories and files within any file system that
supports extended attriubtes (xattrs).
All major linux file systems offer support:
- Ext4
- Btrfs
- ZFS
- XFS
This alpha offers very basic support for set, list, find, and remove.

I made this just to toy with having a simple tool for tagging files and
directories on my own systems, so there is a good chance this will not receive
a lot of support.

## Installation

### Install from github
**Clone and install**
```bash
git clone https://github.com/alteredoxide/linux-fs-tagger.git
cargo install --path ./linux-fs-tagger
```
**Add to path**: add the following line to your `.bashrc` or `.zshrc`
```bash
export PATH=$PATH:$HOME/.cargo/bin
```


## Usage

### Add tags
Add one or more space separated tags to the current directory
```bash
tag set . my_tag1 my_tag2
```

Add one or more space separated tags to a specific file or directory
```bash
tag set ~/path/to/file.ext my_tag1 my_tag2
tag set ~/path/to/dir my_tag1 my_tag2
```

### Remove tags
Remove one or more tags from a specific file or directory
```bash
tag rm ~/path/to/dir my_tag2
```

### List tags
List all tags associated with a specific file or directory
```bash
tag ls ~/path/to/file.ext
// my_tag1
// my_tag2
```

### Find paths with tags
Recursively find paths that match specified tags.
**NOTE:** this project is not at all optimized, and there is currently no way
to limit depth. That means this command could run for a long time if the file
tree for your path is large.
```bash
tag find ~/some/path foo_tag bar_tag
``

