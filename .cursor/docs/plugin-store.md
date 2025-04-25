TITLE: Using Stronghold from JavaScript
DESCRIPTION: Complete example of initializing, loading, and using the Stronghold plugin from JavaScript to store and retrieve encrypted data.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/plugin/stronghold.mdx#2025-04-22_snippet_4

LANGUAGE: javascript
CODE:
```
import { Client, Stronghold } from '@tauri-apps/plugin-stronghold';
// when using `"withGlobalTauri": true`, you may use
// const { Client, Stronghold } = window.__TAURI__.stronghold;
import { appDataDir } from '@tauri-apps/api/path';
// when using `"withGlobalTauri": true`, you may use
// const { appDataDir } = window.__TAURI__.path;

const initStronghold = async () => {
	const vaultPath = `${await appDataDir()}/vault.hold`;
	const vaultPassword = 'vault password';
	const stronghold = await Stronghold.load(vaultPath, vaultPassword);

	let client: Client;
	const clientName = 'name your client';
	try {
		client = await stronghold.loadClient(clientName);
	} catch {
		client = await stronghold.createClient(clientName);
	}

	return {
		stronghold,
		client,
	};
};

// Insert a record to the store
async function insertRecord(store: any, key: string, value: string) {
	const data = Array.from(new TextEncoder().encode(value));
	await store.insert(key, data);
}

// Read a record from store
async function getRecord(store: any, key: string): Promise<string> {
	const data = await store.get(key);
	return new TextDecoder().decode(new Uint8Array(data));
}

const { stronghold, client } = await initStronghold();

const store = client.getStore();
const key = 'my_key';

// Insert a record to the store
insertRecord(store, key, 'secret value');

// Read a record from store
const value = await getRecord(store, key);
console.log(value); // 'secret value'

// Save your updates
await stronghold.save();

// Remove a record from store
await store.remove(key);
```

----------------------------------------

TITLE: Installing Stronghold plugin with Cargo
DESCRIPTION: Command to add the Stronghold plugin to your Tauri project's dependencies using Cargo.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/plugin/stronghold.mdx#2025-04-22_snippet_0

LANGUAGE: sh
CODE:
```
cargo add tauri-plugin-stronghold
```

----------------------------------------

TITLE: Initializing Stronghold plugin in Tauri
DESCRIPTION: Basic code to initialize the Stronghold plugin in a Tauri application's lib.rs file.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/plugin/stronghold.mdx#2025-04-22_snippet_1

LANGUAGE: rust
CODE:
```
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
		tauri::Builder::default()
				.plugin(tauri_plugin_stronghold::Builder::new(|password| {}).build())
				.run(tauri::generate_context!())
				.expect("error while running tauri application");
}
```

----------------------------------------

TITLE: Setting Stronghold plugin permissions in capabilities
DESCRIPTION: Example of enabling the Stronghold plugin permissions in a Tauri capabilities configuration file.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/plugin/stronghold.mdx#2025-04-22_snippet_5

LANGUAGE: json
CODE:
```
{
	...,
	"permissions": [
		"stronghold:default",
	]
}
```

----------------------------------------

TITLE: Initializing with argon2 password hash function
DESCRIPTION: Example of initializing the Stronghold plugin with the built-in argon2 password hash function in a Tauri application.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/plugin/stronghold.mdx#2025-04-22_snippet_2

LANGUAGE: rust
CODE:
```
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            app.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

----------------------------------------

TITLE: Initializing with custom password hash function
DESCRIPTION: Example of initializing the Stronghold plugin with a custom password hash function using argon2 in a Tauri application.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/plugin/stronghold.mdx#2025-04-22_snippet_3

LANGUAGE: rust
CODE:
```
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_stronghold::Builder::new(|password| {
                // Hash the password here with e.g. argon2, blake2b or any other secure algorithm
                // Here is an example implementation using the `rust-argon2` crate for hashing the password
                use argon2::{hash_raw, Config, Variant, Version};

                let config = Config {
                    lanes: 4,
                    mem_cost: 10_000,
                    time_cost: 10,
                    variant: Variant::Argon2id,
                    version: Version::Version13,
                    ..Default::default()
                };
                let salt = "your-salt".as_bytes();
                let key = hash_raw(password.as_ref(), salt, &config).expect("failed to hash password");

                key.to_vec()
            })
            .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

----------------------------------------

TITLE: Package Installation Script
DESCRIPTION: Installation script for handling post-install, post-upgrade, and post-remove actions.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/distribute/aur.mdx#2025-04-22_snippet_5

LANGUAGE: ini
CODE:
```
post_install() {
	gtk-update-icon-cache -q -t -f usr/share/icons/hicolor
	update-desktop-database -q
}

post_upgrade() {
	post_install
}

post_remove() {
	gtk-update-icon-cache -q -t -f usr/share/icons/hicolor
	update-desktop-database -q
}
```

----------------------------------------

TITLE: Localizing Info.plist Strings in German
DESCRIPTION: Example of InfoPlist.strings file for German localization of usage descriptions.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/distribute/macos-application-bundle.mdx#2025-04-22_snippet_1

LANGUAGE: ini
CODE:
```
NSCameraUsageDescription = "Kamera Zugriff wird benötigt für WebRTC Funktionalität";
NSMicrophoneUsageDescription = "Mikrofon Zugriff wird benötigt für WebRTC Funktionalität";
```

----------------------------------------

TITLE: Configuring Subcommands for CLI
DESCRIPTION: JSON configuration for subcommands in the CLI plugin, allowing for nested command interfaces similar to git's subcommands.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/plugin/cli.mdx#2025-04-22_snippet_6

LANGUAGE: json
CODE:
```
{
  "cli": {
    ...
    "subcommands": {
      "branch": {
        "args": []
      },
      "push": {
        "args": []
      }
    }
  }
}
```

----------------------------------------

TITLE: Configuring Flag Arguments for CLI
DESCRIPTION: JSON configuration for flag arguments in the CLI plugin, which are standalone keys that can be present or absent.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/plugin/cli.mdx#2025-04-22_snippet_5

LANGUAGE: json
CODE:
```
{
  "args": [
    {
      "name": "verbose",
      "short": "v"
    }
  ]
}
```

----------------------------------------

TITLE: Generating SRCINFO
DESCRIPTION: Command to generate the required .SRCINFO file from PKGBUILD.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/distribute/aur.mdx#2025-04-22_snippet_2

LANGUAGE: sh
CODE:
```
makepkg --printsrcinfo > .SRCINFO
```

----------------------------------------

TITLE: Accessing CLI Arguments in JavaScript
DESCRIPTION: JavaScript code demonstrating how to get and use CLI argument matches from within the Tauri application.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/plugin/cli.mdx#2025-04-22_snippet_7

LANGUAGE: javascript
CODE:
```
import { getMatches } from '@tauri-apps/plugin-cli';
// when using `"withGlobalTauri": true`, you may use
// const { getMatches } = window.__TAURI__.cli;

const matches = await getMatches();
if (matches.subcommand?.name === 'run') {
  // `./your-app run $ARGS` was executed
  const args = matches.subcommand.matches.args;
  if (args.debug?.value === true) {
    // `./your-app run --debug` was executed
  }
  if (args.release?.value === true) {
    // `./your-app run --release` was executed
  }
}
```

----------------------------------------

TITLE: Simplified Core Plugin Permissions Configuration
DESCRIPTION: Shows the new simplified way to enable all default core plugin permissions using the single 'core:default' permission.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/blog/tauri-2-0-0-release-candidate.mdx#2025-04-22_snippet_2

LANGUAGE: json
CODE:
```
{
    "permissions": [
        "core:default"
    ]
}
```

----------------------------------------

TITLE: macOS Entitlements Configuration
DESCRIPTION: Example of Entitlements.plist file configuring app sandbox permission.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/distribute/macos-application-bundle.mdx#2025-04-22_snippet_3

LANGUAGE: xml
CODE:
```
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.app-sandbox</key>
    <true/>
</dict>
</plist>
```

----------------------------------------

TITLE: Configuring Named Arguments for CLI
DESCRIPTION: JSON configuration for named arguments in the CLI plugin, which are key-value pairs with possible values and support for multiple values.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/plugin/cli.mdx#2025-04-22_snippet_4

LANGUAGE: json
CODE:
```
{
  "args": [
    {
      "name": "type",
      "short": "t",
      "takesValue": true,
      "multiple": true,
      "possibleValues": ["foo", "bar"]
    }
  ]
}
```

----------------------------------------

TITLE: SVG Platform Icons Display Layout
DESCRIPTION: HTML/CSS layout structure for displaying platform support icons in a horizontal flex container with consistent spacing.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/blog/tauri-2.0.mdx#2025-04-22_snippet_0

LANGUAGE: html
CODE:
```
<div style="display:flex; flex-direction: row; justify-content: center; align-items: center">
  <div style="padding-inline-end: 10px">
    <svg width="48" height="48">
      <use xlink:href="/assets/platforms.svg#wpf-android-os"></use>
    </svg>
  </div>
  <div style="padding-inline-end: 10px">
    <svg width="48" height="48">
      <use xlink:href="/assets/platforms.svg#simple-icons-ios"></use>
    </svg>
  </div>
</div>
```

----------------------------------------

TITLE: HTML Support Links for 404 Page
DESCRIPTION: HTML paragraph containing support links to GitHub issues and Discord for users experiencing navigation problems.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/404.md#2025-04-22_snippet_1

LANGUAGE: html
CODE:
```
<p>
  If you're having trouble navigating, please <a href="https://github.com/tauri-apps/tauri-docs/issues/new/choose">create an issue on GitHub</a> or <a href="https://discord.com/invite/tauri"
    >report on Discord</a
  >.
</p>
```

----------------------------------------

TITLE: Defining Scope Entry Structure
DESCRIPTION: Rust struct definition for scope data that specifies allowed binaries for a shell plugin.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/develop/Plugins/index.mdx#2025-04-22_snippet_12

LANGUAGE: rust
CODE:
```
#[derive(Debug, schemars::JsonSchema)]
pub struct Entry {
    pub binary: String,
}
```

----------------------------------------

TITLE: Creating New Desktop Entry Point
DESCRIPTION: New main.rs file implementation that calls the shared run function for desktop builds.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/start/migrate/from-tauri-1.mdx#2025-04-22_snippet_2

LANGUAGE: rust
CODE:
```
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  app_lib::run();
}
```

----------------------------------------

TITLE: Debian Package Extraction PKGBUILD
DESCRIPTION: Complete PKGBUILD example for extracting and packaging a Tauri application from a Debian package.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/distribute/aur.mdx#2025-04-22_snippet_4

LANGUAGE: ini
CODE:
```
# Maintainer:
# Contributor:
pkgname=<pkgname>
pkgver=1.0.0
pkgrel=1
pkgdesc="Description of your app"
arch=('x86_64' 'aarch64')
url="https://github.com/<user>/<project>"
license=('MIT')
depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'libsoup' 'pango' 'webkit2gtk-4.1')
options=('!strip' '!debug')
install=${pkgname}.install
source_x86_64=("${url}/releases/download/v${pkgver}/appname_${pkgver}_amd64.deb")
source_aarch64=("${url}/releases/download/v${pkgver}/appname_${pkgver}_arm64.deb")
sha256sums_x86_64=('ca85f11732765bed78f93f55397b4b4cbb76685088553dad612c5062e3ec651f')
sha256sums_aarch64=('ed2dc3169d34d91188fb55d39867713856dd02a2360ffe0661cb2e19bd701c3c')
package() {
	# Extract package data
	tar -xvf data.tar.gz -C "${pkgdir}"

}
```

----------------------------------------

TITLE: Configuring CLI Plugin Permissions
DESCRIPTION: JSON configuration for enabling the CLI plugin in the capabilities configuration, which is required to use the plugin.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/plugin/cli.mdx#2025-04-22_snippet_9

LANGUAGE: json
CODE:
```
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": ["cli:default"]
}
```

----------------------------------------

TITLE: Creating a Basic Node.js Sidecar Script
DESCRIPTION: A simple Node.js script that processes commands from command line arguments and responds through stdout. It handles a 'ping' command and returns an error for unknown commands.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/learn/sidecar-nodejs.mdx#2025-04-22_snippet_0

LANGUAGE: javascript
CODE:
```
const command = process.argv[2];

switch (command) {
  case 'ping':
    const message = process.argv[3];
    console.log(`pong, ${message}`);
    break;
  default:
    console.error(`unknown command ${command}`);
    process.exit(1);
}
```

----------------------------------------

TITLE: Installing create-tauri-app with Alpha Support
DESCRIPTION: Command examples for installing create-tauri-app using different package managers with alpha flag enabled. Shows installation methods via pnpm, yarn, npm, Cargo, Bash, and Powershell.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/blog/create-tauri-app-version-3-released.md#2025-04-22_snippet_0

LANGUAGE: bash
CODE:
```
# pnpm
pnpm create tauri-app --alpha

# yarn
yarn create tauri-app --alpha

# npm
npm create tauri-app -- --alpha

# Cargo
cargo install create-tauri-app --locked
cargo create-tauri-app --alpha

# Bash
sh <(curl https://create.tauri.app/sh) --alpha

# Powershell
$env:CTA_ARGS="--alpha";iwr -useb https://create.tauri.app/ps | iex
```

----------------------------------------

TITLE: Legacy Template Selection Interface
DESCRIPTION: Example of the previous template selection prompt showing all available options before the v3 update.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/blog/create-tauri-app-version-3-released.md#2025-04-22_snippet_2

LANGUAGE: text
CODE:
```
✔ Choose your package manager · pnpm
? Choose your UI template ›
  vanilla
  vanilla-ts
  vue
❯ vue-ts
  svelte
  svelte-ts
  react
  react-ts
  solid
  solid-ts
  next
  next-ts
  preact
  preact-ts
  angular
  clojurescript
  svelte-kit
  svelte-kit-ts
```

----------------------------------------

TITLE: Triggering App Links on Android Emulator
DESCRIPTION: Command to trigger app links on Android emulator using adb CLI.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/plugin/deep-linking.mdx#2025-04-22_snippet_13

LANGUAGE: sh
CODE:
```
adb shell am start -a android.intent.action.VIEW -d https://<host>/path <bundle-identifier>
```

----------------------------------------

TITLE: Updated Core Plugin Permissions with New Namespace
DESCRIPTION: Demonstrates the new RC version format for core plugin permissions using the 'core:' namespace prefix in the capabilities configuration.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/blog/tauri-2-0-0-release-candidate.mdx#2025-04-22_snippet_1

LANGUAGE: json
CODE:
```
{
    "permissions": [
        "core:path:default",
        "core:event:default",
        "core:window:default",
        "core:app:default",
        "core:image:default",
        "core:resources:default",
        "core:menu:default",
        "core:tray:default"
    ]
}
```

----------------------------------------

TITLE: Accessing CLI Arguments in Rust
DESCRIPTION: Rust code showing how to use the CLI plugin to access and process command line arguments within the Tauri application setup.
SOURCE: https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/plugin/cli.mdx#2025-04-22_snippet_8

LANGUAGE: rust
CODE:
```
use tauri_plugin_cli::CliExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
   tauri::Builder::default()
       .plugin(tauri_plugin_cli::init())
       .setup(|app| {
           match app.cli().matches() {
               // `matches` here is a Struct with { args, subcommand }.
               // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
               // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
               Ok(matches) => {
                   println!("{:?}", matches)
               }
               Err(_) => {}
           }
           Ok(())
       })
       .run(tauri::generate_context!())
       .expect("error while running tauri application");
}
```