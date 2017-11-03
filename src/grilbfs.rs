
use std::collections::HashMap;

use std::ffi::OsStr;
use libc::ENOENT;
use time::Timespec;
use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };                 // 1 second

const CREATE_TIME: Timespec = Timespec { sec: 1381237736, nsec: 0 };    // 2013-10-08 08:56

const HELLO_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: CREATE_TIME,
    mtime: CREATE_TIME,
    ctime: CREATE_TIME,
    crtime: CREATE_TIME,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

const HELLO_TXT_CONTENT: &'static str = "Hello World!\n";

const HELLO_TXT_ATTR: FileAttr = FileAttr {
    ino: 2,
    size: 13,
    blocks: 1,
    atime: CREATE_TIME,
    mtime: CREATE_TIME,
    ctime: CREATE_TIME,
    crtime: CREATE_TIME,
    kind: FileType::RegularFile,
    perm: 0o644,
    nlink: 1,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

enum Node {
    File: FileNode,
    Directory: DirNode,
};

struct FileNode {
    contents: Vec<u8>,
};

struct DirNode {
    children: HashMap<Vec<u8>, Node>,
};

pub struct GrilbFS {
    inodes: Vec<Node>,
    root: DirNode,
};

// code to build the Node graph from the plaintext backing store

fn parse_archive(archive: &[u8]) -> DirNode {
    let mut root = DirNode(children: HashMap::new());
    for (path, contents) in archive.SPLIT() {
        add_file(path, FileNode(contents: contents));
    }
    root
}

fn add_file(root: &mut DirNode, path: &[u8], filenode: FileNode) {
    let &mut here = root;
    let pieces = path.SPLIT("/");
    let leafname = pieces[-1];
    for name in pieces[:-1] {
        match here {
            File(_) => { return Error("cannot dereference files"); },
            Directory(parent) => { 
                match parent.get(&name) {
                    Some(child) => { here = child; },
                    None => { here = DirNode(children: HashMap::new());
                              parent.children.insert(name, here);
                    },
                };
            },
        }
    }
    match here.get(&leafname) {
        Some(child) => { return Error("leaf already exists"); },
        None => { parent.children.insert(leafname, filenode); },
    };
}

// code to serialize the Node graph to a plaintext backing store

fn dump_archive(&root: dirnode) -> Vec<u8> {
}

impl Filesystem for GrilbFS {
    fn lookup (&mut self, _req: &Request, 
               parent: u64, name: &OsStr, 
               reply: ReplyEntry) {
        if parent == 1 && name.to_str() == Some("hello.txt") {
            reply.entry(&TTL, &HELLO_TXT_ATTR, 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr (&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        match ino {
            1 => reply.attr(&TTL, &HELLO_DIR_ATTR),
            2 => reply.attr(&TTL, &HELLO_TXT_ATTR),
            _ => reply.error(ENOENT),
        }
    }

    fn read (&mut self, _req: &Request, ino: u64, _fh: u64, offset: u64, _size: u32, reply: ReplyData) {
        if ino == 2 {
            reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir (&mut self, _req: &Request, ino: u64, _fh: u64, offset: u64, mut reply: ReplyDirectory) {
        if ino == 1 {
            if offset == 0 {
                reply.add(1, 0, FileType::Directory, ".");
                reply.add(1, 1, FileType::Directory, "..");
                reply.add(2, 2, FileType::RegularFile, "hello.txt");
            }
            reply.ok();
        } else {
            reply.error(ENOENT);
        }
    }
}
