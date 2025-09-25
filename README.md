# MCX
A new and simple way to manage [Minecraft](https://www.minecraft.net/) servers!

# Features
- Create a Vanilla, Fabric, Forge, or Neoforge server
- Run any server
- Download mods
- Server console

# Planned
- Quilt servers
- Paper, Spigot, and other plugin software
- Better mod management
- World management
- Changing server options
- Mod verification

# Commands
`mcx` - Entry point for all other commands.
- `init` - Initialize a new Minecraft server
- `pack [ID]` - Initialize a new server from a Modrinth modpack id
- `run` - Run the Minecraft server
- `mod` - Manage mods for your Minecraft server
  - `add [ID]` - Add a mod from Modrinth by mod id
  - `list` - List downloaded mods
  - `update` - Update all mods added from Modrinth
  - `remove [ID]` - Remove a mod added from Modrinth

# Installation
## Installing from GitHub
Open the releases page for this repo and download the latest version for your operating system.  
## Crates.io
Run the following command to install the binary directly from [crates.io](https://crates.io/):
```sh
cargo install mcx
```
Or if you have `binstall` (actually idk if this works):
```sh
cargo binstall mcx
```
