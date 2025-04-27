<h1 align="center">
  <img src=".github/logo.gif" alt="sniffer from minecraft" width="320">
</h1>

**Sniff** is a specialized API service designed to retrieve Google Play Store app
details across different release tracks (Stable, Beta, Alpha). It provides a clean
interface to access app metadata including version information, changelog, download
sizes, and other details for Android applications.

## Features

- **Multi-Track Support**: Access app details from Stable, Beta, and Alpha tracks (where available)
- **Intelligent Track Detection**: Automatically identifies which tracks are available for specific apps
- **Unified API**: Simple REST API endpoints for accessing app information

## API Endpoints

### Get App Details (All Available Tracks)

```
GET /v1/details/:package_name
```

Returns details for all available tracks for the specified package.

**Response Headers:**

- `X-Available-Tracks`: Comma-separated list of available tracks for the app

### Get App Details (Specific Track)

```
GET /v1/details/:package_name/:track
```

Returns details for a specific track (stable, beta, or alpha) if available.

**Possible tracks:**

- `stable` - Production release (always available)
- `beta` - Beta program release (only available for certain apps)
- `alpha` - Alpha program release (only available for certain apps)

## Response Format

Successful responses follow this structure:

```json
{
  "success": true,
  "data": {
    // For multi-track: track name -> details
    "stable": {
      /* app details */
    },
    "beta": {
      /* app details */
    }
    // Alpha if available
  },
  "error": null
}
```

Error responses:

```json
{
  "success": false,
  "data": null,
  "error": "Error message describing the issue"
}
```

## Deployment

Sniff is designed to be deployed as a Cloudflare Worker, providing global distribution and low-latency access to the API.

## Environment Variables

The following environment variables are required:

- `DEVICE_NAME`: Device identifier for Google Play API
- `STABLE_EMAIL`: Email for stable track access
- `STABLE_AAS_TOKEN`: Authentication token for stable track
- `BETA_EMAIL`: Email enrolled in beta programs
- `BETA_AAS_TOKEN`: Authentication token for beta access
- `ALPHA_EMAIL`: Email enrolled in alpha programs
- `ALPHA_AAS_TOKEN`: Authentication token for alpha access
