/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    appDir: true,
  },
  transpilePackages: ["@weatherah/shared-types"],
};

module.exports = nextConfig;
