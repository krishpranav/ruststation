#!/usr/bin/env python3
from argparse import ArgumentParser
import json
import os
import platform
import shutil
from subprocess import PIPE, Popen, run
import sys
from urllib.parse import urlparse

def cargo(package, toolchain=None, target=None, release=False, args=None):
    cmd = ['cargo']

    if toolchain is not None:
        cmd.append(f'+{toolchain}')

    id = run(cmd + ['pkgid', '-p', package], stdout=PIPE, check=True).stdout.decode('utf-8').strip()

    url = urlparse(id)
    path = url.path

    if platform.system() == 'Windows':
        path = path[1:]

    cmd.extend(['build', '-p', package])

    if target is not None:
        cmd.extend(['--target', target])

    if release:
        cmd.append('-r')

    if args is not None:
        cmd.extend(args)

    cmd.extend(['--message-format', 'json-render-diagnostics'])

    with Popen(cmd, stdout=PIPE, cwd=path) as proc:
        for line in proc.stdout:
            line = json.loads(line)
            reason = line['reason']
            if reason == 'build-finished':
                if line['success']:
                    break
                else:
                    sys.exit(1)
            elif reason == 'compiler-artifact':
                if line['package_id'] == id:
                    artifact = line

    return artifact

def export_darwin(root, kern, gui):
    bundle = os.path.join(root, 'RustStation.app')
    contents = os.path.join(bundle, 'Contents')
    macos = os.path.join(contents, 'MacOS')
    resources = os.path.join(contents, 'Resources')

    os.mkdir(bundle)
    os.mkdir(contents)
    os.mkdir(macos)
    os.mkdir(resources)

    out, gui = os.path.split(gui['executable'])
    gui = gui.capitalize()

    shutil.copy(kern['executable'], resources)
    shutil.copy(os.path.join(out, gui), macos)
    shutil.copyfile('bundle.icns', os.path.join(resources, 'ruststation.icns'))
    shutil.copy('Info.plist', contents)

    run(['codesign', '-s', '-', '--entitlements', 'entitlements.plist', bundle], check=True)

    return os.path.join(macos, gui)

def export_linux(root, kern, gui):
    bin = os.path.join(root, 'bin')
    share = os.path.join(root, 'share')

    os.mkdir(bin)
    os.mkdir(share)

    shutil.copy(kern['executable'], share)
    shutil.copy(gui['executable'], bin)

    gui = os.path.basename(gui['executable'])

    return os.path.join(bin, gui)

def export_windows(root, kern, gui):
    share = os.path.join(root, 'share')

    os.mkdir(share)

    shutil.copy(kern['executable'], share)
    shutil.copy(gui['executable'], root)

    gui = os.path.basename(gui['executable'])

    return os.path.join(root, gui)

def main():
    p = ArgumentParser(
        description='Script to build RustStation and create distribution file')

    p.add_argument('-r', '--release', action='store_true', help='enable optimization')
    p.add_argument(
        '--root',
        metavar='PATH',
        help='directory to store build outputs')
    p.add_argument(
        '--debug',
        metavar='ADDR',
        help='immediate launch the VMM in debug mode',
        nargs='?',
        const='127.0.0.1:1234')

    args = p.parse_args()

    m = platform.machine()

    if m == 'aarch64' or m == 'arm64':
        kern = cargo(
            'rskrnl',
            toolchain='nightly',
            target='aarch64-unknown-none-softfloat',
            release=args.release,
            args=['-Z', 'build-std=core,alloc'])
    elif m == 'x86_64' or m == 'AMD64':
        kern = cargo(
            'rskrnl',
            target='x86_64-unknown-none',
            release=args.release)
    else:
        print(f'Architecture {m} is not supported.', file=sys.stderr)
        sys.exit(1)

    gui = cargo('gui', release=args.release, args=['--bin', 'ruststation'])

    dest = args.root

    if dest is None:
        dest = 'dist'

        if os.path.exists(dest):
            shutil.rmtree(dest)

        os.mkdir(dest)

    s = platform.system()

    if s == 'Darwin':
        gui = export_darwin(dest, kern, gui)
    elif s == 'Linux':
        gui = export_linux(dest, kern, gui)
    elif s == 'Windows':
        gui = export_windows(dest, kern, gui)
    else:
        print(f'OS {s} is not supported.', file=sys.stderr)
        sys.exit(1)

    addr = args.debug

    if addr is None:
        return

    run([gui, '--debug', addr], check=True)

if __name__ == '__main__':
    main()