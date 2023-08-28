
use clap::{Args, Parser, Subcommand};
use regex::Regex;
use walkdir::WalkDir;
use xattr;


#[derive(Parser, Debug)]
struct Opts {
    /// The specific command to run (see the Command enum).
    #[command(subcommand)]
    command: Commands
}


#[derive(Args, Debug)]
struct CommandArgs {
    /// Path to file or directory
    #[arg(default_value=".")]
    path: Option<String>,
    /// One or more tags to be associated with the file or directory
    tags: Vec<String>,

}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(name="find")]
    Find(CommandArgs),

    #[clap(name="ls")]
    List(CommandArgs),

    #[clap(name="set")]
    Set(CommandArgs),

    #[clap(name="rm")]
    Remove(CommandArgs),
}


fn get_tags_string(path: &str) -> anyhow::Result<Option<String>> {
    if let Some(tags_bytes) = xattr::get(path, "user.tags")? {
        if tags_bytes.is_empty() {
            return Ok(None)
        }
        let tags_string = String::from_utf8(tags_bytes)?;
        return Ok(Some(tags_string))
    }
    Ok(None)
}


fn get_tags_vec(path: &str) -> anyhow::Result<Vec<String>> {
    let fs_tags_string = get_tags_string(path)?;
    if fs_tags_string.is_none() {
        return Ok(vec![])
    }
    let tags = fs_tags_string
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .collect();
    Ok(tags)
}


fn set_tags(path: &str, tags: Vec<String>) -> anyhow::Result<()> {
    let mut fs_tags = get_tags_vec(path)?;
    for tag in tags {
        let tag = tag.to_lowercase();
        if !fs_tags.contains(&tag) {
            fs_tags.push(tag)
        }
    }
    let joined_tags = fs_tags.join(",");
    xattr::set(path, "user.tags", joined_tags.as_bytes())?;
    Ok(())
}


/// Print all relative sub-paths, within the given path, that contain one or
/// more specified tags. Under each path, the associated tags matching your
/// search will be printed.
fn find_tags(path: &str, tags: Vec<String>) -> anyhow::Result<()> {
    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        let subpath = entry.path();
        let subpath_opt = subpath.to_str();
        if subpath_opt.is_none() {
            continue
        }
        let subpath_str = subpath_opt.unwrap();
        if let Some(fs_tags_string) = get_tags_string(subpath_str)? {
            let pattern = Regex::new(&tags.join("|"))?;
            let matching_tags: Vec<&str> = pattern
                .find_iter(&fs_tags_string)
                .into_iter()
                .map(|m| m.as_str())
                .collect();
            if matching_tags.is_empty() {
                continue
            }
            println!("{}", subpath_str);
            for tag_match in matching_tags {
                println!("  {}", tag_match);
            }
        }
    }
    Ok(())
}


fn list_tags(path: &str) -> anyhow::Result<()> {
    for tag in get_tags_vec(path)? {
        println!("{}", tag);
    }
    Ok(())
}


fn remove_tags(path: &str, tags: Vec<String>) -> anyhow::Result<()> {
    let fs_tags: Vec<String> = get_tags_vec(path)?
        .into_iter()
        .filter(|s| !tags.contains(s))
        .collect();
    let joined_tags = fs_tags.join(",");
    xattr::remove(path, "user.tags")?;
    xattr::set(path, "user.tags", joined_tags.as_bytes())?;
    Ok(())
}


fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let _ = match opts.command {
        Commands::Find(args) => find_tags(&args.path.unwrap(), args.tags),
        Commands::List(args) => list_tags(&args.path.unwrap()),
        Commands::Set(args) => set_tags(&args.path.unwrap(), args.tags),
        Commands::Remove(args) => remove_tags(&args.path.unwrap(), args.tags),
    };
    Ok(())
}
