-- Custom MCP Tools Table
CREATE TABLE mcp_custom_tools (
    id SERIAL PRIMARY KEY,
    tenant_id INT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Tool Metadata
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50) DEFAULT '1.0.0',
    
    -- MCP Tool Definition (JSON Schema)
    input_schema JSONB NOT NULL,  -- JSON Schema for tool inputs
    
    -- HTTP Webhook Execution (200% Isolation - runs externally!)
    webhook_url VARCHAR(1000) NOT NULL,
    webhook_method VARCHAR(10) DEFAULT 'POST',  -- GET, POST, PUT, PATCH
    webhook_headers JSONB DEFAULT '{}',  -- Custom headers for webhook
    
    -- Security & Isolation
    required_role VARCHAR(50) DEFAULT 'editor',  -- 'viewer', 'editor', 'admin'
    timeout_seconds INT DEFAULT 30,  -- Maximum execution time
    max_calls_per_hour INT DEFAULT 100,  -- Rate limit
    
    -- Status
    is_enabled BOOL DEFAULT true,
    is_public BOOL DEFAULT false,  -- If true, available to all tenants (marketplace)
    
    -- Metadata
    created_by INT REFERENCES users(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_used_at TIMESTAMP,
    total_calls INT DEFAULT 0,
    success_count INT DEFAULT 0,
    error_count INT DEFAULT 0,
    avg_execution_ms INT DEFAULT 0,
    
    -- Unique constraint: one tool name per tenant
    UNIQUE(tenant_id, name)
);

-- Indexes for performance
CREATE INDEX idx_mcp_custom_tools_tenant_id ON mcp_custom_tools(tenant_id);
CREATE INDEX idx_mcp_custom_tools_is_enabled ON mcp_custom_tools(is_enabled);
CREATE INDEX idx_mcp_custom_tools_is_public ON mcp_custom_tools(is_public);

-- Tool Execution Logs (for debugging, analytics, and audit trail)
CREATE TABLE mcp_tool_executions (
    id SERIAL PRIMARY KEY,
    tool_id INT NOT NULL REFERENCES mcp_custom_tools(id) ON DELETE CASCADE,
    tenant_id INT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id INT REFERENCES users(id),
    
    -- Execution Details
    input_data JSONB,
    output_data JSONB,
    error_message TEXT,
    execution_time_ms INT,
    http_status_code INT,  -- HTTP response code from webhook
    
    -- Status
    status VARCHAR(50),  -- 'success', 'error', 'timeout', 'rate_limited'
    
    -- Timestamps
    executed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for analytics and debugging
CREATE INDEX idx_mcp_tool_executions_tool_id ON mcp_tool_executions(tool_id);
CREATE INDEX idx_mcp_tool_executions_tenant_id ON mcp_tool_executions(tenant_id);
CREATE INDEX idx_mcp_tool_executions_executed_at ON mcp_tool_executions(executed_at);
CREATE INDEX idx_mcp_tool_executions_status ON mcp_tool_executions(status);

-- Rate Limiting Table (tracks calls per hour)
CREATE TABLE mcp_tool_rate_limits (
    id SERIAL PRIMARY KEY,
    tool_id INT NOT NULL REFERENCES mcp_custom_tools(id) ON DELETE CASCADE,
    tenant_id INT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Rate Limit Tracking
    window_start TIMESTAMP NOT NULL,
    call_count INT DEFAULT 0,
    
    -- Composite index for quick lookups
    UNIQUE(tool_id, tenant_id, window_start)
);

CREATE INDEX idx_mcp_tool_rate_limits_window_start ON mcp_tool_rate_limits(window_start);

-- Comments for documentation
COMMENT ON TABLE mcp_custom_tools IS 'Custom MCP tools registered by tenants. Webhooks execute externally for 200% isolation.';
COMMENT ON COLUMN mcp_custom_tools.webhook_url IS 'External webhook URL - code executes outside our infrastructure for maximum isolation';
COMMENT ON COLUMN mcp_custom_tools.timeout_seconds IS 'Maximum webhook response time before timeout (isolation protection)';
COMMENT ON COLUMN mcp_custom_tools.max_calls_per_hour IS 'Rate limit to prevent abuse (isolation protection)';

COMMENT ON TABLE mcp_tool_executions IS 'Audit log of all custom tool executions for debugging and analytics';
COMMENT ON TABLE mcp_tool_rate_limits IS 'Enforces per-tenant rate limiting for custom tools (protection layer)';
