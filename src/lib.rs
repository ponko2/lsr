use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(
        value_name = "PATH",
        help = "Files and/or directories",
        default_value = "."
    )]
    paths: Vec<String>,

    #[arg(short, long, help = "Long listing")]
    long: bool,

    #[arg(short = 'a', long = "all", help = "Show all files")]
    show_hidden: bool,
}

pub fn get_args() -> Result<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> Result<()> {
    let paths = find_files(&args.paths, args.show_hidden)?;
    for path in paths {
        println!("{}", path.display());
    }
    Ok(())
}

fn find_files(paths: &[String], show_hidden: bool) -> Result<Vec<PathBuf>> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::find_files;

    #[test]
    fn test_find_files() {
        // ディレクトリにある隠しエントリ以外のエントリを検索する
        let res = find_files(&["tests/inputs".to_string()], false);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );

        // 存在するファイルは、隠しファイルであっても検索できるようにする
        let res = find_files(&["tests/inputs/.hidden".to_string()], false);
        assert!(res.is_ok());
        let filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        assert_eq!(filenames, ["tests/inputs/.hidden"]);

        // 複数のパスを与えてテストする
        let res = find_files(
            &[
                "tests/inputs/bustle.txt".to_string(),
                "tests/inputs/dir".to_string(),
            ],
            false,
        );
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            ["tests/inputs/bustle.txt", "tests/inputs/dir/spiders.txt"]
        );
    }

    #[test]
    fn test_find_files_hidden() {
        // ディレクトリにあるすべてのエントリを検索する
        let res = find_files(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/.hidden",
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );
    }
}
