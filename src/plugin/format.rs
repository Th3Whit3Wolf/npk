use std::fs::{write, File};
use std::io::{self, prelude::*, BufReader};

fn vim(file: PathBuf, functions: Vec<String>) -> io::Result<()> {
    let reader = BufReader::new(file);

    let mut packadd: Vec<String> = Vec::with_capacity(5);
    let mut lua: Vec<String> = Vec::with_capacity(5);
    let mut sets: Vec<String> = Vec::with_capacity(5);
    let mut lets: Vec<String> = Vec::with_capacity(10);
    let mut mapping: Vec<String> = Vec::with_capacity(5);
    let mut buffer_vec: Vec<String> = Vec::with_capacity(32);

    for status in reader.lines() {
        match status {
            Ok(line) => {
                if !line.trim().is_empty() {
                    if line.starts_with("packadd") {
                        packadd.push(line)
                    } else if line.starts_with("lua require") {
                        lua.push(line)
                    } else if line.starts_with("set") {
                        sets.push(line)
                    } else if line.starts_with("let") {
                        lets.push(line)
                    } else if line.starts_with("nmap")
                        || line.starts_with("imap")
                        || line.starts_with("vmap")
                        || line.starts_with("smap")
                        || line.starts_with("xmap")
                        || line.starts_with("cmap")
                        || line.starts_with("omap")
                        || line.starts_with("nunmap")
                        || line.starts_with("iunmap")
                        || line.starts_with("vunmap")
                        || line.starts_with("sunmap")
                        || line.starts_with("xunmap")
                        || line.starts_with("cunmap")
                        || line.starts_with("ounmap")
                        || line.starts_with("mapclear")
                        || line.starts_with("mapclear!")
                        || line.starts_with("nmapclear")
                        || line.starts_with("imapclear")
                        || line.starts_with("vmapclear")
                        || line.starts_with("smapclear")
                        || line.starts_with("xmapclear")
                        || line.starts_with("cmapclear")
                        || line.starts_with("omapclear")
                        || line.starts_with("nnoremap")
                        || line.starts_with("inoremap")
                        || line.starts_with("vnoremap")
                        || line.starts_with("snoremap")
                        || line.starts_with("xnoremap")
                        || line.starts_with("cnoremap")
                        || line.starts_with("onoremap")
                    {
                        mapping.push(line)
                    } else {
                        buffer_vec.push(line)
                    }
                }
            }
            Err(e) => eprintln!("Oh no!\n{} occured", e),
        }
    }
    if !packadd.is_empty() {
        packadd.sort();
        packadd.dedup();
        packadd.insert(0, String::from("\" Plugins"))
    }

    if !lua.is_empty() {
        lua.sort();
        lua.dedup();
        packadd.extend(lua);
        packadd.push(String::from("\n"));
    }

    if !sets.is_empty() {
        sets.sort();
        sets.dedup();
        sets.insert(0, String::from("\" Options"));
        sets.push(String::from("\n"));
        packadd.extend(sets);
    }

    if !lets.is_empty() {
        lets.sort();
        lets.dedup();
        lets.insert(0, String::from("\" Set Variables"));
        lets.push(String::from("\n"));
        packadd.extend(lets);
    }

    if !buffer_vec.is_empty() {
        let (function, mut buffer_vec) = get_fn_wrapper(buffer_vec);
        if let Some(mut f) = function {
            f.extend(functions);
            f.sort();
            f.dedup();
            f.insert(0, String::from("\" Declare Functions"));
            packadd.extend(f);
        } else if !buffer_vec.is_empty() {
            packadd.extend(buffer_vec);
        }
        packadd.push(String::from("\n"));
    }

    if !mapping.is_empty() {
        mapping.insert(0, String::from("\" Declare Key Mappings"));
        packadd.extend(mapping);
        packadd.push(String::from("\n"));
    }

    let packadd = packadd.join("\n").replace("\n\n", "\n");

    write(file, packadd.as_bytes())?;

    Ok(())
}

fn get_fn_wrapper(v: Vec<String>) -> (Option<Vec<String>>, Vec<String>) {
    let mut v1 = Vec::new();
    let (f, old) = get_function(v);
    match f {
        Some(x) => {
            v1.push(x.join("\n"));
            let (f2, old2) = get_fn_wrapper(old);
            match f2 {
                Some(x) => {
                    v1.push(x.join("\n"));
                    (Some(v1), old2)
                },
                None => (Some(v1), old2)
            }
        },
        None => (None, old)
    }
}

fn get_function(mut v: Vec<String>) -> (Option<Vec<String>>, Vec<String>) {
    let mut v1 = Vec::new();
    if let (Some(x), Some(y)) = (
        v.iter().position(|val| val.starts_with("fun")),
        v.iter().position(|val| val.starts_with("endfun")),
    ) {
        let mut j = 0 as usize;
        for i in x..=y {
            v1.push(v.remove(i-j));
            j += 1
        }
        (Some(v1), v)
    } else {
        (None, v)
    }
}
