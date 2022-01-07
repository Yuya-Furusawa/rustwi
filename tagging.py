#!/usr/bin/python3

import subprocess

def main():
    subprocess.run("git tag | xargs git tag -d", shell=True)

    commit_hashes = subprocess.getoutput("git log --reverse --pretty='%h'")
    commit_hashes = commit_hashes.split("\n")
    for i, commit_hash in enumerate(commit_hashes):
        subprocess.run("git tag {} {}".format(i, commit_hash), shell=True)

if __name__ == '__main__':
    main()
