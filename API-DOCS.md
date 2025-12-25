# FreeRadical CMS - API Documentation

**Version**: 0.4.0-alpha (Iteration 4)  
**Base URL**: `http://localhost:8080`

---

## 📦 Media Library API

### Upload Media File
Upload an image file with metadata.

```http
POST /api/media/upload
Content-Type: multipart/form-data
```

**Form Fields**:
- `file` (required): The image file to upload
- `alt_text` (optional): Alt text for accessibility
- `caption` (optional): Image caption
- `folder` (optional): Folder/category for organization

**Constraints**:
- Max file size: 10MB
- Allowed types: Images only (JPEG, PNG, GIF, WebP)

**Response** (201 Created):
```json
{
  "uuid": "550e8400-e29b-41d4-a716-446655440000",
  "filename": "550e8400-e29b-41d4-a716-446655440000.jpg",
  "original_filename": "photo.jpg",
  "mime_type": "image/jpeg",
  "file_size": 1024567,
  "width": 1920,
  "height": 1080,
  "storage_path": "uploads/550e8400-e29b-41d4-a716-446655440000.jpg",
  "message": "File uploaded successfully"
}
```

### List Media Files
Get list of uploaded media files.

```http
GET /api/media
```

**Response** (200 OK):
```json
[
  {
    "id": 1,
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "filename": "550e8400-e29b-41d4-a716-446655440000.jpg",
    "original_filename": "photo.jpg",
    "mime_type": "image/jpeg",
    "file_size": 1024567,
    "width": 1920,
    "height": 1080,
    "storage_path": "uploads/...",
    "created_at": "2025-12-24T18:00:00",
    "updated_at": "2025-12-24T18:00:00"
  }
]
```

### Get Media File
Get details of a specific media file.

```http
GET /api/media/:uuid
```

**Response** (200 OK): Single media object

### Delete Media File
Delete a media file (both database record and file from disk).

```http
DELETE /api/media/:uuid
```

**Response** (200 OK):
```json
"Media deleted"
```

---

## 📝 Revision History API

### List Page Revisions
Get all revisions for a specific page.

```http
GET /api/pages/:page_uuid/revisions
```

**Response** (200 OK):
```json
[
  {
    "id": 123,
    "revision_number": 5,
    "change_summary": "Updated header content",
    "created_at": "2025-12-24T18:00:00",
    "changed_by_user_id": 1
  },
  {
    "id": 122,
    "revision_number": 4,
    "change_summary": "Page updated",
    "created_at": "2025-12-24T17:30:00",
    "changed_by_user_id": 1
  }
]
```

### Get Specific Revision
Get full details of a specific revision.

```http
GET /api/pages/:page_uuid/revisions/:revision_number
```

**Response** (200 OK):
```json
{
  "id": 123,
  "page_uuid": "abc-123-def",
  "revision_number": 5,
  "page_title": "My Page",
  "page_url": "/my-page",
  "full_snapshot": "{...complete page state as JSON...}",
  "change_summary": "Updated header content",
  "changed_by_user_id": 1,
  "created_at": "2025-12-24T18:00:00"
}
```

### Rollback to Revision
Restore a page to a previous revision state.

```http
POST /api/pages/:page_uuid/rollback/:revision_number
```

**Response** (200 OK):
```json
{
  "message": "Page rolled back successfully",
  "page_uuid": "abc-123-def",
  "rollback_to_revision": 3,
  "new_revision": 6
}
```

**Note**: Rollback creates a new revision to document the rollback action.

---

## ⏰ Scheduled Publishing

Scheduled publishing runs automatically in the background (no API calls needed).

### How It Works
1. Set page `status` to `"scheduled"` and `publish_at` to future timestamp
2. Scheduler runs every minute checking for pages to publish
3. When `publish_at` time is reached, status automatically changes to `"published"`
4. Optionally set `unpublish_at` to automatically archive at specific time

### Page Status Values
- `draft` - Work in progress
- `scheduled` - Waiting for publish time
- `published` - Live and visible
- `archived` - No longer active

### Example: Schedule a Page
```http
PUT /api/pages/:uuid
Content-Type: application/json
Authorization: Bearer {token}

{
  "status": "scheduled",
  "publish_at": "2025-12-25T09:00:00",
  "unpublish_at": "2025-12-31T23:59:59"
}
```

The page will automatically:
- Change to `published` at 9:00 AM on Dec 25
- Change to `archived` at 11:59 PM on Dec 31

---

## 🔐 Authentication

Most endpoints require authentication. Include JWT token in header:

```http
Authorization: Bearer {your-jwt-token}
```

Get token via:
```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "your-username",
  "password": "your-password"
}
```

---

## 📊 Response Codes

- `200 OK` - Success
- `201 Created` - Resource created
- `400 Bad Request` - Invalid input
- `401 Unauthorized` - Authentication required
- `404 Not Found` - Resource not found
- `413 Payload Too Large` - File too large
- `500 Internal Server Error` - Server error

---

## 🚀 Environment Configuration

Required environment variables:

```bash
# Database
APP_MYSQL_URL=localhost
APP_MYSQL_PORT=3306
APP_MYSQL_USERNAME=root
APP_MYSQL_PASSWORD=password
APP_MYSQL_DATABASE=freeradical

# Server
APP_BASE_URL=http://localhost:8080

# File Upload
UPLOAD_DIR=uploads  # Directory for media files
MAX_FILE_SIZE=10485760  # 10MB in bytes
```

---

**For more information**, see:
- `ITERATION-4-STATUS.md` - Feature status and roadmap
- `walkthrough.md` - Implementation details
- `completion_report.md` - Verification and benchmarks
