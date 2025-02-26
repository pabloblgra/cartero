<p align="center"><img src="data/icons/scalable/apps/es.danirod.Cartero.svg" width="256" height="256" alt=""></p>

<h1 align="center">Cartero</h1>
<p align="center">Make HTTP requests and test APIs</p>

<p align="center">
<img src="data/screenshots/cartero-default.png" alt="Screenshot of Cartero">
</p>

Cartero is a graphical HTTP client that can be used as a developer tool to
test web APIs and perform all kind of HTTP requests to web servers. It is
compatible with any REST, SOAP or XML-RPC API and it supports multiple request
methods as well as attaching body payloads to compatible requests.

Features:

- Loads and saves to plain Git-friendly TOML files, so that you can own your data.
- Customization and modification of the request headers and body payloads.
- Variable binding for API keys and other secret information.

## Motivation

This project exists because there aren't many native graphical HTTP testing
applications / graphical alternatives to cURL that are fully free software, and
I think the world has had enough of Electron / non-native applications that are
anonymously accesible until one day you are forced to create an account and
log in to use just to make some investor happy with their numbers or to chug
some unwanted artificial intelligence at users.

## Download

<a href="https://flathub.org/apps/es.danirod.Cartero">
<img width="240" alt="Get it on Flathub" src="https://flathub.org/api/badge?svg&locale=en">
</a>

You can also download a version for Windows or macOS.

> [!WARNING]
>
> **The Windows version is still not signed.** On first run on Windows, you'll need to confirm the SmartScreen warning. (I promise I'm purchasing a certificate after Christmas.)

|                               | Platform              | Latest version                       |
| ----------------------------- | --------------------- | ------------------------------------ |
| ![Windows](doc/windows.png)   | Windows, x86_64       | [v0.1.3 (installer)][windows-x86_64] |
| ![macOS](doc/macos.png)       | macOS, Apple Sillicon | [v0.1.3 (.dmg)][macos-sillicon]      |
| ![macOS](doc/macos.png)       | macOS, Intel          | [v0.1.3 (.dmg)][macos-intel]         |
| ![AppImage](doc/appimage.png) | AppImage, x86_64      | [v0.1.3 (AppImage)][appimage-x86_64] |

## Building

### Flatpak

Install the runtime:

```sh
flatpak install --user org.gnome.Sdk//46 org.freedesktop.Sdk.Extension.rust-stable//23.08
```

Install the Flatpak, possibly passing the `--user` parameter.

```sh
flatpak-builder --user --install flatpak_app build-aux/es.danirod.Cartero.json
```

You will find Cartero in your application launcher, or you can launch it with
`flatpak run es.danirod.Cartero`.

### Homebrew

To install Cartero from this cask, simply add this repository as a tap.

```sh
brew tap SoloAntonio/cartero
```

Now you can install any version hosted as cask with

```sh
brew install --cask cartero
```

### Meson

Use this to build the application outside of a Flatpak image. Note that this method should
only be used by power users, to package the application on build scripts for rolling-release
distributions, or for development purposes.

Currently, to build the application you'll have to make sure that the required
libraries are installed on your system.

- glib >= 2.72
- gtk >= 4.14
- gtksourceview >= 5.4
- libadwaita >= 1.5

Make sure that you have Meson in your system. For instance,

```sh
sudo apt install meson
sudo dnf install meson
sudo pacman -S meson
```

Then use the following commands to build and install the application

```sh
meson setup build
ninja -C build
ninja -C build install
```

To avoid installing system-wide the application, you can use a prefix:

```sh
meson setup build --prefix=/usr
ninja -C build
ninja -C build install
```

**If you plan on contributing to the project**, use the development profile.

```sh
meson setup build -Dprofile=development
```

It will also configure a Git hook so that the source code is checked prior to
authoring a Git commit. The hook runs `cargo fmt` to assert that the code is
formatted. Read `hooks/pre-commit.hook` to inspect what the script does.

### Cargo (Rust)

This may be the preferred approach if you use rust-analyzer and tools like
that, because Meson uses a separate target directory, and you might prefer to
use the standard target/ directory.

You can use `cargo build` and `cargo run` to build and run the project
executable, but you will still need to use Meson to build the data files
(GSettings schema files, Gio resource files, gettext translations...).

**You should use `build-aux/cargo-build.sh`**. It wraps `cargo build`, but also
compiles the data files and copies them in `target/share`, so that during
application startup those files can be picked.

### Nix/NixOS

Use this approach to install, build or try cartero on a nixos system. Instructions
assume you're using a flakes nixos system, but you could install it in a regular
nixos system aswell by importing the derivation and adding the appropiate src attribute
on it, note that this may require some manual intervation though.

First of all, add cartero to your flake inputs so you can import the package.

```nix
{
  inputs = {
    cartero.url = "github:danirod/cartero";
  };
}
```

> [!WARNING]
> This examples assume you're passing `inputs` in the `specialArgs` so you can utilize it
> in others modules if you're splitting your config in multiple files.

Then in your `home.packages` (when using home manager) or `environment.systemPackages`
(global nix packages), add the derivation.

```nix
environment.systemPackages = [
  inputs.cartero.packages.x86_64-linux.default
];
```

> [!TIP]
> You can try changing the architecture, not tested in every arch atm though.

Another way is by making a nixpkgs overlay to add cartero and then install it
easily.

```nix
nixpkgs.overlays = [
  (_: final: let
    inherit (inputs) cartero;
    inherit (final) system;
  in {
    cartero = cartero.packages.${system}.default
  })
];
```

And then in the packages list of your choice.

```nix
home.packages = with pkgs; [
  cartero
];
```

> [!NOTE]
> You may need to reboot the system or relogin to be able to see cartero on your launcher

## Contributing

> 🐛 This project is currently a larva trying to grow. Do you want to get in?
> Take a seat!

### Contributing with code

This project is highly appreciative of contributions. If you know about Rust,
GTK or the GNOME technologies and want to help during the development, you can
contribute if you wish. [Fork the project][fork] and commit your code.

Some checklist rules before submitting a pull request:

- **Use a feature branch**, do not make your changes in the trunk branch
  directly.

- **Rebase your code** and make sure that you are working on top of the most
  recent version of the trunk branch, in case something has changed while you
  were working on your code.

- **Update the locales** if you changed strings. The ninja target that you are
  looking for is called `cartero-update-po` (such as `ninja -C build
cartero-update-po`). Don't worry, you don't have to translate the strings by
  yourself, but make sure that the new templates are added to the .po and .pot
  files.

- **Use the pre-commit hook**. The pre-commit hook will validate that your code
  is formatted. It should be automatically configured if you run Meson in
  development mode (`-Dprofile=development`), but you can install it on your
  own or run `hooks/pre-commit.hook`.

The project is starting small, so if you want to do something big, it is best
to first start a discussion thread with your proposal in order to see how to
make it fit inside the application.

While this application is not official and at the moment is not affiliated with
GNOME, you are expected to follow the [GNOME Code of Conduct][coc] when
interacting with this repository.

### Contributing with translations

Do you want to use Cartero in your language? We are using [Weblate][weblate]
to coordinate and translate comfortably this project using a web interface.
Make an account and start proposing strings and they will be added to the
application. That will also entitle you as a contributor!

### Contributing with feedback

Cartero is still getting new features, and hopes to be as useful as it can be.
Found a bug or something is wrong? Report it. An use case you are missing?
Report it. Show us how you integrate Cartero on your workflow so that we can
build our diverse list of use cases.

## Licenses

Cartero is published under the terms of the GNU General Public License v3.0 or later.

```
Copyright 2024 the Cartero authors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

The Cartero icon is published under the a [Creative Commons
Attribution-ShareAlike 4.0 International license][ccbysa].

## Credits and acknowledgments

Cartero is maintained by [Dani Rodríguez][danirod].

Big shoutout to the [contributors][contrib] who have sent patches or
translations!

Also, Christian suggested Cartero as the name for the application and I liked
it enough to call it like so, therefore shoutout to Christian as well!

Finally, shoutout to many of the GTK and GNOME Circle applications out there whose
source code I've read in order to know how to use some of the GTK features that
you cannot learn just by reading the official docs.

## Blog

Dani's [dev blog][blog] (in Spanish) of Cartero.

[ccbysa]: https://creativecommons.org/licenses/by-sa/4.0/
[coc]: https://conduct.gnome.org
[contrib]: https://github.com/danirod/cartero/graphs/contributors
[danirod]: https://github.com/danirod
[fork]: https://github.com/danirod/cartero/fork
[blog]: https://danirod.es/secciones/devlogs/cartero/
[weblate]: https://hosted.weblate.org/projects/cartero/
[windows-x86_64]: https://github.com/danirod/cartero/releases/download/v0.1.3/Cartero-0.1.3-windows-x64.exe
[macos-sillicon]: https://github.com/danirod/cartero/releases/download/v0.1.3/Cartero-0.1.3-macOS-arm64.dmg
[macos-intel]: https://github.com/danirod/cartero/releases/download/v0.1.3/Cartero-0.1.3-macOS-amd64.dmg
[appimage-x86_64]: https://github.com/danirod/cartero/releases/download/v0.1.3/Cartero-0.1.3-x86_64.AppImage
