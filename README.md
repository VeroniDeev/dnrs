# dnrs
#### Light dns server written in rust
**Dnrs** is a dns server written in rust that aims to modernize the approach to writing configuration files and to be lightweight.

**Dnrs** is a mix between dns and rs from rust.

## Badges
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/VeroniDeev/dnrs/blob/main/LICENSE)

# Table of contents

1. [Requirement](#requirement)
2. [Installation](#installation)
3. [Setup](#setup)
4. [Documentation](#documentation)
5. [Feedback](#feedback)

# Requirement

For **dnrs** to work perfectly, you just need to install rust and have a Linux system. 

# Installation

### For the moment, dnrs installation is only available on Arch Linux with the AUR package. Here's how to proceed:

Updating the aur repository
```sh
yay -Syy
```

Install **dnrs**
```sh
yay -S dnrs
```

### For all linux systems other than Arch, here's how to configure it easily

Clone the repository
```sh
git clone https://github.com/VeroniDeev/dnrs
```

Go to the folder
```sh
cd dnrs
```

Set build.sh as executable then run 
```sh
chmod +x config.sh && ./build.sh
```

# Setup

### Now before running dnrs you need to configure the systemd

Enable dnrs.service
```sh
sudo systemctl enable dnrs.service
```

Start dnrs.service
```sh
sudo systemctl start dnrs.service
```

Check the status
```sh
 sudo systemctl status dnrs.service
 ```
If you see an error, don't hesitate to open an [issue](https://github.com/VeroniDeev/dnrs/issues)

# Documentation
the documentation is currently being written!

# Feedback
If you have any comments, please do not hesitate to contact me at zoubheir@gmail.com.