# rpc_benchmark_rust

A simple Rust program to interact with the Solana blockchain using the Solana RPC API and benchmark it.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Requirements](#requirements)
- [Usage](#usage)
- [Configuration](#configuration)
- [License](#license)

## Overview

This Rust program demonstrates basic interactions with the Solana blockchain through the Solana RPC API. It performs multiple RPC calls such as `get_block_height`, `get_account`, `get_cluster_nodes`, and `get_multiple_accounts` with random account keys.

## Features

- Sends multiple Solana RPC requests with random account keys.
- Measures and displays the duration of each RPC request.
- Calculates and prints the average duration of all requests.

## Requirements

- Rust: Ensure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).

## Usage

1. Clone the repository:

   ```bash
   git clone git@github.com:JawadHussain05/rpc_benchmark_rust.git
