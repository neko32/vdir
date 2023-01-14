#![warn(clippy::all, clippy::pedantic)]

pub mod vdir {

    use once_cell::sync::Lazy;

    static VERTICAL:Lazy<String> = Lazy::new(|| {
        String::from_utf16(&[0x2514]).unwrap()
    });

    static HORIZONTAL:Lazy<String> = Lazy::new(|| {
        String::from_utf16(&[0x2500]).unwrap()
    });

    #[must_use]
    pub fn walk(root:&str) -> String {
        println!("horiz - {}, vert - {}", *HORIZONTAL, *VERTICAL);
        let mut buf:String = String::new();
        do_walk(&mut buf, root.to_string(), 0);
        buf
    }

    fn do_walk(buf_on_heap:&mut String, fpath:String, depth:u16) {
        let entries = std::fs::read_dir(fpath).unwrap();
        let mut dirs:Vec<PathAndName> = Vec::new();
        let mut files :Vec<PathAndName> = Vec::new();
        buf_on_heap.push_str((*VERTICAL).as_str());
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
            let space:String = (*HORIZONTAL).repeat(depth as usize);
            buf_on_heap.push_str(format!("{} {} [D]\n", space, ent.path).as_str());
            // buf_on_heap.push_str(format!("{}|\n", space).as_str());
            // buf_on_heap.push_str((*vertical).as_str());
            do_walk(buf_on_heap, ent.path, depth + 2);
        }
        for (idx, ent) in files.iter().enumerate() {
            let revised_depth = if idx == 0  && depth > 0 {
                depth - 1
            } else {
                depth
            };
            let space:String = (*HORIZONTAL).repeat(revised_depth as usize);
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

        let rez = vdir::walk("/home/neko32/dev/docker");
        println!("rez - \n{}", rez);

    }

}