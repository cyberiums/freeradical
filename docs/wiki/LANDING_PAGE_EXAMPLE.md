# Complete Landing Page Example

This example demonstrates a complete landing page integration with FreeRadical CMS, including newsletter subscription, contact form, survey, and analytics.

## Live Demo

**Features:**
- ✅ Newsletter subscription with email verification
- ✅ Contact form with anti-spam
- ✅ Customer satisfaction survey
- ✅ Analytics tracking
- ✅ Mobile responsive design

---

## Complete HTML Page

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Welcome to Our Site</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            color: #333;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 20px;
        }
        
        /* Hero Section */
        .hero {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 100px 0;
            text-align: center;
        }
        
        .hero h1 {
            font-size: 3rem;
            margin-bottom: 20px;
        }
        
        .hero p {
            font-size: 1.3rem;
            margin-bottom: 30px;
        }
        
        /* Newsletter Section */
        .newsletter {
            background: #f4f4f4;
            padding: 60px 0;
        }
        
        .newsletter h2 {
            text-align: center;
            margin-bottom: 30px;
            font-size: 2rem;
        }
        
        .form-group {
            margin-bottom: 20px;
        }
        
        input[type="text"],
        input[type="email"],
        textarea {
            width: 100%;
            padding: 12px;
            border: 1px solid #ddd;
            border-radius: 4px;
            font-size: 16px;
        }
        
        button {
            background: #667eea;
            color: white;
            padding: 12px 30px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 16px;
            transition: background 0.3s;
        }
        
        button:hover {
            background: #5568d3;
        }
        
        .message {
            margin-top: 15px;
            padding: 10px;
            border-radius: 4px;
        }
        
        .success {
            background: #d4edda;
            color: #155724;
            border: 1px solid #c3e6cb;
        }
        
        .error {
            background: #f8d7da;
            color: #721c24;
            border: 1px solid #f5c6cb;
        }
        
        /* Contact Section */
        .contact {
            padding: 60px 0;
        }
        
        /* Survey Section */
        .survey {
            background: #f9f9f9;
            padding: 60px 0;
        }
        
        .question {
            margin-bottom: 25px;
        }
        
        .question label {
            display: block;
            margin-bottom: 10px;
            font-weight: 600;
        }
        
        .radio-group label {
            font-weight: normal;
            margin-bottom: 5px;
        }
        
        /* Footer */
        footer {
            background: #333;
            color: white;
            text-align: center;
            padding: 30px 0;
        }
    </style>
</head>
<body>
    <!-- Hero Section -->
    <section class="hero">
        <div class="container">
            <h1>Welcome to Our Amazing Platform</h1>
            <p>The best solution for your business needs</p>
        </div>
    </section>

    <!-- Newsletter Section -->
    <section class="newsletter">
        <div class="container">
            <h2>Subscribe to Our Newsletter</h2>
            <form id="newsletter-form" style="max-width: 500px; margin: 0 auto;">
                <div class="form-group">
                    <input type="email" name="email" placeholder="Your email address" required>
                </div>
                <div class="form-group">
                    <input type="text" name="name" placeholder="Your name">
                </div>
                <button type="submit">Subscribe</button>
                <div id="newsletter-message"></div>
            </form>
        </div>
    </section>

    <!-- Contact Form Section -->
    <section class="contact">
        <div class="container">
            <h2 style="text-align: center; margin-bottom: 30px;">Get In Touch</h2>
            <form id="contact-form" style="max-width: 600px; margin: 0 auto;">
                <div class="form-group">
                    <input type="text" name="name" placeholder="Name" required>
                </div>
                <div class="form-group">
                    <input type="email" name="email" placeholder="Email" required>
                </div>
                <div class="form-group">
                    <input type="text" name="subject" placeholder="Subject" required>
                </div>
                <div class="form-group">
                    <textarea name="message" rows="5" placeholder="Message" required></textarea>
                </div>
                <button type="submit">Send Message</button>
                <div id="contact-message"></div>
            </form>
        </div>
    </section>

    <!-- Survey Section -->
    <section class="survey">
        <div class="container">
            <h2 style="text-align: center; margin-bottom: 30px;">Quick Survey</h2>
            <div id="survey-container" style="max-width: 600px; margin: 0 auto;"></div>
        </div>
    </section>

    <!-- Footer -->
    <footer>
        <div class="container">
            <p>&copy; 2024 Your Company. All rights reserved.</p>
        </div>
    </footer>

    <script>
        // Configuration
        const API_BASE = 'https://your-domain.com/v1';
        const API_TOKEN = 'YOUR_API_TOKEN'; // Only for admin endpoints
        
        // Newsletter Form Handler
        document.getElementById('newsletter-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const formData = new FormData(e.target);
            const messageDiv = document.getElementById('newsletter-message');
            
            try {
                const response = await fetch(`${API_BASE}/public/crm/customers`, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        email: formData.get('email'),
                        first_name: formData.get('name'),
                        lifecycle_stage: 'lead',
                        tags: ['newsletter_subscriber'],
                        source: 'website_signup'
                    })
                });
                
                const data = await response.json();
                
                if (response.ok) {
                    messageDiv.className = 'message success';
                    messageDiv.textContent = `✅ ${data.message}`;
                    e.target.reset();
                    
                    // Track conversion
                    trackEvent('newsletter_signup', { email: formData.get('email') });
                } else {
                    messageDiv.className = 'message error';
                    messageDiv.textContent = `❌ ${data.error || 'Something went wrong'}`;
                }
            } catch (error) {
                messageDiv.className = 'message error';
                messageDiv.textContent = '❌ Network error. Please try again.';
            }
        });
        
        // Contact Form Handler
        document.getElementById('contact-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const formData = new FormData(e.target);
            const messageDiv = document.getElementById('contact-message');
            
            try {
                const response = await fetch(`${API_BASE}/public/crm/customers`, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        email: formData.get('email'),
                        first_name: formData.get('name'),
                        lifecycle_stage: 'lead',
                        tags: ['contact_form'],
                        source: 'contact_form',
                        metadata: {
                            subject: formData.get('subject'),
                            message: formData.get('message'),
                            submitted_at: new Date().toISOString()
                        }
                    })
                });
                
                const data = await response.json();
                
                if (response.ok) {
                    messageDiv.className = 'message success';
                    messageDiv.textContent = '✅ Message sent! Please verify your email to complete.';
                    e.target.reset();
                    
                    // Track conversion
                    trackEvent('contact_form_submit', { subject: formData.get('subject') });
                } else {
                    messageDiv.className = 'message error';
                    messageDiv.textContent = `❌ ${data.error || 'Failed to send message'}`;
                }
            } catch (error) {
                messageDiv.className = 'message error';
                messageDiv.textContent = '❌ Network error. Please try again.';
            }
        });
        
        // Load Survey
        async function loadSurvey() {
            try {
                const response = await fetch(`${API_BASE}/surveys/123`); // Replace with your survey ID
                const survey = await response.json();
                
                let html = `<h3>${survey.survey.title}</h3>`;
                html += `<form id="survey-form">`;
                
                survey.questions.forEach((q) => {
                    html += `<div class="question">`;
                    html += `<label>${q.question_text}${q.is_required ? ' *' : ''}</label>`;
                    
                    if (q.question_type === 'multiple_choice') {
                        html += '<div class="radio-group">';
                        q.options.choices.forEach(choice => {
                            html += `
                                <label>
                                    <input type="radio" name="q_${q.id}" value="${choice}" ${q.is_required ? 'required' : ''}>
                                    ${choice}
                                </label><br>
                            `;
                        });
                        html += '</div>';
                    } else {
                        html += `<textarea name="q_${q.id}" ${q.is_required ? 'required' : ''}></textarea>`;
                    }
                    
                    html += `</div>`;
                });
                
                html += '<button type="submit">Submit Survey</button><div id="survey-message"></div></form>';
                document.getElementById('survey-container').innerHTML = html;
                
                // Survey submit handler
                document.getElementById('survey-form').addEventListener('submit', async (e) => {
                    e.preventDefault();
                    const formData = new FormData(e.target);
                    const messageDiv = document.getElementById('survey-message');
                    const answers = {};
                    
                    for (let [key, value] of formData.entries()) {
                        answers[key] = value;
                    }
                    
                    try {
                        await fetch(`${API_BASE}/surveys/123/responses`, {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify(answers)
                        });
                        
                        messageDiv.className = 'message success';
                        messageDiv.textContent = '✅ Thank you for your feedback!';
                        e.target.reset();
                        
                        // Track completion
                        trackEvent('survey_completed', { survey_id: 123 });
                    } catch (error) {
                        messageDiv.className = 'message error';
                        messageDiv.textContent = '❌ Failed to submit. Please try again.';
                    }
                });
            } catch (error) {
                document.getElementById('survey-container').innerHTML = 
                    '<p>Survey temporarily unavailable.</p>';
            }
        }
        
        // Analytics Tracking
        function trackEvent(eventName, data = {}) {
            // Track page events (when user is authenticated)
            // You would need to get customer_id from session/cookie
            console.log('Event tracked:', eventName, data);
            
            // Example: Send to your analytics endpoint
            /*
            fetch(`${API_BASE}/api/crm/interactions`, {
                method: 'POST',
                headers: {
                    'Authorization': `Bearer ${API_TOKEN}`,
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    customer_id: getCustomerId(),
                    interaction_type: eventName,
                    metadata: data
                })
            });
            */
        }
        
        // Track page view on load
        trackEvent('page_view', {
            page: window.location.pathname,
            referrer: document.referrer
        });
        
        // Load survey on page load
        loadSurvey();
    </script>
</body>
</html>
```

---

## Key Features Demonstrated

### 1. Newsletter Subscription ✅
- Email validation
- Double opt-in verification
- Success/error handling
- Event tracking

### 2. Contact Form ✅
- Multi-field form
- Metadata storage
- Spam protection via verification
- User feedback

### 3. Survey Integration ✅
- Dynamic survey loading
- Multiple question types
- Required fields validation
- Response submission

### 4. Analytics Tracking ✅
- Page view tracking
- Event tracking
- Conversion tracking
- Metadata collection

---

## Customization

### Change API Endpoint
```javascript
const API_BASE = 'https://your-custom-domain.com/v1';
```

### Update Survey ID
```javascript
loadSurvey(456); // Your survey ID
```

### Add Custom Styling
Modify the `<style>` section to match your brand colors and design.

### Track Custom Events
```javascript
trackEvent('custom_event_name', {
    custom_field: 'value'
});
```

---

## Deployment

### Option 1: FreeRadical Hosting
1. Create page via API
2. Set content to this HTML
3. Publish

### Option 2: External Hosting
1. Upload HTML to your server
2. Update API_BASE to point to your FreeRadical instance
3. Configure CORS if needed

### Option 3: Static Site Generator
Integrate this code into your Next.js, Hugo, or Jekyll site.

---

## Testing

1. **Newsletter:** Submit email, check inbox for verification
2. **Contact Form:** Submit message, verify email sent
3. **Survey:** Complete survey, check admin panel for responses
4. **Analytics:** Check interaction logs in CRM

---

## Production Checklist

- [ ] Replace `API_BASE` with production URL
- [ ] Remove or secure `API_TOKEN` (don't expose in frontend)
- [ ] Update survey ID to real survey
- [ ] Configure rate limiting
- [ ] Set up email templates
- [ ] Test on mobile devices
- [ ] Configure TTL settings
- [ ] Set up monitoring

---

## Support

Questions? Check the [Site Hosting Guide](https://github.com/cyberiums/freeradical/wiki/SITE_HOSTING_GUIDE) or open an issue.

**Happy coding! 🚀**
