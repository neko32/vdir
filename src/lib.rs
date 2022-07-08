

pub mod vdir {

    pub fn walk(root:&str) -> String {
        let buf:String = String::new();
        let mut bufbox:Box<String> = Box::new(buf);
        do_walk(&mut bufbox, root.to_string(), 0);
        *bufbox
    }

    fn do_walk(buf_on_heap:&mut Box<String>, fpath:String, depth:u16) -> () {
        let entries = std::fs::read_dir(fpath).unwrap();
        let mut dirs:Vec<PathAndName> = Vec::new();
        let mut files :Vec<PathAndName> = Vec::new();
        for entry in entries {
            let ent = entry.unwrap();
            let meta = ent.metadata().unwrap();
            let pair = PathAndName { path: ent.path().to_str().unwrap().to_string(), name:ent.file_name().to_str().unwrap().to_string() };
            if meta.is_dir() {
                dirs.push(pair);
            } else {
                files.push(pair);
            }
        }
        dirs.sort_by(|a,b|{a.path.cmp(&b.path)});
        files.sort_by(|a,b|{a.path.cmp(&b.path)});
        for ent in dirs {
            let space:String = std::iter::repeat("-").take(depth as usize).collect();
            buf_on_heap.push_str(format!("{} {} [D]\n", space, ent.path).as_str());
            // buf_on_heap.push_str(format!("{}|\n", space).as_str());
            do_walk(buf_on_heap, ent.path, depth + 2);
        } 
        for ent in files {
            let space:String = std::iter::repeat("-").take(depth as usize).collect();
            buf_on_heap.push_str(format!("{} {} [F]\n", space, ent.name).as_str());
        }
    }

    struct PathAndName {
        path:String,
        name:String,
    }
}

#[cfg(test)]
mod test_suite {

    use super::vdir;

    #[test]
    fn test_basic() {

        let rez = vdir::walk("/home/neko32/appstore");
        println!("rez - \n{}", rez);

    }

}