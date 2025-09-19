# Weatherah

Weatherah is a comprehensive weather application built using a modern tech stack in a Turbo monorepo structure. The project combines the power of Next.js for frontend and Rust for backend services to deliver a fast, reliable, and scalable weather experience.

## Project Overview

This monorepo contains multiple applications and shared packages that work together to provide a complete weather solution:

### Apps

- `web`: The main Next.js application that users interact with
- `docs`: A Next.js application for documentation
- `api`: A high-performance Rust API that handles weather data and business logic

### Packages

- `@weatherah/ui`: A React component library shared across frontend applications
- `@weatherah/eslint-config`: Shared ESLint configurations
- `@weatherah/typescript-config`: Shared TypeScript configurations
- `@weatherah/shared-types`: Common TypeScript types used across the project

## Tech Stack

### Frontend

- **Framework**: Next.js 15.5+
- **UI Library**: React 19
- **Styling**: TailwindCSS 4.1+
- **Language**: TypeScript 5.9

### Backend

- **Language**: Rust (2021 edition)
- **Web Framework**: Axum 0.7
- **Database**: PostgreSQL with SQLx ORM
- **Runtime**: Tokio
- **API Communication**: RESTful + WebSockets

### Development & Infrastructure

- **Monorepo Management**: Turborepo
- **Package Manager**: pnpm 9.0+
- **Code Quality**: ESLint, Prettier
- **Node Version**: 18+

## Getting Started

### Prerequisites

- Node.js 18 or later
- pnpm 9.0+
- Rust toolchain (for API development)
- PostgreSQL database

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/weatherah.git
cd weatherah

# Install dependencies
pnpm install

# Set up environment variables
# Copy .env.example files in each app directory to .env and update the values
```

### Development

To develop all apps and packages simultaneously:

```bash
pnpm dev
```

To develop a specific app or package:

```bash
# Example: develop only the web app
pnpm dev --filter=weatherah-web
```

### Building

To build all apps and packages:

```bash
pnpm build
```

To build a specific app or package:

```bash
# Example: build only the web app
pnpm build --filter=weatherah-web
```

## Project Structure

```
weatherah/
├── apps/
│   ├── api/           # Rust-based backend API
│   ├── docs/          # Documentation site
│   └── web/           # Main web application
├── packages/
│   ├── eslint-config/ # Shared ESLint configurations
│   ├── shared-types/  # Common TypeScript types
│   ├── typescript-config/ # Shared TypeScript configurations
│   └── ui/            # Shared UI components
```

## Turborepo Features

This project leverages several powerful Turborepo features:

### Remote Caching

Turborepo can use [Remote Caching](https://turborepo.com/docs/core-concepts/remote-caching) to share cache artifacts across machines, enabling you to share build caches with your team and CI/CD pipelines.

To enable Remote Caching:

```bash
pnpm exec turbo login
pnpm exec turbo link
```

### Task Filtering

Run specific tasks for specific packages with filtering:

```bash
# Example: lint only the web app
pnpm lint --filter=weatherah-web
```

## Useful Links

Learn more about the technologies used in this project:

- [Turborepo Documentation](https://turborepo.com/docs)
- [Next.js Documentation](https://nextjs.org/docs)
- [Rust Programming Language](https://www.rust-lang.org/learn)
- [Axum Web Framework](https://github.com/tokio-rs/axum)
- [pnpm Documentation](https://pnpm.io/motivation)
- [TailwindCSS Documentation](https://tailwindcss.com/docs)
