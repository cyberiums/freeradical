use log::{info, warn};
use std::collections::HashMap;

/// Text Command Parser
/// Parses natural language commands for AI automation
pub struct CommandParser;

/// Parsed command with intent and parameters
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedCommand {
    pub intent: CommandIntent,
    pub parameters: HashMap<String, String>,
    pub raw_text: String,
}

/// Command intent types
#[derive(Debug, Clone, PartialEq)]
pub enum CommandIntent {
    GeneratePage { topic: String },
    OptimizeSEO { page_id: i32 },
    Summarize { url: String },
    Translate { page_id: i32, language: String },
    AnalyzeContent { content: String },
    Unknown,
}

impl CommandParser {
    /// Parse a text command
    pub fn parse(text: &str) -> ParsedCommand {
        let trimmed = text.trim();
        
        info!("Parsing command: {}", trimmed);

        // Detect slash commands first
        if let Some(intent) = Self::parse_slash_command(trimmed) {
            return ParsedCommand {
                intent,
                parameters: HashMap::new(),
                raw_text: trimmed.to_string(),
            };
        }

        // Detect natural language commands
        if let Some(intent) = Self::parse_natural_language(trimmed) {
            return ParsedCommand {
                intent,
                parameters: HashMap::new(),
                raw_text: trimmed.to_string(),
            };
        }

        warn!("Could not parse command intent");
        ParsedCommand {
            intent: CommandIntent::Unknown,
            parameters: HashMap::new(),
            raw_text: trimmed.to_string(),
        }
    }

    /// Parse slash-style commands (/command args)
    fn parse_slash_command(text: &str) -> Option<CommandIntent> {
        if !text.starts_with('/') {
            return None;
        }

        let parts: Vec<&str> = text.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        let command = parts[0].to_lowercase();

        match command.as_str() {
            "/generate-page" | "/generate" => {
                if parts.len() > 1 {
                    let topic = parts[1..].join(" ");
                    Some(CommandIntent::GeneratePage { topic })
                } else {
                    None
                }
            }
            "/optimize-seo" | "/seo" => {
                if parts.len() > 1 {
                    if let Ok(page_id) = parts[1].parse::<i32>() {
                        Some(CommandIntent::OptimizeSEO { page_id })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "/summarize" | "/summary" => {
                if parts.len() > 1 {
                    let url = parts[1].to_string();
                    Some(CommandIntent::Summarize { url })
                } else {
                    None
                }
            }
            "/translate" => {
                if parts.len() > 2 {
                    if let Ok(page_id) = parts[1].parse::<i32>() {
                        let language = parts[2].to_string();
                        Some(CommandIntent::Translate { page_id, language })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Parse natural language commands
    fn parse_natural_language(text: &str) -> Option<CommandIntent> {
        let lower = text.to_lowercase();

        // Generate page intent
        if (lower.contains("create") || lower.contains("generate") || lower.contains("write"))
            && (lower.contains("page") || lower.contains("article") || lower.contains("post"))
        {
            // Extract topic after "about" or "on"
            let topic = if let Some(idx) = lower.find("about") {
                text[idx + 5..].trim().to_string()
            } else if let Some(idx) = lower.find("on ") {
                text[idx + 3..].trim().to_string()
            } else {
                // Use everything after the verb
                let words: Vec<&str> = text.split_whitespace().collect();
                if words.len() > 3 {
                    words[3..].join(" ")
                } else {
                    "untitled".to_string()
                }
            };

            return Some(CommandIntent::GeneratePage { topic });
        }

        // SEO optimization intent
        if (lower.contains("optimize") || lower.contains("improve"))
            && (lower.contains("seo") || lower.contains("search"))
        {
            // Try to extract page ID
            if let Some(page_id) = Self::extract_number(&lower) {
                return Some(CommandIntent::OptimizeSEO { page_id });
            }
        }

        // Summarize intent
        if lower.contains("summarize") || lower.contains("summary") {
            // Extract URL if present
            if let Some(url) = Self::extract_url(text) {
                return Some(CommandIntent::Summarize { url });
            }
        }

        // Translate intent  
        if lower.contains("translate") {
            if let Some(page_id) = Self::extract_number(&lower) {
                // Extract language
                let language = Self::extract_language(&lower).unwrap_or_else(|| "es".to_string());
                return Some(CommandIntent::Translate { page_id, language });
            }
        }

        // Analyze content intent
        if lower.contains("analyze") {
            return Some(CommandIntent::AnalyzeContent {
                content: text.to_string(),
            });
        }

        None
    }

    /// Extract first number from text
    fn extract_number(text: &str) -> Option<i32> {
        for word in text.split_whitespace() {
            if let Ok(num) = word.parse::<i32>() {
                return Some(num);
            }
        }
        None
    }

    /// Extract URL from text
    fn extract_url(text: &str) -> Option<String> {
        for word in text.split_whitespace() {
            if word.starts_with("http://") || word.starts_with("https://") {
                return Some(word.to_string());
            }
        }
        None
    }

    /// Extract language code from text
    fn extract_language(text: &str) -> Option<String> {
        let lower = text.to_lowercase();
        
        // Common language keywords
        if lower.contains("spanish") || lower.contains("español") {
            Some("es".to_string())
        } else if lower.contains("french") || lower.contains("français") {
            Some("fr".to_string())
        } else if lower.contains("german") || lower.contains("deutsch") {
            Some("de".to_string())
        } else if lower.contains("italian") || lower.contains("italiano") {
            Some("it".to_string())
        } else if lower.contains("portuguese") || lower.contains("português") {
            Some("pt".to_string())
        } else if lower.contains("japanese") || lower.contains("日本語") {
            Some("ja".to_string())
        } else if lower.contains("chinese") || lower.contains("中文") {
            Some("zh".to_string())
        } else {
            None
        }
    }

    /// Get command description
    pub fn describe_intent(intent: &CommandIntent) -> String {
        match intent {
            CommandIntent::GeneratePage { topic } => {
                format!("Generate a new page about '{}'", topic)
            }
            CommandIntent::OptimizeSEO { page_id } => {
                format!("Optimize SEO for page #{}", page_id)
            }
            CommandIntent::Summarize { url } => {
                format!("Summarize content from {}", url)
            }
            CommandIntent::Translate { page_id, language } => {
                format!("Translate page #{} to {}", page_id, language)
            }
            CommandIntent::AnalyzeContent { .. } => {
                "Analyze content".to_string()
            }
            CommandIntent::Unknown => {
                "Unknown command".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slash_command_generate() {
        let cmd = CommandParser::parse("/generate-page How to build a CMS");
        assert!(matches!(cmd.intent, CommandIntent::GeneratePage { .. }));
        if let CommandIntent::GeneratePage { topic } = cmd.intent {
            assert_eq!(topic, "How to build a CMS");
        }
    }

    #[test]
    fn test_slash_command_seo() {
        let cmd = CommandParser::parse("/optimize-seo 42");
        assert!(matches!(cmd.intent, CommandIntent::OptimizeSEO { .. }));
        if let CommandIntent::OptimizeSEO { page_id } = cmd.intent {
            assert_eq!(page_id, 42);
        }
    }

    #[test]
    fn test_natural_language_generate() {
        let cmd = CommandParser::parse("Create a page about Rust programming");
        assert!(matches!(cmd.intent, CommandIntent::GeneratePage { .. }));
    }

    #[test]
    fn test_natural_language_seo() {
        let cmd = CommandParser::parse("Optimize SEO for page 123");
        assert!(matches!(cmd.intent, CommandIntent::OptimizeSEO { .. }));
        if let CommandIntent::OptimizeSEO { page_id } = cmd.intent {
            assert_eq!(page_id, 123);
        }
    }

    #[test]
    fn test_summarize_with_url() {
        let cmd = CommandParser::parse("/summarize https://example.com/article");
        assert!(matches!(cmd.intent, CommandIntent::Summarize { .. }));
    }

    #[test]
    fn test_translate_command() {
        let cmd = CommandParser::parse("/translate 42 es");
        assert!(matches!(cmd.intent, CommandIntent::Translate { .. }));
        if let CommandIntent::Translate { page_id, language } = cmd.intent {
            assert_eq!(page_id, 42);
            assert_eq!(language, "es");
        }
    }

    #[test]
    fn test_language_extraction() {
        assert_eq!(CommandParser::extract_language("translate to spanish"), Some("es".to_string()));
        assert_eq!(CommandParser::extract_language("translate to french"), Some("fr".to_string()));
    }

    #[test]
    fn test_unknown_command() {
        let cmd = CommandParser::parse("Just some random text");
        assert!(matches!(cmd.intent, CommandIntent::Unknown));
    }
}
