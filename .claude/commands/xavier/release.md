---
name: Xavier: Release
description: Build the site, commit changes, and push to remote
category: Xavier
tags: [release, build, deploy]
---

**Steps**

Execute the following steps in order:

1. **Build the site**
   Run `make build` to generate the static site to `docs/` directory.

2. **Create a commit**
   Use the `/commit` command (commit-commands:commit) to create a git commit with all changes.
   - Stage the relevant files (docs/, content/, etc.)
   - Write a clear commit message describing the changes

3. **Push to remote**
   Run `git push` to push the commit to the remote repository.

**Notes**
- If the build fails, stop and report the error
- If there are no changes to commit, skip the commit step
- The site will be automatically deployed to GitHub Pages after push
