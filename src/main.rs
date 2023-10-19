use std::collections::BTreeSet;
use std::io::{self, BufRead};

use smallvec::SmallVec;

fn main() -> io::Result<()> {
    let input = read_stdin()?;
    let hosts: BTreeSet<_> = hosts_iter(&input)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();

    for host in hosts {
        println!("local-zone: \"{host}\" refuse");
    }

    Ok(())
}

fn read_stdin() -> io::Result<String> {
    let mut stdin = io::stdin().lock();
    let mut buffer = String::with_capacity(128);
    let mut output = String::with_capacity(4096);

    loop {
        buffer.clear();

        let n = stdin.read_line(&mut buffer)?;
        if n == 0 {
            break Ok(output);
        }

        normalize_whitespace(&mut buffer);
        let Some(line) = filter_and_remove_comments(&mut buffer) else {
            continue;
        };
        output.push_str(line);
        output.push('\n');
    }
}

// Removes leading and consecutive internal whitespace from the string.
fn normalize_whitespace(buf: &mut String) {
    let mut prev = true;
    buf.retain(|ch| {
        let yes = ch.is_ascii_whitespace();
        let p = prev;
        prev = yes;
        !(yes && p)
    });
}

// Returns the input if it is not a comment or empty after removing a trailing comment.
fn filter_and_remove_comments(buf: &mut String) -> Option<&str> {
    if buf.starts_with('#') {
        return None;
    }

    let i = buf.find('#').unwrap_or(usize::MAX);
    buf.truncate(i);
    let s = buf.trim_end();

    (!s.is_empty()).then_some(s)
}

fn hosts_iter(s: &str) -> impl Iterator<Item = Result<SmallVec<[&str; 1]>, io::Error>> {
    s.lines().map(|line| {
        let (_addr, hosts) = line
            .split_once(|ch: char| ch.is_ascii_whitespace())
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::Other, "invalid line: no addr/host split")
            })?;
        Ok(hosts.split_ascii_whitespace().collect())
    })
}
