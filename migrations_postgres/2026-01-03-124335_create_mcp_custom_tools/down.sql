-- Drop tables in reverse order (respect foreign key constraints)
DROP TABLE IF EXISTS mcp_tool_rate_limits CASCADE;
DROP TABLE IF EXISTS mcp_tool_executions CASCADE;
DROP TABLE IF EXISTS mcp_custom_tools CASCADE;
