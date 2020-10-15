from typing import NoReturn
from os import environ
from sys import argv

def run_fake_command(keyword: str) -> NoReturn:
  with open(environ[f'FAKE_{keyword}_OUTPUT_FILE'], 'w') as output:
    for x in argv[1:]: output.write(x + '\n')
  exit(int(environ[f'FAKE_{keyword}_EXIT_STATUS']))
