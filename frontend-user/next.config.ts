import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  /* config options here */
  reactStrictMode: true,
  allowedDevOrigins: [process.env.NEXT_PUBLIC_ALLOWED_DEV_ORIGINS || ''],
};

export default nextConfig;
