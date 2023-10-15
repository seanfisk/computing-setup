# Executing the setup

## Subsequent updates

This is easy:

```bash
sudo ./combootcha --homebrew
```

If you don't feel like updating Homebrew, that can of course be omitted.

## First time

### Shared

Everything in this section should be done for all setups.

#### Time Machine

Before doing anything, it is important to set up Time Machine. Select a free partition on an external hard drive and add it to Time Machine. I don't think the partition needs to be erased and reformatted — I think Time Machine will do this automatically. Add the disk and run a backup before going farther.

#### Xcode / Command Line Tools

Next, install either Xcode or Command Line Tools. For Homebrew, Python, Ruby, and normal C++ compiles, we can get along with just the Command Line Tools. However, to compile Mac applications using Qt, we need the full Xcode installation.

If using Command Line Tools, run `xcode-select --install`. If using Xcode, open the Mac App Store and install Xcode from there.

#### Homebrew

Next, change some permissions that Homebrew needs to install Zsh completions. When I ran this setup in early 2023, I had to run this manually to get Homebrew to install properly:

```bash
sudo chown -R $(whoami) /usr/local/share/zsh
chmod u+w /usr/local/share/zsh
```

Next, install Homebrew manually by [following the instructions](https://brew.sh/#install). I am having trouble automating Homebrew system install in this current setup and it doesn't seem worth fretting over automating it right now since it's a one-time install.

After this, run `brew doctor` and attempt to address any problems that are reported.

#### SSH key generation

Run `ssh-keygen` to generate a new key. Accept the defaults. Security of this will be improved in the future; see #3.

Next, upload the public key to GitHub.

#### Combootcha

Transfer the compiled executable to the new system and execute it. The directory of execution does not matter, although the home directory is recommended.

Some of the options are disabled by default because they can take a while to run. However, this is the first run, so all options need to be enabled.

```bash
sudo ./combootcha --homebrew --set-default-browser
```

#### Karabiner

This does not start up properly when using the launch agent approach, so we just use native startup support through the app itself. Looking on 2023-10-15, I can't find where to enable it, so it is possible this setting is automatic now.

#### Jettison

This is licensed software that is installed using Homebrew. I purchased a license that has to be manually entered/activated. Don't get confused: I initially purchased Jettison from the Mac App Store, but bought a separate license when I found that the Mac App Store version isn't up-to-date. The license is stored in LastPass.

#### Desktop backgrounds

No desktop background is set automatically, so go set one of my own preference. This might be nice to automate, but I change these from time to time manually, and that would be just one more thing to change every time.

#### Firefox

Disable the auto-redirection of domains. Specifically this is annoying for `localhost`. Hopefully this will be synced. See http://cdivilly.wordpress.com/2013/08/15/disable-firefox-redirecting-to-localhost-com/.

#### Dash

This is licensed software that is installed using Homebrew. Grab the license from LastPass and run `open license.dash-license`.

#### Cathode

This is licensed software bundled with and installed by Combootcha. Copy the license file from LastPass directly to `~/Library/Application Support/Cathode/License.cathodelicense` to license the software.

#### Seagate's Paragon NTFS driver

TODO Reword this

Visit https://www.seagate.com/support/software/paragon/ and install *Paragon Driver for macOS (Big Sur and later)*.

This is difficult to automate because it's distributed as a DMG with an app bundle in there with a scripted install.

#### Privacy & Security approvals

At minimum, this software needs to be granted permission:

- Hammerspoon
- Karabiner driver

Add more software here as I am prompted.

#### Touch ID

Add prints for right pointer, right middle, and left pointer fingers.

#### aText

TODO Update for new aText

<!-- This is licensed software that is installed using Homebrew Cask. I purchased it from the Mac App Store (MAS). However, as stated in [aText Support](http://www.trankynam.com/atext/support.html), the non-MAS version generally works better. You'll have to follow the process to migrate your MAS license to get the non-MAS version working. -->

<!-- When using the launch agent approach to start up aText, it does not validate the license. So we're just going for the regular startup process. -->

### Personal

Everything in this section is for my personal setup only.

#### LastPass

  - The universal installer has installer and uninstaller app bundles that need to be run manually. Using the universal installer is preferable to individual browser add-ons due to the inclusion of all browser add-ons and the binary component, which allows sharing state between browsers. The universal installer is not present in Homebrew, Homebrew Cask, or the Mac App Store and must be downloaded manually from here: https://lastpass.com/misc_download2.php
