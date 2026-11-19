//! Translations. The program's own texts are written in English; `tr("English text")` returns the
//! text in the language the person picked. Every other language is a table that maps the English
//! text to its translation (see `i18n/de/`). A text missing from a table shows in English.
//!
//! Adding a language: add a variant to `Lang`, a folder like `i18n/de/` with its tables, and a
//! line in `table()`. The test at the bottom checks that every `tr`/`trf` text has a German entry.

use std::collections::HashMap;
use std::fmt::Display;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU8, Ordering};

use serde::{Deserialize, Serialize};

mod de;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lang {
    En,
    De,
}

impl Lang {
    pub const ALL: [Lang; 2] = [Lang::En, Lang::De];

    /// The language's own name, as shown in the language menu.
    pub fn native_name(self) -> &'static str {
        match self {
            Lang::En => "English",
            Lang::De => "Deutsch",
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::De => "de",
        }
    }

    pub fn from_code(code: &str) -> Option<Lang> {
        let c = code.trim().to_ascii_lowercase();
        if c.starts_with("de") {
            Some(Lang::De)
        } else if c.starts_with("en") {
            Some(Lang::En)
        } else {
            None
        }
    }

    /// The computer's language if we have it, English otherwise.
    pub fn system() -> Lang {
        sys_locale::get_locale().and_then(|l| Lang::from_code(&l)).unwrap_or(Lang::En)
    }
}

static CURRENT: AtomicU8 = AtomicU8::new(0);

pub fn set(lang: Lang) {
    CURRENT.store(lang as u8, Ordering::Relaxed);
}

pub fn current() -> Lang {
    match CURRENT.load(Ordering::Relaxed) {
        1 => Lang::De,
        _ => Lang::En,
    }
}

fn table(lang: Lang) -> Option<&'static HashMap<&'static str, &'static str>> {
    static DE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    match lang {
        Lang::En => None,
        Lang::De => Some(DE.get_or_init(|| de::TABLES.iter().flat_map(|t| t.iter().copied()).collect())),
    }
}

/// The text in the current language.
pub fn tr(en: &'static str) -> &'static str {
    table(current()).and_then(|t| t.get(en).copied()).unwrap_or(en)
}

/// The German text for `en` whatever the current language is (for tests).
#[cfg(test)]
pub fn german(en: &'static str) -> &'static str {
    table(Lang::De).and_then(|t| t.get(en).copied()).unwrap_or(en)
}

/// Like `tr`, then each `{}` is replaced by the next argument.
pub fn trf(en: &'static str, args: &[&dyn Display]) -> String {
    fill(tr(en), args)
}

fn fill(template: &str, args: &[&dyn Display]) -> String {
    let mut out = String::with_capacity(template.len() + 16);
    let mut rest = template;
    let mut i = 0;
    while let Some(p) = rest.find("{}") {
        out.push_str(&rest[..p]);
        if let Some(a) = args.get(i) {
            out.push_str(&a.to_string());
        }
        i += 1;
        rest = &rest[p + 2..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn fill_replaces_in_order() {
        assert_eq!(fill("{} of {}", &[&1, &"two"]), "1 of two");
        assert_eq!(fill("none", &[&1]), "none");
    }

    #[test]
    fn german_lookup_and_fallback() {
        // (doesn't switch the language: other tests run at the same time and read it)
        let de = table(Lang::De).unwrap();
        assert_eq!(de.get("🔧 Tools"), Some(&"🔧 Werkzeuge"));
        assert!(!de.contains_key("this text is not in any table"));
        assert_eq!(Lang::from_code("de-DE"), Some(Lang::De));
        assert_eq!(Lang::from_code("en_US"), Some(Lang::En));
        assert_eq!(Lang::from_code("fr-FR"), None);
    }

    /// Reads one Rust string literal (normal or raw) at the start of `s`.
    fn literal(s: &str) -> Option<String> {
        let s = s.trim_start();
        if let Some(r) = s.strip_prefix('r') {
            let hashes = r.chars().take_while(|c| *c == '#').count();
            let body = r[hashes..].strip_prefix('"')?;
            let end = format!("\"{}", "#".repeat(hashes));
            return body.find(&end).map(|e| body[..e].to_string());
        }
        let body = s.strip_prefix('"')?;
        let mut out = String::new();
        let mut it = body.chars().peekable();
        while let Some(c) = it.next() {
            match c {
                '"' => return Some(out),
                '\\' => match it.next()? {
                    'n' => out.push('\n'),
                    't' => out.push('\t'),
                    'r' => out.push('\r'),
                    '0' => out.push('\0'),
                    '\\' => out.push('\\'),
                    '"' => out.push('"'),
                    '\'' => out.push('\''),
                    'u' => {
                        let mut hex = String::new();
                        it.next();
                        for h in it.by_ref() {
                            if h == '}' {
                                break;
                            }
                            hex.push(h);
                        }
                        out.push(char::from_u32(u32::from_str_radix(&hex, 16).ok()?)?);
                    }
                    '\n' | '\r' => {
                        while it.peek().is_some_and(|c| c.is_whitespace()) {
                            it.next();
                        }
                    }
                    other => {
                        out.push('\\');
                        out.push(other);
                    }
                },
                c => out.push(c),
            }
        }
        None
    }

    fn collect(dir: &Path, found: &mut Vec<(String, String)>) {
        for e in std::fs::read_dir(dir).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                if p.file_name().is_some_and(|n| n != "i18n") {
                    collect(&p, found);
                }
                continue;
            }
            if p.extension().is_none_or(|x| x != "rs") || p.file_name().is_some_and(|n| n == "i18n.rs") {
                continue;
            }
            // (the compiler reads CRLF line ends as LF, so the texts it sees have LF)
            let src = std::fs::read_to_string(&p).unwrap().replace("\r\n", "\n");
            for pat in ["tr(", "trf("] {
                let mut from = 0;
                while let Some(i) = src[from..].find(pat) {
                    let at = from + i;
                    from = at + pat.len();
                    let before = src[..at].chars().next_back();
                    if before.is_some_and(|c| c.is_alphanumeric() || c == '_') {
                        continue;
                    }
                    if let Some(text) = literal(&src[from..]) {
                        found.push((p.file_name().unwrap().to_string_lossy().into_owned(), text));
                    }
                }
            }
        }
    }

    #[test]
    fn every_text_has_a_german_translation() {
        let mut found = Vec::new();
        collect(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut found);
        assert!(found.len() > 50, "too few texts found: {}", found.len());
        let de = table(Lang::De).unwrap();
        let mut missing: Vec<String> = found
            .iter()
            .filter(|(_, t)| t.chars().any(|c| c.is_alphabetic()) && !de.contains_key(t.as_str()))
            .map(|(f, t)| format!("{f}: {t:?}"))
            .collect();
        missing.sort();
        missing.dedup();
        assert!(missing.is_empty(), "{} texts without German translation:\n{}", missing.len(), missing.join("\n"));
    }

    #[test]
    fn german_tables_are_consistent() {
        let mut seen: HashMap<&str, &str> = HashMap::new();
        let mut problems = Vec::new();
        for t in de::TABLES {
            for (en, de) in t.iter() {
                if en.matches("{}").count() != de.matches("{}").count() {
                    problems.push(format!("placeholders differ: {en:?} -> {de:?}"));
                }
                if let Some(old) = seen.insert(en, de) {
                    if old != *de {
                        problems.push(format!("two translations for {en:?}: {old:?} / {de:?}"));
                    }
                }
            }
        }
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }
}
