from typing import NoReturn
from os import environ
from sys import argv
import subprocess

def run_fake_command(name: str) -> NoReturn:
  args = argv[1:]

  if any(x == '--query' or x.startswith('-Q') for x in args):
    result = subprocess.run([f'/usr/bin/{name}', *args])
    exit(result.returncode)

  keyword = name.upper()
  with open(environ[f'FAKE_{keyword}_OUTPUT_FILE'], 'w') as output:
    for x in args: output.write(x + '\n')
  exit(int(environ[f'FAKE_{keyword}_EXIT_STATUS']))
