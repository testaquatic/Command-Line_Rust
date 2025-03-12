use std::{
    fs, io,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::PathBuf,
};

use args::Args;
use chrono::{DateTime, Local};
use tabular::{Row, Table};
use users::get_user_by_uid;

mod args;

pub fn run() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let paths = find_files(&args.paths, args.show_hiden)?;
    if args.long {
        println!("{}", format_output(&paths)?);
    } else {
        paths.iter().for_each(|path| println!("{}", path.display()));
    }

    Ok(())
}

fn find_files(paths: &[String], show_hidden: bool) -> Result<Vec<PathBuf>, io::Error> {
    let mut result = Vec::new();
    for path in paths {
        let meta = match fs::metadata(path) {
            Ok(meta) => meta,
            Err(e) => {
                eprintln!("{}: {}", path, e);
                continue;
            }
        };

        if meta.is_file() {
            result.push(PathBuf::from(path));
        } else if meta.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                if !show_hidden && entry.file_name().to_string_lossy().starts_with('.') {
                    continue;
                }
                result.push(entry.path());
            }
        }
    }

    Ok(result)
}

fn format_output(paths: &[PathBuf]) -> Result<String, anyhow::Error> {
    let fmt = "{:<}{:<} {:>} {:<} {:<} {:>} {:<} {:<}";
    let mut table = Table::new(fmt);

    for path in paths {
        let meta = fs::metadata(path)?;
        let mode = meta.permissions().mode();
        let username = get_user_by_uid(meta.uid())
            .map(|u| u.name().to_string_lossy().to_string())
            .unwrap_or_default();
        let groupname = get_user_by_uid(meta.gid())
            .map(|u| u.name().to_string_lossy().to_string())
            .unwrap_or_default();
        let modified = DateTime::<Local>::from(meta.modified()?).format("%b %d %R");

        table.add_row(
            Row::new()
                .with_cell(if meta.is_dir() { "d" } else { "-" }) // "d" or "-"
                .with_cell(format_mod(mode)) // 권한
                .with_cell(meta.nlink()) // 링크 수
                .with_cell(username) // 사용자 이름
                .with_cell(groupname) // 그룹 이름
                .with_cell(meta.len()) // 크기
                .with_cell(modified) // 수정
                .with_cell(path.display()), // 경로
        );
    }

    Ok(format!("{}", table))
}

/// "rwxr-x--x"와 같은 문자열을 반환한다.
fn format_mod(mode: u32) -> String {
    (0..3)
        .map(|shift| {
            let bits = mode >> (shift * 3);
            let r = if bits & 0o4 != 0 { "r" } else { "-" };
            let w = if bits & 0o2 != 0 { "w" } else { "-" };
            let x = if bits & 0o1 != 0 { "x" } else { "-" };
            format!("{}{}{}", r, w, x)
        })
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use pretty_assertions::assert_eq;

    use crate::{find_files, format_mod, format_output};

    #[test]
    fn test_find_files() {
        // 숨겨지지 않은 모든 항목
        let res = find_files(&["tests/inputs".to_string()], false);
        assert!(res.is_ok());
        let mut filenames = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect::<Vec<_>>();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt"
            ]
        );

        // 모든 항목
        let res = find_files(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());
        let mut filenames = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect::<Vec<_>>();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/.hidden",
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt"
            ]
        );

        // 숨겨진 파일
        let res = find_files(&["tests/inputs/.hidden".to_string()], false);
        assert!(res.is_ok());
        let filenames = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect::<Vec<_>>();
        assert_eq!(filenames, ["tests/inputs/.hidden"]);

        // 여러개의 경로
        let res = find_files(
            &[
                "tests/inputs/bustle.txt".to_string(),
                "tests/inputs/dir".to_string(),
            ],
            false,
        );
        assert!(res.is_ok());
        let mut filenames = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect::<Vec<_>>();
        filenames.sort();
        assert_eq!(
            filenames,
            ["tests/inputs/bustle.txt", "tests/inputs/dir/spiders.txt"]
        );
    }

    #[test]
    fn test_find_files_hidden() {
        let res = find_files(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());
        let mut filenames = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect::<Vec<_>>();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/.hidden",
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt"
            ]
        )
    }

    #[test]
    fn test_format_mode() {
        assert_eq!(format_mod(0o777), "rwxrwxrwx");
        assert_eq!(format_mod(0o775), "rwxrwxr-x");
        assert_eq!(format_mod(0o755), "rwxr-xr-x");
        assert_eq!(format_mod(0o664), "rw-rw-r--");
        assert_eq!(format_mod(0o421), "r---w---x");
    }

    fn long_match(
        line: &str,
        expected_name: &str,
        expected_perms: &str,
        expected_size: Option<&str>,
    ) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() > 0 && parts.len() <= 10);
        assert_eq!(&parts[0], &expected_perms);

        if let Some(size) = expected_size {
            assert_eq!(&parts[4], &size);
        }

        assert_eq!(parts.last().unwrap(), &expected_name);
    }

    #[test]
    fn test_format_output_one() {
        let bustle_path = "tests/inputs/bustle.txt";
        let bustle = PathBuf::from(bustle_path);

        let res = crate::format_output(&[bustle]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let lines = out
            .split('\n')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        assert_eq!(lines.len(), 1);

        let line1 = lines[0];
        long_match(&line1, bustle_path, "-rw-r--r--", Some("193"));
    }

    #[test]
    fn test_format_output_two() {
        let res = format_output(&[
            PathBuf::from("tests/inputs/dir"),
            PathBuf::from("tests/inputs/empty.txt"),
        ]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let mut lines = out
            .split('\n')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        lines.sort();
        assert_eq!(lines.len(), 2);

        let empty_line = lines.remove(0);
        long_match(
            &empty_line,
            "tests/inputs/empty.txt",
            "-rw-r--r--",
            Some("0"),
        );

        let dir_line = lines.remove(0);
        long_match(&dir_line, "tests/inputs/dir", "drwxr-xr-x", None);
    }
}
