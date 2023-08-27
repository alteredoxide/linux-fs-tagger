
use clap::{Args, Parser, Subcommand};
use regex::Regex;
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

    #[clap(name="list")]
    List(CommandArgs),

    #[clap(name="set")]
    Set(CommandArgs),

    #[clap(name="remove")]
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


fn find_tags(path: &str, tags: Vec<String>) -> anyhow::Result<()> {
    if let Some(fs_tags_string) = get_tags_string(path)? {
        let pattern = Regex::new(&tags.join("|"))?;
        let matches = pattern.find_iter(&fs_tags_string);
        for tag_match in matches {
            println!("{}", tag_match.as_str());
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
