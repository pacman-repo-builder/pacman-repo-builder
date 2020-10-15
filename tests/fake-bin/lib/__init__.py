from typing import NoReturn
from os import environ
from sys import argv
import subprocess

def run_fake_pacman(keyword: str) -> NoReturn:
  args = argv[1:]

  if any(x == '--query' or x.startswith('-Q') for x in args):
    result = subprocess.run([f'/usr/bin/pacman', *args])
    exit(result.returncode)

  with open(environ[f'FAKE_{keyword}_OUTPUT_FILE'], 'w') as output:
    for x in args: output.write(x + '\n')
  exit(int(environ[f'FAKE_{keyword}_EXIT_STATUS']))
