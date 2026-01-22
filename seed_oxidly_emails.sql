-- DB Seed for Oxidly Tenant Email Templates (Tenant ID: 1)

INSERT INTO email_templates (tenant_id, template_key, subject, body_template, template_type, is_active, created_at, updated_at)
VALUES 
(
    1, 
    'auth/welcome', 
    'Welcome to Oxidly!', 
    '<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Welcome to {{app_name}}</title>
</head>
<body style="margin: 0; padding: 0; font-family: Arial, sans-serif; background-color: #f4f4f4;">
    <table width="100%" cellpadding="0" cellspacing="0" style="background-color: #f4f4f4; padding: 20px;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0"
                    style="background-color: #ffffff; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <!-- Header -->
                    <tr>
                        <td
                            style="background-color: #10B981; padding: 40px 20px; text-align: center;">
                            <h1 style="color: #ffffff; margin: 0; font-size: 32px;">Welcome to Oxidly!</h1>
                        </td>
                    </tr>
                    <!-- Content -->
                    <tr>
                        <td style="padding: 40px 30px;">
                            <p style="font-size: 16px; color: #333333; line-height: 1.6; margin-bottom: 20px;">
                                Hi {{username}},
                            </p>
                            <p style="font-size: 16px; color: #333333; line-height: 1.6; margin-bottom: 20px;">
                                Welcome to Oxidly, the platform for advanced rust development. Your account is ready.
                            </p>
                            <table width="100%" cellpadding="0" cellspacing="0">
                                <tr>
                                    <td align="center">
                                        <a href="{{dashboard_url}}"
                                            style="display: inline-block; padding: 15px 30px; background-color: #10B981; color: #ffffff; text-decoration: none; border-radius: 5px; font-weight: bold; font-size: 16px;">
                                            Go to Dashboard
                                        </a>
                                    </td>
                                </tr>
                            </table>
                        </td>
                    </tr>
                    <!-- Footer -->
                    <tr>
                        <td
                            style="background-color: #f8f9fa; padding: 20px 30px; text-align: center; border-top: 1px solid #e9ecef;">
                            <p style="font-size: 12px; color: #999999; margin: 0;">
                                © {{app_name}} (Oxidly Tenant). All rights reserved.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>', 
    'html', 
    true, 
    NOW(), 
    NOW()
)
ON CONFLICT (tenant_id, template_key) DO UPDATE SET 
    subject = EXCLUDED.subject,
    body_template = EXCLUDED.body_template,
    updated_at = NOW();

INSERT INTO email_templates (tenant_id, template_key, subject, body_template, template_type, is_active, created_at, updated_at)
VALUES 
(
    1, 
    'billing/invoice_paid', 
    'Oxidly: Payment Received', 
    '<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { background-color: #10B981; padding: 20px; text-align: center; color: white; }
        .content { padding: 20px; }
        .amount { font-size: 24px; font-weight: bold; color: #10B981; }
        .details { background-color: #f8f9fa; padding: 15px; border-radius: 5px; margin: 20px 0; }
        .footer { text-align: center; font-size: 12px; color: #666; margin-top: 20px; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h2>Payment Successful (Oxidly)</h2>
        </div>
        <div class="content">
            <p>Hello,</p>
            <p>We successfully processed your payment for the latest invoice.</p>
            <div class="details">
                <p>Invoice: <strong>{{invoice_number}}</strong></p>
                <p>Amount: <span class="amount">${{amount}}</span></p>
                <p>Date: {{date}}</p>
            </div>
            <p>You can view and download your invoice from your dashboard.</p>
        </div>
        <div class="footer">
            <p>© {{year}} Oxidly. All rights reserved.</p>
        </div>
    </div>
</body>
</html>', 
    'html', 
    true, 
    NOW(), 
    NOW()
)
ON CONFLICT (tenant_id, template_key) DO UPDATE SET 
    subject = EXCLUDED.subject,
    body_template = EXCLUDED.body_template,
    updated_at = NOW();
