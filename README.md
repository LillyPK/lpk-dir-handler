# lpk - LillyPK Directory Handler

`lpk` is a simple and intelligent command-line tool that enhances basic directory navigation on Linux systems. It provides a safer and more convenient replacement for `cd` with useful features such as auto-directory creation and drive listing.

---

## What It Does

`lpk` replaces the traditional `cd` command with more interactive behavior:

* `lpk /some/path`:
  Changes to the specified directory.

  * If the directory exists, it prints a `cd` command for shell scripts to use.
  * If the directory **does not exist**, it prompts:
    `ERROR: directory missing. Would you like to create this directory?`

    * Press **Enter** to create and enter it
    * Press **Backspace** or **Ctrl+C** to cancel

* `lpk -ls`:
  Lists all mounted drives on the system, showing their mount points, names, and sizes. This is useful for quickly identifying attached storage devices.

* `lpk -help`:
  Displays a list of available commands and usage instructions.

* `lpk --uninstall`:
  Removes the `lpk` binary from your system (`/usr/local/bin/lpk`).

---

## Why You Should Use It

* Prevents `cd` errors by prompting before creating new directories
* Helps you quickly identify available drives using `-ls`
* User-friendly and interactive terminal prompts
* Simple to install and lightweight
* Safer directory handling for shell scripts or regular terminal usage

---

## How to Install

Run the following command in your terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/LillyPK/lpk-dir-handler/main/lpk/install.sh | bash
```

This will:

1. Download the latest release binary from GitHub
2. Move it to `/usr/local/bin/lpk`
3. Make it globally available as the `lpk` command

No dependencies are required beyond standard system tools (`curl`, `bash`, `sudo`).

---

## Getting Help

To view available commands and options:

```bash
lpk -help
```
