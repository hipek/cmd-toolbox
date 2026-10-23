# Video Project Creator Specification

## Overview
The Video Project Creator is a utility designed to standardize the setup of new video editing projects. It ensures that every project follows a consistent file structure, reducing manual effort and keeping assets organized from the beginning.

## Help and Usage
If the user needs guidance on how to use the tool, they can run the command with the `--help` flag (or `-h`). This will display usage instructions, including the expected format for the project name and a description of the default folder structure that will be created.

## Workflow
1.  **Project Naming**: The user provides a name for the new project.
2.  **Standardization**: The system automatically adjusts the name to ensure it is compatible with file system conventions.
3.  **Organization**: The system generates a new, unique project folder located within a centralized video directory. The folder is automatically tagged with the current date (year and month).
4.  **Structure**: Inside the new project folder, the system automatically creates a set of predefined subfolders for different types of project files:
    *   **Project files**: For storing the primary project files.
    *   **Source files**: For storing raw footage.
    *   **Assets**: For storing supporting assets.
    *   **Music and Sound Effects**: For organizing audio resources.
    *   **Export**: For organizing final output files.
5.  **Conflict Prevention**: If a project with the same name already exists for the current month, the system prevents creation to avoid overwriting or duplicating work.
