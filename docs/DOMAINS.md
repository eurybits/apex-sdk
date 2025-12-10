# Apex SDK Domain Structure

Official documentation for all Apex SDK domains and subdomains.

## Primary Domains

### Main Site
- **[apexsdk.dev](https://apexsdk.dev)** - Main documentation hub with full SDK documentation

### WWW Redirect
- **[www.apexsdk.dev](https://www.apexsdk.dev)** - Automatically redirects to `apexsdk.dev`

## Subdomain Navigation

Convenient shortcuts to specific documentation sections:

### Documentation Subdomains

| Subdomain | Target | Purpose |
|-----------|--------|---------|
| [start.apexsdk.dev](https://start.apexsdk.dev) | Quick Start Guide | Get started in 5 minutes |
| [api.apexsdk.dev](https://api.apexsdk.dev) | API Reference | Complete API documentation |
| [cli.apexsdk.dev](https://cli.apexsdk.dev) | CLI Guide | Command-line interface reference |
| [docs.apexsdk.dev](https://docs.apexsdk.dev) | Documentation Mirror | Full documentation site mirror |
| [blog.apexsdk.dev](https://blog.apexsdk.dev) | Roadmap | Development roadmap and blog updates |
| [play.apexsdk.dev](https://play.apexsdk.dev) | Interactive Viewer | Documentation viewer and playground |

## Technical Details

### DNS Configuration

All subdomains are configured as CNAME records pointing to `apex-sdk.pages.dev`:

```
Type: CNAME
Name: <subdomain>
Target: apex-sdk.pages.dev
Proxy: Enabled
```

### Redirect Rules

Redirects are managed via Cloudflare Pages `_redirects` file:

```
# Subdomain routing - rendered with viewer
https://start.apexsdk.dev/* → https://apexsdk.dev/viewer.html?doc=QUICK_START.md (301)
https://api.apexsdk.dev/* → https://apexsdk.dev/viewer.html?doc=API.md (301)
https://cli.apexsdk.dev/* → https://apexsdk.dev/viewer.html?doc=CLI_GUIDE.md (301)
https://blog.apexsdk.dev/* → https://apexsdk.dev/viewer.html?doc=ROADMAP.md (301)
https://play.apexsdk.dev/* → https://apexsdk.dev/viewer.html (301)
https://docs.apexsdk.dev/* → https://apexsdk.dev/:splat (200)
```

**Note:** All markdown documentation is rendered through the interactive `viewer.html` interface, which provides:
- Syntax-highlighted code blocks
- Responsive navigation sidebar
- Dark/light theme toggle
- Search functionality
- Table of contents
- Clean, modern UI

## Legacy URLs

Old GitHub Pages URLs automatically redirect to the new domain:

```
https://eurybits.github.io/apex-sdk/* → https://apexsdk.dev/:splat (301)
```

## Clean URLs

Shorter aliases for common documentation pages (all rendered with viewer):

| Short URL | Target | Full URL |
|-----------|--------|----------|
| `/quick-start` or `/quickstart` | Quick Start Guide | `viewer.html?doc=QUICK_START.md` |
| `/api` | API Reference | `viewer.html?doc=API.md` |
| `/cli` or `/cli-guide` | CLI Guide | `viewer.html?doc=CLI_GUIDE.md` |
| `/contributing` | Contributing Guide | `viewer.html?doc=CONTRIBUTING.md` |
| `/security` | Security Policy | `viewer.html?doc=SECURITY.md` |
| `/testnets` | Testnet Guide | `viewer.html?doc=TESTNETS.md` |
| `/roadmap` | Development Roadmap | `viewer.html?doc=ROADMAP.md` |
| `/architecture` | System Architecture | `viewer.html?doc=SYSTEM_ARCHITECTURE.md` |

**Example:** `https://apexsdk.dev/quick-start` → Beautifully rendered Quick Start Guide

## Deployment

The site is deployed via **Cloudflare Pages** from the `main` branch:

- **Repository**: [github.com/eurybits/apex-sdk](https://github.com/eurybits/apex-sdk)
- **Build Command**: `chmod +x build.sh && ./build.sh`
- **Output Directory**: `dist/`
- **Pages URL**: [apex-sdk.pages.dev](https://apex-sdk.pages.dev)

## Support

For domain or deployment issues, contact: kherld@duck.com
