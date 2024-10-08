#!/bin/bash

LATEST_TAG=$(curl "https://api.github.com/repos/Luganodes/hypermon/tags" | jq -r '.[0].name')
DESTINATION_DIR="/usr/local/bin"
DOWNLOAD_LINK="https://github.com/Luganodes/hypermon/releases/download/$LATEST_TAG/hypermon"
BINARY_NAME="hypermon"

# Function to check OS type
check_os_type() {
  case "$(uname -s)" in
    Linux*)     OS="Linux" ;;
    Darwin*)    OS="MacOS" ;;
    *)          echo "Unsupported OS"; exit 1 ;;
  esac
}

# Function to install the binary
install_binary() {
  Check if the user is root or has sudo privileges
  if [ "$EUID" -ne 0 ]; then
    echo "Please run this script with sudo or as root."
    exit 1
  fi

  # Check if the destination directory exists
  if [ ! -d "$DESTINATION_DIR" ]; then
    echo "Creating $DESTINATION_DIR..."
    mkdir -p "$DESTINATION_DIR"
  fi

  # Copy the binary to the destination directory
  echo "Installing $BINARY_NAME to $DESTINATION_DIR..."
  wget $DOWNLOAD_LINK -O "$DESTINATION_DIR/$BINARY_NAME"
  chmod +x "$DESTINATION_DIR/$BINARY_NAME"

  # Verify installation
  if [ -f "$DESTINATION_DIR/$BINARY_NAME" ]; then
    echo "$BINARY_NAME successfully installed in $DESTINATION_DIR"
  else
    echo "Installation failed."
    exit 1
  fi
}

# Check the OS type
check_os_type

# Install the binary
install_binary

# Success message
echo "Installation complete."
