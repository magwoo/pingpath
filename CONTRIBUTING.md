# Contributing Guidelines

Thank you for your interest in contributing to this project! To ensure a smooth and efficient workflow, please follow these guidelines and we won't mind if you help improve this guide.

---

## 📌 1. How to Take an Issue

### ✅ If You Have Write Access to the Repository
1. Find an open issue in the **Issues** tab.
2. Leave a comment: _"I'll take this!"_
3. A maintainer will assign you to the issue (or you can self-assign if possible).
4. Create a feature branch (`42-api-improve`) and start working.
5. Once done, submit a **Pull Request (PR)** and link it to the issue (`Closes #42`).
6. After the review, the PR will be merged, and the issue will be closed automatically.

### 🔄 If You Don’t Have Write Access (Working via Fork)
1. **Fork the repository** by clicking the "Fork" button on GitHub.
2. **Clone your fork** locally:
   ```sh
   git clone https://github.com/your-username/Pingpath.git
   cd project
   ```
3. **Add the original repository as an upstream remote**:
   ```sh
   git remote add upstream https://github.com/openRings/Pingpath.git
   git fetch upstream
   ```
4. **Create a new branch**:
   ```sh
   git checkout -b 1-my-awesome-feature
   ```
5. **Make your changes, commit, and push**:
   ```sh
   git add .
   git commit -m "Added a cool feature"
   git push origin 1-my-awesome-feature
   ```
6. **Create a Pull Request (PR)** to the original repository.
7. Before making another contribution, **sync your fork**:
   ```sh
   git checkout main
   git fetch upstream
   git merge upstream/main
   git push origin main
   ```

---

## 🔧 2. Branch Naming Conventions
- Use descriptive branch names based on issue numbers:
  - Features: `{issue-number}-{short-description}`  
    _Example: `42-api-improve`_
  - Bug fixes: `{issue-number}-{short-description}`  
    _Example: `88-login-bug`_
- If multiple developers work on the same issue:
  - One creates the main feature branch (`42-main`).
  - Others create sub-branches (`42-helper`) and merge into the main feature branch before the final PR.

---

## 🚀 3. Collaboration on Large Features
- If multiple developers work on the same feature:
  - **Divide tasks** (e.g., one works on backend, another on UI).
  - **Use a shared feature branch** (`42-main`) with sub-branches.
  - **Sync regularly** (`git pull --rebase` before pushing).
  - **Discuss progress in the Issue comments.**

---

## 🛠 4. Project Board and Issue Tracking
- All issues and features are tracked on the **Project Board**.
- Move issues to **"In Progress"** when you start working.
- Move issues to **"Review"** when a PR is created.
- Issues are closed automatically when the corresponding PR is merged.

---

## 💡 Final Notes
- Keep your commits **clean and meaningful**.
- Follow the **branching and PR guidelines**.
- Be **respectful and collaborative** in discussions.
- Happy coding! 🚀

