<h1 align="center">
  <img src=".github/logo.gif" alt="sniffer from minecraft" width="320">
</h1>

**Sniff** is a specialized API service designed to retrieve Google Play Store app
details across different release channels (Stable, Beta, Alpha). It provides a clean
interface to access app metadata including version information, changelog, download
sizes, and other details for Android applications.

## Features

- **Multi-Channel Support**: Access app details from Stable, Beta, and Alpha channels (where available)
- **Intelligent Track Detection**: Automatically identifies which channels are available for specific apps
- **Unified API**: Simple REST API endpoints for accessing app information

## API Endpoints

### Get App Details (All Available Channels)

```
GET /v1/details/:package_name
```

Returns details for all available channels for the specified package.

**Response Headers:**

- `X-Available-Channels`: Comma-separated list of available channels for the app

### Get App Details (Specific Channel)

```
GET /v1/details/:package_name/:channel
```

Returns details for a specific channel (stable, beta, or alpha) if available.

**Possible channels:**

- `stable` - Production release (always available)
- `beta` - Beta program release (only available for certain apps)
- `alpha` - Alpha program release (only available for certain apps)

## Response Format

Successful responses follow this structure:

```jsonc
{
  "success": true,
  "data": {
    // For multi-channel: track name -> details
    "stable": {
      /* app details */
    },
    "beta": {
      /* app details */
    },
    // Alpha if available
  },
  "error": null,
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
