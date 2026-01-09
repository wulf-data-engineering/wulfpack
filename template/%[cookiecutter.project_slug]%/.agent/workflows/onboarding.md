---
description: Onboarding guide for new developers (requirements, MCP server setup)
---

# Onboarding

Start implementation plan with greeting to developer that you will guide through setup.

# Requirements

Run command to check for missing requirements:

// turbo-all

```bash
echo "git" && \
git --version && \
echo "cargo" && \
cargo --version && \
echo "cargo lambda" && \
cargo lambda --version && \
echo "npm" && \
npm --version && \
echo "docker" && \
docker --version && \
echo "docker compose" && \
docker compose version && \
echo "aws" && \
aws --version &&  \
echo "done"
```

Add tasks to install missing dependencies to implementation plan.

# MCP servers

Explain suggested MCP servers in implementation plan and final output.

```json
{
  "mcpServers": {
    "svelte": {
      "command": "npx",
      "args": ["-y", "@sveltejs/mcp"]
    },
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "<GITHUB_PERSONAL_ACCESS_TOKEN>"
      }
    }
  }
}
```

For each missing credentials/token explain separately where and how to obtain.

Explain developer has to merge json manually with ~/.gemini/antigravity/mcp_config.json for security reasons.

# Docker Compose Alias

Add following information exactly to implementation plan and final output:

The project relies on `docker compose` (v2), but AI assistants often try to use the legacy `docker-compose` command. To prevent friction and errors, please add an alias to your shell configuration (`~/.zshrc`, `~/.bashrc`, etc.):

```bash
alias docker-compose="docker compose"
```