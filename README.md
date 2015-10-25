# BASMAP
**B**untine's **A**wesome **S**ite**M**ap **A**udit **P**rogram is a command line utility for auditing the [Sitemap](https://en.wikipedia.org/wiki/Sitemaps) at a given website.

BASMAP will fetch all URLs from the given sitemap and report on any HTTP errors it encounters. It's a useful tool for providing a very high-level integration test suite and may be useful as a post-deploy hook to ensure that a change in one webpage has not affected other webpages unintentionally.

BASMAP is written in the Rust programming language.

## Installation

If you want a standalone binary:

  * [Download the standalone binary from the latest release](https://github.com/buntine/basmap/releases)
  * Ensure it's executable ```$ chmod 755 /full/path/to/basmap```
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
    -z, --gzip          Decode gzipped response
    -r, --redirects     Consider HTTP redirects (30x) successful
    -v, --verbose       Print verbose summary
        --google        Ping Sitemap to Google
        --bing          Ping Sitemap to Bing
        --min-ping      Minimum success rate % required to ping search engines (default 100)
    -h, --help          Print this help menu
```

## Examples


20 concurrents with a 2 second sleep.
```
$ basmap http://www.news.com.au/sitemap.xml -c 20 -s 2000 -r
```

60 concurrents with a 1 second sleep. Ping search engines on 95%+ success rate
```
$ basmap http://www.news.com.au/sitemap.xml -c 60 --min-ping 95 --google --bing
```

Decode gzipped sitemap
```
$ basmap http://hardhatdigital.com.au/sitemap.xml.gz -z
```
