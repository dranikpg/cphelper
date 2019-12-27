use std::path::PathBuf;

/**
 * If path is dir, add file "a.out"
 */
pub fn fix_ex_path(opt: &str) -> PathBuf {
    let mut p = PathBuf::from(opt);
    if p.is_dir(){
        p.push("a.out");
    }
    p
}


/**
 * Get child of pathbuf
 */
pub fn path_child(buf: &PathBuf, sub: &str) -> PathBuf{
    let mut p = buf.clone();
    p.push(sub);
    p
}
