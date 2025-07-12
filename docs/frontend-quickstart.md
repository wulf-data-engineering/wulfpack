# Frontend Quickstart

This document provides a quickstart guide for developing frontend applications in the Tool-Set monorepo.

## Getting Started

1.  **Install Node.js and npm:** If you don't have Node.js and npm installed, follow the instructions on the [official Node.js website](https://nodejs.org/en/download/).

2.  **Scaffold a new project:** Use the `cookiecutter` command to scaffold a new project from the root template. This will create a new directory with the project name you provide.

    ```bash
    cookiecutter /path/to/wulfpack
    ```

3.  **Navigate to the frontend directory:**

    ```bash
    cd my-new-app/frontend
    ```

4.  **Run the development server:**

    ```bash
    npm run dev
    ```

## Available Scripts

The generated frontend project in `my-new-app/frontend/` contains its own [`README.md`](../template/%7B%7Bcookiecutter.project_slug%7D%7D/frontend/README.md) file.

Please refer to that file for a detailed list of all available npm scripts and their descriptions, which you can use to build, test, and lint the frontend application.
