use log::{info, warn};
use serde::{Deserialize, Serialize};

/// AI Response Handler
/// Processes and formats AI responses
pub struct AIResponseHandler;

/// Response format types
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseFormat {
    Markdown,
    PlainText,
    JSON,
    HTML,
}

/// Processed response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedResponse {
    pub content: String,
    pub format: String,
    pub metadata: ResponseMetadata,
}

/// Response metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub word_count: usize,
    pub character_count: usize,
    pub estimated_reading_time_minutes: usize,
    pub has_code_blocks: bool,
    pub has_headers: bool,
    pub has_lists: bool,
}

impl AIResponseHandler {
    /// Process raw AI response
    pub fn process(
        raw_response: &str,
        target_format: ResponseFormat,
    ) -> Result<ProcessedResponse, String> {
        info!("Processing response to {:?} format", target_format);

        // Analyze content
        let metadata = Self::analyze_content(raw_response);

        // Convert format
        let content = match target_format {
            ResponseFormat::Markdown => raw_response.to_string(),
            ResponseFormat::PlainText => Self::markdown_to_plain_text(raw_response),
            ResponseFormat::JSON => Self::to_json(raw_response, &metadata)?,
            ResponseFormat::HTML => Self::markdown_to_html(raw_response),
        };

        Ok(ProcessedResponse {
            content,
            format: format!("{:?}", target_format).to_lowercase(),
            metadata,
        })
    }

    /// Analyze content structure
    fn analyze_content(text: &str) -> ResponseMetadata {
        let words: Vec<&str> = text.split_whitespace().collect();
        let word_count = words.len();
        let character_count = text.len();
        
        // Estimate reading time (average 200 words per minute)
        let estimated_reading_time_minutes = (word_count as f64 / 200.0).ceil() as usize;

        // Detect markdown features
        let has_code_blocks = text.contains("```");
        let has_headers = text.contains("# ") || text.contains("## ") || text.contains("### ");
        let has_lists = text.contains("- ") || text.contains("* ") || text.contains("1. ");

        ResponseMetadata {
            word_count,
            character_count,
            estimated_reading_time_minutes,
            has_code_blocks,
            has_headers,
            has_lists,
        }
    }

    /// Convert markdown to plain text
    fn markdown_to_plain_text(markdown: &str) -> String {
        let mut plain = markdown.to_string();

        // Remove code blocks
        plain = plain.replace("```", "");

        // Remove headers
        plain = plain.replace("# ", "");
        plain = plain.replace("## ", "");
        plain = plain.replace("### ", "");
        plain = plain.replace("#### ", "");
        plain = plain.replace("##### ", "");
        plain = plain.replace("###### ", "");

        // Remove bold/italic
        plain = plain.replace("**", "");
        plain = plain.replace("__", "");
        plain = plain.replace("*", "");
        plain = plain.replace("_", "");

        // Remove links but keep text
        // Simplified link removal
        plain = plain.replace("[", "").replace("]", "");

        plain.trim().to_string()
    }

    /// Convert to JSON structure
    fn to_json(text: &str, metadata: &ResponseMetadata) -> Result<String, String> {
        let response = serde_json::json!({
            "content": text,
            "metadata": metadata,
        });

        serde_json::to_string_pretty(&response)
            .map_err(|e| format!("JSON serialization error: {}", e))
    }

    /// Convert markdown to HTML (basic)
    fn markdown_to_html(markdown: &str) -> String {
        let mut html = markdown.to_string();

        // Headers
        html = html.replace("### ", "<h3>").replace("\n", "</h3>\n");
        html = html.replace("## ", "<h2>").replace("\n", "</h2>\n");
        html = html.replace("# ", "<h1>").replace("\n", "</h1>\n");

        // Bold
        html = html.replace("**", "<strong>").replace("**", "</strong>");

        // Italic
        html = html.replace("*", "<em>").replace("*", "</em>");

        // Paragraphs
        html = format!("<p>{}</p>", html);

        html
    }

    /// Stream processor for real-time responses
    pub fn process_stream_chunk(chunk: &str) -> Result<String, String> {
        // For now, just return the chunk
        // In future, can do real-time processing
        Ok(chunk.to_string())
    }

    /// Validate response quality
    pub fn validate_response(response: &str) -> Result<(), String> {
        if response.trim().is_empty() {
            return Err("Response is empty".to_string());
        }

        if response.len() < 50 {
            warn!("Response is very short ({} chars)", response.len());
        }

        if response.len() > 10000 {
            warn!("Response is very long ({} chars)", response.len());
        }

        Ok(())
    }

    /// Extract code blocks from markdown
    pub fn extract_code_blocks(markdown: &str) -> Vec<CodeBlock> {
        let mut code_blocks = Vec::new();
        let mut in_code_block = false;
        let mut current_language = String::new();
        let mut current_code = String::new();

        for line in markdown.lines() {
            if line.starts_with("```") {
                if in_code_block {
                    // End of code block
                    code_blocks.push(CodeBlock {
                        language: current_language.clone(),
                        code: current_code.trim().to_string(),
                    });
                    current_code.clear();
                    current_language.clear();
                    in_code_block = false;
                } else {
                    // Start of code block
                    current_language = line.trim_start_matches("```").trim().to_string();
                    in_code_block = true;
                }
            } else if in_code_block {
                current_code.push_str(line);
                current_code.push('\n');
            }
        }

        code_blocks
    }

    /// Count tokens (rough estimate)
    pub fn estimate_tokens(text: &str) -> usize {
        // Rough estimate: ~4 characters per token
        text.len() / 4
    }
}

/// Code block extracted from markdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    pub language: String,
    pub code: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_plain_text() {
        let markdown = "# Hello\n\nThis is **bold** text.";
        let plain = AIResponseHandler::markdown_to_plain_text(markdown);
        assert!(!plain.contains("**"));
        assert!(!plain.contains("#"));
    }

    #[test]
    fn test_analyze_content() {
        let text = "# Title\n\nSome content here.";
        let metadata = AIResponseHandler::analyze_content(text);
        assert!(metadata.has_headers);
        assert!(metadata.word_count > 0);
    }

    #[test]
    fn test_extract_code_blocks() {
        let markdown = "```rust\nfn main() {}\n```";
        let blocks = AIResponseHandler::extract_code_blocks(markdown);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].language, "rust");
    }

    #[test]
    fn test_validate_response() {
        assert!(AIResponseHandler::validate_response("Valid response").is_ok());
        assert!(AIResponseHandler::validate_response("").is_err());
    }

    #[test]
    fn test_process_formats() {
        let text = "# Test\n\nContent here.";
        
        let md = AIResponseHandler::process(text, ResponseFormat::Markdown).unwrap();
        assert_eq!(md.format, "markdown");

        let plain = AIResponseHandler::process(text, ResponseFormat::PlainText).unwrap();
        assert_eq!(plain.format, "plaintext");

        let json = AIResponseHandler::process(text, ResponseFormat::JSON).unwrap();
        assert_eq!(json.format, "json");
    }
}
