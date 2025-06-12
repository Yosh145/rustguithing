URL Tracker GUI
This project is a desktop application developed in Rust using the egui and eframe libraries. It provides a user interface based on a specified design, featuring a tabbed layout for a main "URL Tracker" screen and a "Settings" page.

Screenshot
Here is a preview of the application's user interface

Features
Cross-Platform UI: Built with egui to run on Windows, macOS, and Linux.
Tabbed Interface: Cleanly separates the main tracker functionality from the application settings.
Dynamic Layouts: Utilizes egui's layout system for responsive and organized UI elements.
Functional Control Widgets: Includes interactive elements like buttons, radio buttons, and dropdown menus to manage application state.
Configurable Settings: A dedicated settings page with functional examples:
Live theme switching (Dark/Light mode).
A slider for numerical input.
A masked text field for sensitive data like API keys.
Getting Started
Follow these instructions to get a copy of the project up and running on your local machine.

Prerequisites
You need to have the Rust programming language and its package manager, Cargo, installed on your system. You can install them by following the official instructions at rustup.rs.

Running the Application
Clone the repository (or navigate to the project directory if you already have the files):

Bash

git clone <your-repository-url>
cd <repository-name>
Build and run the project using Cargo:

Bash

cargo run
This command will compile all the necessary dependencies and launch the application. For a production build, you can run cargo run --release for better performance.

Project Structure
The source code is organized into a few key files:

Cargo.toml: Defines the project metadata and lists its dependencies, primarily eframe and egui.
src/main.rs: The main entry point for the application. It sets up the native window and initializes the eframe application loop.
src/app.rs: The core of the application. It contains the struct that holds the application's state and the impl eframe::App block where the entire UI is defined and updated every frame.
Dependencies
eframe: The framework used to create a native application window and run an egui app.
egui: An easy-to-use, immediate-mode GUI library for Rust.
License
This project is licensed under the MIT License - see the LICENSE.md file for details.
