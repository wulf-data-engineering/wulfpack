---
trigger: always_on
---

Never make changes in this repository on the `main` branch.

**CRITICAL**: Before performing any edit to any file in this repository, you **MUST** first check the current branch:
```bash
git branch --show-current
```

If the output is `main`, you are **FORBIDDEN** from editing any file. You **MUST** stop and ask the user to switch to a feature branch.

**CRITICAL**: If this repository is on a branch that is not yours, it might be used by another agent!

Then check out a branch. The name of a test instance is a good branch name.
