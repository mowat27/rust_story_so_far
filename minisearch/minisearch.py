#!/usr/bin/env python3

import os
import sys


def main():
    try:
        config = Config(sys.argv)
        run(config)
    except ValueError as ex:
        print("Error parsing arguments:", ex)
        sys.exit(1)
    except OSError as ex:
        print("Error reading file:", ex)
        sys.exit(1)


class Config:
    def __init__(self, args):
        if len(args) < 3:
            raise ValueError('Not enough arguments')
        self.query = args[1]
        self.filename = args[2]


def run(config):
    with open(config.filename) as file:
        contents = file.read().split("\n")
        for line in search(config.query, contents):
            print(line)


def search(query, contents):
    return [line for line in contents if query in line]


if __name__ == '__main__':
    main()
