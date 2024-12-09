# Standard Ruby – A Zed Extension

This extension for the [Zed](https://zed.dev) editor adds support for running [Standard Ruby](https://github.com/standardrb/standard) as a language server.

## Install as a Dev Extension

1. [Install the Rust toolchain via `rustup`](https://rustup.rs)
2. Clone the repo via `git clone https://github.com/himynameisjonas/zed-standardrb.git`
3. Launch Zed and to go to Extensions
4. Click the "Install Dev Extension" button (top right)
5. Select the directory you cloned the repo into
6. Wait a moment while Zed compiles the extension

## Configure Zed for Ruby

First, install the `standard` gem, either via `gem install standard` or by adding it to your Gemfile:

```ruby
gem "standard", require: false, group: :development
```

Next, launch Zed, press <kbd>Command</kbd> + <kbd>,</kbd> to open Settings, and change the language server for Ruby:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["standardrb"]
    }
  }
}
```

Here's a slightly more elaborate Zed config that uses Shopify's [Ruby LSP](https://github.com/Shopify/ruby-lsp) as the primary language server, with auto-formatting and diagnostics handled by Standard Ruby.

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["ruby-lsp", "standardrb", "!rubocop", "!solargraph"],
      "formatter": "language_server",
      "format_on_save": "on"
    }
  },
  "lsp": {
    "ruby-lsp": {
      "initialization_options": {
        "enabledFeatures": {
          "diagnostics": false, // Zed currently doesn't support Ruby LSP's pull-based diagnostics
          "formatting": true
        },
        "formatter": "standard"
      }
    },
    "standardrb": {
      "initialization_options": {
        "enabledFeatures": {
          "diagnostics": true,
          "formatting": false // Because we use Standard Ruby's formatting via Ruby LSP
        }
      }
    }
  }
}
```
