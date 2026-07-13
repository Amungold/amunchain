# ADR-0003

# Adopt Vite as the Build System

Status: Accepted

Date: 2026-07-01

---

# Context

The Amun Design Platform requires a build system
that is:

- Fast
- Simple
- Standards based
- TypeScript friendly
- ES Module native
- Suitable for Monorepos

The build system is a development tool only.

Production deployments SHALL consist of
static assets.

---

# Decision

The platform SHALL use Vite
as the official build system.

Vite SHALL be used for:

- Development Server
- TypeScript Compilation
- Production Bundling
- Asset Optimization
- Code Splitting

Runtime dependency on Vite
is prohibited.

---

# Rationale

Vite provides:

- Extremely fast startup
- Native ES Module development
- Excellent TypeScript support
- Simple configuration
- Efficient production builds
- Automatic code splitting

These characteristics align with
the goals of the platform.

---

# Alternatives Considered

Webpack

Rejected because:

- Complex configuration
- Slower builds
- Higher maintenance cost

---

Rollup

Rejected because:

- Better suited as a library bundler
- Less convenient developer experience

---

esbuild

Rejected because:

- Excellent speed
- Limited ecosystem compared with Vite
- Less mature application tooling

---

Parcel

Rejected because:

- Less predictable configuration
- Smaller ecosystem for enterprise workflows

---

# Consequences

Positive

- Faster development
- Faster builds
- Better developer experience
- Native ES Module workflow
- Excellent TypeScript integration

Negative

- Requires Node.js
- Toolchain dependency during development

These drawbacks are acceptable.

---

# Production Output

Production builds SHALL generate
static assets only.

Typical output includes:

- HTML
- CSS
- JavaScript
- Fonts
- Icons
- Images

Applications SHALL be deployable
using any static web server.

---

# Build Principles

The build process SHALL support:

- Tree Shaking
- Code Splitting
- Lazy Loading
- Asset Compression
- Source Maps
- Content Hashing

Development-only code SHALL NOT
appear in production builds.

---

# Compatibility

The generated output SHALL be
compatible with modern evergreen browsers.

Legacy browser support MAY be added
through optional plugins if required.

---

# Compliance

All official packages and applications
SHALL use the shared Vite configuration
defined by the platform.

Package-specific configuration SHALL be
kept to a minimum.

---

# References

CONSTITUTION.md

ARCHITECTURE.md

VERSIONING.md

