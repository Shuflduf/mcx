# MCX
A new and simple way to manage [Minecraft](https://www.minecraft.net/) servers!

# Features
- Create a Vanilla server
- Create a NeoForge server
- Run any server
- Download mods
- Server console

# Planned
- Fabric, Forge, and Quilt servers
- Paper, Spigot, and other plugin software
- Add mods with name
- Better mod management
- World management
- Changing server options
- Mod verification

# Commands
`mcx` - Entry point for all other commands.
- `init` - Create a Minecraft server. You can choose the loader as well as the version
- `mod` - Shows the help menu for mods  
  - `add [URL]` - Adds a mod from the URL. Needs a direct download link
  - `add [MOD_ID]` - Adds a mod from the mod ID. Find the mod ID in the URL on the Modrinth website.
  For example, the mod ID for [NeoTech](https://modrinth.com/mod/neotech) is `neotech`
  - `list` - Lists all installed mods on the server

# Installation
## Installing from GitHub
Open the releases page for this repo and download the latest version for your operating system.  
## Crates.io
Run the following command to install the binary directly from [crates.io](https://crates.io/):
```sh
cargo install mcx
```
Or,
