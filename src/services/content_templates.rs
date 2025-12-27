use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI Content Templates
/// Pre-built templates for structured content generation
pub struct ContentTemplateLibrary;

/// Content template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub template_type: TemplateType,
    pub prompt_template: String,
    pub sections: Vec<TemplateSection>,
    pub metadata: TemplateMetadata,
}

/// Template types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TemplateType {
    BlogPost,
    ProductDescription,
    LandingPage,
    EmailCampaign,
    SocialMediaPost,
    PressRelease,
    TechnicalDocumentation,
    Custom(String),
}

/// Template section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSection {
    pub id: String,
    pub name: String,
    pub order: u32,
    pub required: bool,
    pub prompt: String,
    pub max_words: Option<usize>,
}

/// Template metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    pub seo_optimized: bool,
    pub target_word_count: usize,
    pub tone: String,
    pub style: String,
    pub tags: Vec<String>,
}

impl ContentTemplateLibrary {
    /// Get blog post template
    pub fn blog_post() -> ContentTemplate {
        ContentTemplate {
            id: "blog_post_standard".to_string(),
            name: "Standard Blog Post".to_string(),
            description: "Professional blog post with SEO optimization".to_string(),
            template_type: TemplateType::BlogPost,
            prompt_template: "Write a comprehensive blog post about {topic}. \
                Target audience: {audience}. Tone: {tone}. \
                Include practical examples and actionable insights.".to_string(),
            sections: vec![
                TemplateSection {
                    id: "title".to_string(),
                    name: "Title".to_string(),
                    order: 1,
                    required: true,
                    prompt: "Create an engaging, SEO-optimized title (50-60 characters)".to_string(),
                    max_words: Some(10),
                },
                TemplateSection {
                    id: "introduction".to_string(),
                    name: "Introduction".to_string(),
                    order: 2,
                    required: true,
                    prompt: "Write a compelling introduction that hooks the reader".to_string(),
                    max_words: Some(150),
                },
                TemplateSection {
                    id: "main_content".to_string(),
                    name: "Main Content".to_string(),
                    order: 3,
                    required: true,
                    prompt: "Develop the main content with 3-5 key sections, examples, and evidence".to_string(),
                    max_words: Some(800),
                },
                TemplateSection {
                    id: "conclusion".to_string(),
                    name: "Conclusion".to_string(),
                    order: 4,
                    required: true,
                    prompt: "Summarize key points and provide clear next steps".to_string(),
                    max_words: Some(150),
                },
            ],
            metadata: TemplateMetadata {
                seo_optimized: true,
                target_word_count: 1200,
                tone: "professional".to_string(),
                style: "informative".to_string(),
                tags: vec!["blog".to_string(), "seo".to_string()],
            },
        }
    }

    /// Get product description template
    pub fn product_description() -> ContentTemplate {
        ContentTemplate {
            id: "product_desc_ecommerce".to_string(),
            name: "E-commerce Product Description".to_string(),
            description: "Conversion-focused product description".to_string(),
            template_type: TemplateType::ProductDescription,
            prompt_template: "Write a compelling product description for {product_name}. \
                Highlight key features, benefits, and unique selling points. \
                Target customer: {target_customer}.".to_string(),
            sections: vec![
                TemplateSection {
                    id: "headline".to_string(),
                    name: "Product Headline".to_string(),
                    order: 1,
                    required: true,
                    prompt: "Create a benefit-driven headline".to_string(),
                    max_words: Some(15),
                },
                TemplateSection {
                    id: "overview".to_string(),
                    name: "Product Overview".to_string(),
                    order: 2,
                    required: true,
                    prompt: "Brief compelling overview of the product".to_string(),
                    max_words: Some(100),
                },
                TemplateSection {
                    id: "features".to_string(),
                    name: "Key Features".to_string(),
                    order: 3,
                    required: true,
                    prompt: "List 5-7 key features with brief explanations".to_string(),
                    max_words: Some(200),
                },
                TemplateSection {
                    id: "benefits".to_string(),
                    name: "Benefits".to_string(),
                    order: 4,
                    required: true,
                    prompt: "Explain how features translate to customer benefits".to_string(),
                    max_words: Some(150),
                },
            ],
            metadata: TemplateMetadata {
                seo_optimized: true,
                target_word_count: 500,
                tone: "persuasive".to_string(),
                style: "benefit-focused".to_string(),
                tags: vec!["ecommerce".to_string(), "product".to_string()],
            },
        }
    }

    /// Get landing page template
    pub fn landing_page() -> ContentTemplate {
        ContentTemplate {
            id: "landing_page_conversion".to_string(),
            name: "High-Converting Landing Page".to_string(),
            description: "Conversion-optimized landing page copy".to_string(),
            template_type: TemplateType::LandingPage,
            prompt_template: "Create landing page copy for {offer}. \
                Goal: {goal}. Target: {target_audience}. \
                Focus on benefits, social proof, and clear CTA.".to_string(),
            sections: vec![
                TemplateSection {
                    id: "hero".to_string(),
                    name: "Hero Section".to_string(),
                    order: 1,
                    required: true,
                    prompt: "Attention-grabbing headline and subheadline".to_string(),
                    max_words: Some(30),
                },
                TemplateSection {
                    id: "value_prop".to_string(),
                    name: "Value Proposition".to_string(),
                    order: 2,
                    required: true,
                    prompt: "Clear explanation of unique value".to_string(),
                    max_words: Some(100),
                },
                TemplateSection {
                    id: "benefits".to_string(),
                    name: "Benefits".to_string(),
                    order: 3,
                    required: true,
                    prompt: "3-5 key benefits with explanations".to_string(),
                    max_words: Some(200),
                },
                TemplateSection {
                    id: "social_proof".to_string(),
                    name: "Social Proof".to_string(),
                    order: 4,
                    required: false,
                    prompt: "Testimonial suggestions and trust indicators".to_string(),
                    max_words: Some(150),
                },
                TemplateSection {
                    id: "cta".to_string(),
                    name: "Call to Action".to_string(),
                    order: 5,
                    required: true,
                    prompt: "Compelling CTA copy".to_string(),
                    max_words: Some(20),
                },
            ],
            metadata: TemplateMetadata {
                seo_optimized: true,
                target_word_count: 600,
                tone: "persuasive".to_string(),
                style: "conversion-focused".to_string(),
                tags: vec!["landing".to_string(), "conversion".to_string()],
            },
        }
    }

    /// Get email campaign template
    pub fn email_campaign() -> ContentTemplate {
        ContentTemplate {
            id: "email_newsletter".to_string(),
            name: "Email Newsletter".to_string(),
            description: "Engaging email newsletter template".to_string(),
            template_type: TemplateType::EmailCampaign,
            prompt_template: "Write an email newsletter about {topic}. \
                Goal: {goal}. Audience: {audience}. \
                Keep it concise, scannable, and action-oriented.".to_string(),
            sections: vec![
                TemplateSection {
                    id: "subject".to_string(),
                    name: "Subject Line".to_string(),
                    order: 1,
                    required: true,
                    prompt: "Compelling subject line (40-50 characters)".to_string(),
                    max_words: Some(10),
                },
                TemplateSection {
                    id: "preheader".to_string(),
                    name: "Preheader Text".to_string(),
                    order: 2,
                    required: true,
                    prompt: "Supporting preheader text".to_string(),
                    max_words: Some(15),
                },
                TemplateSection {
                    id: "body".to_string(),
                    name: "Email Body".to_string(),
                    order: 3,
                    required: true,
                    prompt: "Main content with clear sections and CTAs".to_string(),
                    max_words: Some(300),
                },
            ],
            metadata: TemplateMetadata {
                seo_optimized: false,
                target_word_count: 350,
                tone: "friendly".to_string(),
                style: "conversational".to_string(),
                tags: vec!["email".to_string(), "newsletter".to_string()],
            },
        }
    }

    /// Get all available templates
    pub fn list_templates() -> Vec<ContentTemplate> {
        vec![
            Self::blog_post(),
            Self::product_description(),
            Self::landing_page(),
            Self::email_campaign(),
        ]
    }

    /// Get template by ID
    pub fn get_template(id: &str) -> Option<ContentTemplate> {
        Self::list_templates()
            .into_iter()
            .find(|t| t.id == id)
    }

    /// Render template with variables
    pub fn render_prompt(
        template: &ContentTemplate,
        variables: &HashMap<String, String>,
    ) -> String {
        let mut prompt = template.prompt_template.clone();
        
        for (key, value) in variables {
            let placeholder = format!("{{{}}}", key);
            prompt = prompt.replace(&placeholder, value);
        }

        prompt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blog_post_template() {
        let template = ContentTemplateLibrary::blog_post();
        assert_eq!(template.template_type, TemplateType::BlogPost);
        assert_eq!(template.sections.len(), 4);
        assert!(template.metadata.seo_optimized);
    }

    #[test]
    fn test_product_description_template() {
        let template = ContentTemplateLibrary::product_description();
        assert_eq!(template.template_type, TemplateType::ProductDescription);
        assert!(template.sections.iter().any(|s| s.id == "features"));
    }

    #[test]
    fn test_render_prompt() {
        let template = ContentTemplateLibrary::blog_post();
        let mut vars = HashMap::new();
        vars.insert("topic".to_string(), "AI in Healthcare".to_string());
        vars.insert("audience".to_string(), "doctors".to_string());
        vars.insert("tone".to_string(), "professional".to_string());

        let rendered = ContentTemplateLibrary::render_prompt(&template, &vars);
        assert!(rendered.contains("AI in Healthcare"));
        assert!(rendered.contains("doctors"));
    }

    #[test]
    fn test_list_templates() {
        let templates = ContentTemplateLibrary::list_templates();
        assert_eq!(templates.len(), 4);
    }

    #[test]
    fn test_get_template_by_id() {
        let template = ContentTemplateLibrary::get_template("blog_post_standard");
        assert!(template.is_some());
        assert_eq!(template.unwrap().name, "Standard Blog Post");
    }
}
