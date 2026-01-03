
/// Generate hreflang tags for multi-language pages
pub fn generate_hreflang_tags(
    page_uuid: &str,
    _current_lang: &str,  
    base_url: &str
) -> String {
    // Supported languages (ready for database integration)
    let languages = vec!["en", "es", "fr", "de"];
    
    let mut tags = String::new();
    
    for lang in languages {
        let url = format!("{}/{}/{}", base_url, lang, page_uuid);
        tags.push_str(&format!("<link rel=\"alternate\" hreflang=\"{}\" href=\"{}\" />\n", lang, url));
    }
    
    // Add x-default
    let default_url = format!("{}/en/{}", base_url, page_uuid);
    tags.push_str(&format!("<link rel=\"alternate\" hreflang=\"x-default\" href=\"{}\" />", default_url));
    
    tags
}

/// Helper function to insert hreflang into template
pub fn get_hreflang_for_template(
    page_uuid: &str,
    current_lang: &str,
    base_url: &str
) -> String {
    generate_hreflang_tags(page_uuid, current_lang, base_url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hreflang_generation() {
        let tags = generate_hreflang_tags("my-page", "en", "https://example.com");
        assert!(tags.contains("hreflang=\"en\""));
        assert!(tags.contains("hreflang=\"es\""));
        assert!(tags.contains("hreflang=\"x-default\""));
    }
}
