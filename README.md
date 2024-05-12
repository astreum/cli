# Astreum CLI

The Astreum CLI is a command-line interface for interacting with the Astreum blockchain. It provides a variety of commands to manage your account, send aster, chain synchronization, serve http api and access a Lispeum interface.

## Installation

This guide will help you set up the Astreum CLI on your Windows system.

### Create Working Folder

1. Open your Command Prompt.
2. Create a new directory where you would like to install the Astreum CLI:
   ```
   mkdir Astreum
   cd Astreum
   ```

### Download

Download the latest version of the Astreum CLI from Google Drive:

- [Download Astreum CLI](https://drive.google.com/file/d/1cORBmPOyrW-tBGuFWyWttz7GZquvPvYK/view?usp=sharing)

After downloading, you should have the `astreum.exe` file in your Downloads folder or the location you chose to save it.

### Installation Steps

1. Move the CLI binary to your working folder:
   ```
   move path\to\your\downloads\astreum.exe .
   ```

### Optional: Add to Environment

To run the Astreum CLI from any command prompt or terminal window, you can add it to your system's PATH.

1. Right-click on 'This PC' or 'Computer' on your desktop or in File Explorer.
2. Click 'Properties'.
3. Click 'Advanced system settings'.
4. Click 'Environment Variables'.
5. In the 'System variables' section, scroll down and select the 'Path' variable, then click 'Edit'.
6. Click 'New' and add the path to the folder where `astreum.exe` is located.
7. Click 'OK' to close all dialogs.

## Usage

The general syntax for using the Astreum CLI is:

    astreum <command>

### Commands

Astreum CLI supports the following commands:

- account
- code
- api
- sync
- send [amount] to [address] `coming soon`

© 2024 Astreum Foundation
