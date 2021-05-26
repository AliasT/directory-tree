#![feature(trait_alias)]

use neon::prelude::*;
use serde::Serialize;
use serde_json;
use std::fs::{canonicalize, metadata, read_dir, Metadata};
use std::io::Result;
use std::path::Path;

#[derive(Serialize, Debug)]
pub struct FileNode {
    /// 文件名称
    name: String,

    /// 绝对路径
    path: String,

    /// 文件大小
    size: u64,

    /// 子目录
    children: Vec<FileNode>,
}

fn walk(dir: &Path) -> Result<FileNode> {
    let mut root: FileNode = path2node(&dir)?;
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                root.children.push(walk(&path)?);
            } else {
                let child = path2node(&entry.path())?;
                root.children.push(child);
            }
        }
    }
    Ok(root)
}

fn path2node(path: &Path) -> Result<FileNode> {
    let path = &canonicalize(path)?;
    let data: Metadata = metadata(&path)?;
    let node = FileNode {
        name: path.file_name().unwrap().to_string_lossy().to_string(),
        path: path.to_string_lossy().to_string(),
        size: data.len(),
        children: Vec::new(),
    };

    Ok(node)
}

pub fn filetree(path: &Path) -> Result<FileNode> {
    let root = walk(path)?;
    Ok(root)
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}

#[test]
fn test_output() {
    let tree = filetree(Path::new("./node_modules")).unwrap();
    println!("{}", serde_json::to_string(&tree).unwrap());
}
