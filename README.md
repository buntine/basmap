# BASMAP
**B**untine's **A**wesome **S**ite**M**ap **A**udit **P**rogram is a command line utility for auditing the [Sitemap](https://en.wikipedia.org/wiki/Sitemaps) at a given website.

BASMAP will fetch all URLs from the given sitemap and report on any HTTP errors it encounters. It's a useful tool for providing a very high-level integration test suite and may be useful as a post-deploy hook to ensure that a change in one webpage has not affected other webpages unintentionally.

## Installation

If you want a standalone binary:

  * [Download the latest version](https://github.com/buntine/basmap/tree/master/dist)
  * Setup a symlink to somewhere on your path ```$ ln -s /full/path/to/basmap /usr/local/bin/basmap```

Or to build from source, clone this repo and:

```
$ cargo build
```

## Usage

```
basmap URL [options]

Options:
    -c NUMBER           Amount of concurrent HTTP requests (chunk size) (default 5)
    -s NUMBER           Milliseconds to sleep between chunks (default 1000)
    -z, --gzip          Decode gzip response
    -r, --redirects     Consider HTTP redirects (30x) successful
    -v, --verbose       Print progress verbosely
    -h, --help          Print this help menu
        --google        Ping Sitemap to Google
        --bing          Ping Sitemap to Bing
        --min-ping      Minimum success rate % required to ping search engines (default 100)
```

# Example

```
$ basmap http://news.com.au/sitemap.xml -c 20 -s 2000 -r
```
