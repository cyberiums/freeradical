# Performance Monitoring Setup Guide

**Version**: 0.4.0-alpha  
**Date**: December 24, 2025

---

## 📊 Overview

FreeRadical now includes comprehensive performance monitoring for all Iteration 4 features:
- Media Library upload tracking
- Revision History activity
- Scheduled Publishing automation
- Real-time metrics via API

---

## 🎯 Monitoring Endpoints

### Get Full Metrics
```http
GET /api/metrics

Response:
{
  "server": {
    "start_time": "2025-12-24T18:00:00Z",
    "uptime_seconds": 3600,
    "uptime_hours": 1
  },
  "media_library": {
    "uploads_total": 42,
    "uploads_failed": 2,
    "bytes_uploaded": 52428800,
    "avg_upload_time_ms": 220,
    "success_rate": 95.2
  },
  "revision_history": {
    "revisions_created": 127,
    "rollbacks_performed": 5,
    "rollbacks_failed": 0,
    "rollback_success_rate": 100.0
  },
  "scheduled_publishing": {
    "scheduler_runs": 60,
    "pages_auto_published": 12,
    "pages_auto_archived": 3,
    "scheduler_errors": 0,
    "error_rate": 0.0
  }
}
```

### Health Check
```http
GET /api/health

Response:
{
  "status": "healthy",
  "uptime_seconds": 3600,
  "features": {
    "media_library": "operational",
    "revision_history": "operational",
    "scheduled_publishing": "operational"
  }
}
```

---

## 📈 Tracked Metrics

### Media Library
- **uploads_total**: Total files uploaded
- **uploads_failed**: Failed upload attempts
- **bytes_uploaded**: Total storage used  
- **avg_upload_time_ms**: Average upload duration
- **success_rate**: Upload success percentage

### Revision History
- **revisions_created**: Auto-saved revisions
- **rollbacks_performed**: Successful rollbacks
- **rollbacks_failed**: Failed rollback attempts
- **rollback_success_rate**: Rollback success percentage

### Scheduled Publishing
- **scheduler_runs**: Total scheduler executions
- **pages_auto_published**: Pages auto-published
- **pages_auto_archived**: Pages auto-archived
- **scheduler_errors**: Scheduler failures
- **error_rate**: Scheduler error percentage

---

## 🔍 Monitoring Best Practices

### Production Deployment
1. **Monitor `/api/health`** - Set up uptime monitoring (e.g., Pingdom, UptimeRobot)
2. **Track `/api/ metrics`** - Pull metrics every 5 minutes for dashboards
3. **Log Analysis** - Review scheduler logs for patterns
4. **Alert Thresholds**:
   - Upload success rate < 90%
   - Rollback failure rate > 5%
   - Scheduler error rate > 1%

### Log Monitoring
```bash
# Watch scheduler activity
tail -f logs/freeradical.log | grep "📅\|✅\|📦"

# Watch metrics summary (logged every hour)
tail -f logs/freeradical.log | grep "📊 Performance Metrics"
```

### Grafana/Prometheus Setup
To integrate with monitoring tools:
1. Query `/api/metrics` endpoint periodically
2. Parse JSON response
3. Feed to time-series database
4. Create dashboards for visualization

---

## 🚨 Alert Examples

### High Upload Failure Rate
```
if media_library.success_rate < 90:
  alert("Media uploads degraded - check disk space")
```

### Scheduler Errors
```
if scheduled_publishing.error_rate > 1:
  alert("Scheduler experiencing issues - check database connection")
```

### Disk Space (Manual Check)
```bash
df -h uploads/
# Alert if > 80% full
```

---

## 📊 Metrics Dashboard (Example)

Create a simple dashboard with:

**Widget 1: Uptime**
- Server uptime in hours
- Status: healthy/degraded

**Widget 2: Media Library**
- Total uploads (last 24h)
- Success rate trend
- Average upload time

**Widget 3: Content Activity**
- Revisions created (last 24h)
- Rollbacks performed
- Pages published automatically

**Widget 4: Scheduler Health**
- Last run timestamp
- Pages processed
- Error count

---

## 🔧 Integration with APM Tools

### New Relic
```rust
// Add instrumentation in controllers
newrelic::web_transaction!("Media Upload");
```

### Datadog
```rust
// Track custom metrics
datadog::statsd::increment("media.upload.success");
```

### Sentry
```rust
// Already integrated for error tracking
sentry::capture_message("Scheduler error", sentry::Level::Error);
```

---

## 📝 Logging Configuration

All features log activity:

**Media Library**:
```
[INFO] 📦 Media uploaded: uuid=abc-123, size=1.2MB, time=215ms
[WARN] ❌ Upload failed: File too large (15MB > 10MB limit)
```

**Revision History**:
```
[INFO] 📝 Created revision 5 for page abc-123
[INFO] ↩️ Rolled back page abc-123 to revision 3
```

**Scheduler**:
```
[INFO] 📅 Scheduled publishing service started
[INFO] ✅ Auto-published 2 page(s)
[INFO] 📦 Auto-archived 1 page(s)
```

---

## 🎯 Performance Targets

### Expected Metrics (Production)
- **Upload success rate**: > 95%
- **Average upload time**: < 300ms (for 1MB file)
- **Rollback success rate**: > 99%
- **Scheduler uptime**: 100% (runs every minute)
- **Scheduler error rate**: < 0.5%

### Scaling Considerations
- **> 1000 uploads/day**: Consider CDN integration
- **> 10,000 revisions**: Implement retention policy
- **> 100 scheduled pages**: Optimize scheduler query

---

**Setup Complete**: ✅  
**Endpoints Available**: `/api/metrics`, `/api/health`  
**Next**: Configure external monitoring tools
