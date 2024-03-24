use std::path::Path;

use hyperpolyglot::Detection;

pub struct GuessLangManager {}

impl GuessLangManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn guess(&self, content: &str, path: Option<&Path>) -> String {
        if let Some(path) = path {
            return self.by_path(path).unwrap_or_else(|| {
                self.by_content(content)
                    .unwrap_or_else(|| "PlainText".to_string())
            });
        }

        self.by_content(content)
            .unwrap_or_else(|| "PlainText".to_string())
    }

    pub fn by_content(&self, content: &str) -> Option<String> {
        // 进行纯文本检测
        if content.chars().all(|c| c.is_whitespace()) {
            return Some("PlainText".to_string());
        }

        let result = hyperpolyglot::detectors::classify(content, &*vec![]);
        Some(result.to_string())
    }

    pub fn by_path(&self, path: &Path) -> Option<String> {
        let result = hyperpolyglot::detect(path);

        match result {
            Ok(detection) => {
                let result = detection.ok_or(Detection::Classifier("PlainText")).unwrap();
                Some(result.language().to_string())
            }
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_lang_plan_text() {
        let guess_lang_manager = GuessLangManager::new();
        let code = r#"Hello, World!"#;
        let lang = guess_lang_manager.guess(code, None);
        assert_eq!(lang, "PlainText");
    }

    #[test]
    fn test_guess_lang_rust() {
        let guess_lang_manager = GuessLangManager::new();
        let code = r#"
        use std::str::FromStr;

mod hyperpolyglot;

pub struct GuessLangManager {}

impl GuessLangManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn truncate_to_char_boundary(s: &str, mut max: usize) -> &str {
        if max >= s.len() {
            s
        } else {
            while !s.is_char_boundary(max) {
                max -= 1;
            }
            &s[..max]
        }
    }

    pub fn guess_lang(&self, code: &str) -> String {
        let content = Self::truncate_to_char_boundary(code, 51200);

        let result = hyperpolyglot::detectors::classify(content, &*vec![]);
        result.to_string()
    }
}
        "#;
        let lang = guess_lang_manager.guess(code, None);
        assert_eq!(lang, "Rust");
    }
}
