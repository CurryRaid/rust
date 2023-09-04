use regex::Regex;
use std::fs;
use std::path::Path;

// <P>泛型，且要求实现了AsRef<Path> trait
pub fn find<P: AsRef<Path>>(
    root: P,
    regex: &Regex,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regex, &mut matches)?; //?表示如果有错误，就返回错误
    Ok(matches)
}

fn walk_tree(
    dir: &Path,
    regex: &Regex,
    matches: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        //第一个问号表示是否成功返回一个目录迭代器，第二个问号表示是否成功遍历目录迭代器
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            // println!("{:?}", entry);
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, matches)?;
            } else {
                // to_string_lossy() 方法将 PathBuf 对象转换为 Cow<str> 类型的对象，
                // 调用 .to_string() 方法将其转换为 String 类型。
                // 注意，to_string_lossy() 方法可以处理包含非 UTF-8 字符的路径，并将其转换为合法的 UTF-8 字符串
                let path_str = path.to_string_lossy().to_string();
                if regex.is_match(&path_str) {
                    matches.push(path_str);
                }
            }
        }
    }
    Ok(())
}
