[![Build Status](https://travis-ci.org/dan-t/rust-confsolve.svg?branch=master)](https://travis-ci.org/dan-t/rust-confsolve)
[![](http://meritbadge.herokuapp.com/confsolve)](https://crates.io/crates/confsolve)

confsolve
=========

A command line tool for resolving file synchronization conflicts introduced by
running Dropbox or Wuala.

If you prefer a Haskell version: https://github.com/dan-t/confsolve.

Installation
============

    $ cargo install confsolve

The build binary will be located at `~/.cargo/bin/confsolve`.

Usage
=====

    Usage: confsolve wuala <dir>
           confsolve dropbox <dir>
           confsolve --help
    
    Options:
      -h, --help   Show this message.

Runtime Options
===============

    (T)ake File (NUM) => By pressing 't' and a number (e.g 't1'), the conflicting file with the
                         number NUM is used as the new version. A copy of the
                         current file and the other conflicting files is put
                         into the trash directory '~/.cache/confsolve/trash'.

    (M)ove to Trash   => By pressing 'm', all conflicting files are
                         moved into the trash directory '~/.cache/confsolve/trash'.

    Show (D)iff (NUM) => By pressing 'd' and a number (e.g 'd1'), the difference between the
                         current file and the conflicting file NUM is shown.
                         If there's only one conflicting file, then only pressing
                         'd' is sufficient.
                         By pressing 'd' and two numbers (e.g 'd1 2'), the difference between
                         the two conflicting files is shown.
                         The diff tool can be specified by the user by setting the environment
                         variable 'CONFSOLVE_DIFF'. The default diff tool is 'gvimdiff -f'.

    (S)kip            => By pressing 's', the current conflict is skipped
                         and the next one is shown.

    (Q)uit            => By pressing 'q', the application is quit.

    (H)elp            => By pressing 'h', this help is printed.
