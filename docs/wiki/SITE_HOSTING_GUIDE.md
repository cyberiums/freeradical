# FreeRadical CMS - Site Hosting & Integration Guide

## Welcome to FreeRadical CMS! 🚀

This guide will help you host your site on FreeRadical CMS and integrate powerful tools like newsletters, surveys, polls, and forms using our comprehensive API.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Hosting Your Site](#hosting-your-site)
3. [Newsletter Subscriptions](#newsletter-subscriptions)
4. [Surveys & Polls](#surveys--polls)
5. [Contact Forms](#contact-forms)
6. [Analytics & Tracking](#analytics--tracking)
7. [Advanced Integrations](#advanced-integrations)

---

## Getting Started

### Prerequisites

- FreeRadical CMS account
- API access token (get from admin dashboard)
- Basic HTML/JavaScript knowledge

### API Endpoints

- **Base URL:** `https://your-domain.com/v1`
- **Documentation:** `https://your-domain.com/swagger-ui/`
- **Authentication:** Bearer token (for admin endpoints)

---

## Hosting Your Site

### 1. Create Your Site

**Via API:**
```bash
curl -X POST https://your-domain.com/v1/sites \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Awesome Site",
    "subdomain": "mysite"
  }'
```

**Your site will be available at:** `https://mysite.freeradical.com`

### 2. Custom Domain

```bash
# Validate your custom domain
curl -X POST https://your-domain.com/v1/sites/validate-cname \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "domain": "www.yoursite.com"
  }'
```

**DNS Settings:**
```
Type: CNAME
Host: www
Value: mysite.freeradical.com
```

### 3. Deploy Content

Create pages using the Pages API:
```javascript
const page = await fetch('https://your-domain.com/v1/pages', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer YOUR_TOKEN',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    title: 'Home',
    slug: 'home',
    content: '<h1>Welcome!</h1>',
    meta_title: 'Home - My Site',
    meta_description: 'Welcome to my site'
  })
});
```

---

## Newsletter Subscriptions

### Quick Setup (5 minutes)

#### 1. Add HTML Form

```html
<form id="newsletter-form">
  <input type="email" name="email" placeholder="Your email" required>
  <input type="text" name="name" placeholder="Your name">
  <button type="submit">Subscribe</button>
  <div id="message"></div>
</form>
```

#### 2. Add JavaScript

```html
<script>
document.getElementById('newsletter-form').addEventListener('submit', async (e) => {
  e.preventDefault();
  const formData = new FormData(e.target);
  const message = document.getElementById('message');
  
  try {
    const response = await fetch('https://your-domain.com/v1/public/crm/customers', {
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
      message.innerHTML = `<p style="color: green;">✅ ${data.message}</p>`;
      e.target.reset();
    } else {
      message.innerHTML = `<p style="color: red;">❌ ${data.error}</p>`;
    }
  } catch (error) {
    message.innerHTML = '<p style="color: red;">❌ Something went wrong. Please try again.</p>';
  }
});
</script>
```

#### 3. Done! 🎉

Users will receive a verification email. After clicking the link, they're subscribed!

### Managing Subscribers

**List Subscribers:**
```bash
curl https://your-domain.com/v1/api/crm/customers?tags=newsletter_subscriber \
  -H "Authorization: Bearer YOUR_TOKEN"
```

**Create Segment:**
```bash
curl -X POST https://your-domain.com/v1/api/crm/segments \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Newsletter Subscribers",
    "criteria": {"tags": ["newsletter_subscriber"]}
  }'
```

**Send Campaign:**
```bash
curl -X POST https://your-domain.com/v1/api/crm/campaigns \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Weekly Newsletter",
    "segment_id": 123,
    "subject": "Here's what's new this week",
    "content": "..."
  }'
```

---

## Surveys & Polls

### Create a Survey

#### 1. Create Survey

```javascript
const survey = await fetch('https://your-domain.com/v1/surveys', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer YOUR_TOKEN',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    title: 'Customer Satisfaction Survey',
    description: 'Help us improve!',
    status: 'active'
  })
});

const surveyData = await survey.json();
const surveyId = surveyData.id;
```

#### 2. Add Questions

```javascript
// Multiple choice question
await fetch(`https://your-domain.com/v1/surveys/${surveyId}/questions`, {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer YOUR_TOKEN',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    question_text: 'How satisfied are you with our service?',
    question_type: 'multiple_choice',
    options: {
      "choices": [
        "Very Satisfied",
        "Satisfied",
        "Neutral",
        "Unsatisfied",
        "Very Unsatisfied"
      ]
    },
    is_required: true,
    order_index: 1
  })
});

// Text question
await fetch(`https://your-domain.com/v1/surveys/${surveyId}/questions`, {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer YOUR_TOKEN',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    question_text: 'What can we improve?',
    question_type: 'text',
    is_required: false,
    order_index: 2
  })
});
```

#### 3. Display Survey on Your Site

```html
<div id="survey-container"></div>

<script>
async function loadSurvey(surveyId) {
  const response = await fetch(`https://your-domain.com/v1/surveys/${surveyId}`);
  const survey = await response.json();
  
  let html = `<h2>${survey.survey.title}</h2>`;
  html += `<p>${survey.survey.description}</p>`;
  html += '<form id="survey-form">';
  
  survey.questions.forEach((q, index) => {
    html += `<div class="question">`;
    html += `<label>${q.question_text}${q.is_required ? ' *' : ''}</label>`;
    
    if (q.question_type === 'multiple_choice') {
      q.options.choices.forEach(choice => {
        html += `
          <label>
            <input type="radio" name="question_${q.id}" value="${choice}" ${q.is_required ? 'required' : ''}>
            ${choice}
          </label><br>
        `;
      });
    } else if (q.question_type === 'text') {
      html += `<textarea name="question_${q.id}" ${q.is_required ? 'required' : ''}></textarea>`;
    }
    
    html += `</div>`;
  });
  
  html += '<button type="submit">Submit</button></form>';
  document.getElementById('survey-container').innerHTML = html;
  
  // Handle submission
  document.getElementById('survey-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const formData = new FormData(e.target);
    const answers = {};
    
    for (let [key, value] of formData.entries()) {
      answers[key] = value;
    }
    
    await fetch(`https://your-domain.com/v1/surveys/${surveyId}/responses`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(answers)
    });
    
    alert('Thank you for your feedback!');
  });
}

// Load survey (replace with your survey ID)
loadSurvey(123);
</script>
```

### View Results

```bash
curl https://your-domain.com/v1/surveys/123/results \
  -H "Authorization: Bearer YOUR_TOKEN"
```

---

## Contact Forms

### Simple Contact Form with Email Verification

```html
<form id="contact-form">
  <input type="text" name="name" placeholder="Name" required>
  <input type="email" name="email" placeholder="Email" required>
  <input type="text" name="subject" placeholder="Subject" required>
  <textarea name="message" placeholder="Message" required></textarea>
  <button type="submit">Send</button>
  <div id="contact-message"></div>
</form>

<script>
document.getElementById('contact-form').addEventListener('submit', async (e) => {
  e.preventDefault();
  const formData = new FormData(e.target);
  const message = document.getElementById('contact-message');
  
  // Submit as CRM customer with contact form tag
  const response = await fetch('https://your-domain.com/v1/public/crm/customers', {
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
    message.innerHTML = '<p style="color: green;">✅ Message sent! Please verify your email.</p>';
    e.target.reset();
  }
});
</script>
```

---

## Analytics & Tracking

### Track User Interactions

```javascript
// Track page view
await fetch('https://your-domain.com/v1/api/crm/interactions', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer YOUR_TOKEN',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    customer_id: 123,
    interaction_type: 'page_view',
    notes: 'Viewed pricing page',
    metadata: {
      page: window.location.pathname,
      referrer: document.referrer
    }
  })
});
```

### Get Customer Timeline

```bash
curl https://your-domain.com/v1/api/crm/customers/123/timeline \
  -H "Authorization: Bearer YOUR_TOKEN"
```

---

## Advanced Integrations

### AI-Powered Features

#### Sentiment Analysis
```javascript
const sentiment = await fetch('https://your-domain.com/v1/ai/analyze/sentiment', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer YOUR_TOKEN',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    text: 'Customer feedback text...',
    provider_id: 1
  })
});
```

#### Content Recommendations
```javascript
const recommendations = await fetch('https://your-domain.com/v1/recommendations/related', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer YOUR_TOKEN',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    content_id: 456,
    limit: 5
  })
});
```

### Product Catalog

```javascript
// List products
const products = await fetch('https://your-domain.com/v1/products?page=1&limit=20');

// Create order
const order = await fetch('https://your-domain.com/v1/orders', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer YOUR_TOKEN',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    user_id: 123,
    items: [
      { product_id: 1, quantity: 2, price_cents: 2999 }
    ]
  })
});
```

---

## Best Practices

### Security

✅ **Always validate email** - Use verification for public submissions  
✅ **Rate limit forms** - Prevent spam (max 5 submissions/IP/hour)  
✅ **Sanitize input** - Clean user-submitted content  
✅ **Use HTTPS** - Encrypt all API communications  

### Performance

✅ **Cache API responses** - Store frequently accessed data  
✅ **Paginate lists** - Load data in chunks  
✅ **Async operations** - Don't block UI during API calls  
✅ **Compress assets** - Minimize bundle sizes  

### User Experience

✅ **Show loading states** - Indicate API activity  
✅ **Handle errors gracefully** - Display friendly error messages  
✅ **Provide feedback** - Confirm successful actions  

✅ **Mobile responsive** - Test on all device sizes  

---

## Example: Complete Landing Page

See our [Complete Landing Page Example](https://github.com/cyberiums/freeradical/wiki/Landing-Page-Example) for a full implementation with:
- Newsletter subscription
- Contact form
- Product showcase
- Analytics tracking

---

## Support

- **Documentation:** https://your-domain.com/swagger-ui/
- **GitHub:** https://github.com/cyberiums/freeradical
- **Issues:** https://github.com/cybers/freeradical/issues

---

**Happy Building! 🎉**
