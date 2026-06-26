#!/usr/bin/env node
const { platform, arch } = process;
const fs = require('fs');
const path = require('path');
const os = require('os');

const VERSION = '0.4.1';
const REPO = 'billlzzz26/blink-md';

function getTarget() {
  const plat = platform;
  const archMap = { x64: 'amd64' };
  
  if (plat === 'linux') return `x86_64-unknown-linux-gnu`;
  if (plat === 'darwin') return archMap[arch] === 'amd64' ? 'x86_64-apple-darwin' : 'aarch64-apple-darwin';
  if (plat === 'win32') return 'x86_64-pc-windows-msvc';
  
  throw new Error(`Unsupported platform: ${plat}`);
}

function getBinaryName() {
  return platform() === 'win32' ? 'blink-md.exe' : 'blink-md';
}

async function install() {
  const target = getTarget();
  const binary = getBinaryName();
  const url = `https://github.com/${REPO}/releases/download/v${VERSION}/blink-md-${target}.tar.gz`;
  
  console.log(`Installing blink-md v${VERSION} for ${target}`);
  
  const { execSync } = require('child_process');
  const dest = path.join(__dirname, '..', binary);
  
  try {
    // Download and extract
    execSync(`curl -sL ${url} | tar xzO ${binary} > ${dest}`, { stdio: 'inherit' });
    execSync(`chmod +x ${dest}`, { stdio: 'inherit' });
    console.log(`Installed to ${dest}`);
  } catch (err) {
    console.error('Install failed. Please install Rust and run: cargo install blink-md');
    process.exit(1);
  }
}

install();