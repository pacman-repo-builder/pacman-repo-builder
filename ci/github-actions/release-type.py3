#! /usr/bin/env python3
from os import environ
import re
import toml

release_tag = environ.get('RELEASE_TAG', None)

if not release_tag:
  print('::error ::Environment variable RELEASE_TAG is required but missing')
  exit(1)

tag_prefix = 'refs/tags/'
if release_tag.startswith(tag_prefix):
  release_tag = release_tag.replace(tag_prefix, '', 1)

def dict_path(source: dict, path: list) -> object:
  if not path: return source
  key, *next = path
  res = source.get(key)
  if res is None: return None
  if type(res) == dict: return dict_path(res, next)
  if next: return None
  return res

with open('Cargo.toml') as cargo_toml:
  data = toml.load(cargo_toml)

  if type(data) != dict:
    print('::error Content of package.json is not an object')
    exit(1)

  version = dict_path(data, ['package', 'version'])

  if not version:
    print('::error ::Cargo.toml#package.version is required but missing')
    exit(1)

  if version != release_tag:
    print(f'::warning ::RELEASE_TAG ({release_tag}) does not match Cargo.toml#package.version ({version})')
    print('::set-output name=release_type::none')
    print('::set-output name=is_release::false')
    print('::set-output name=is_prerelease::false')
    print(f'::set-output name=release_tag::{release_tag}')
    exit(0)

if re.match(r'^[0-9]+\.[0-9]+\.[0-9]+-.+$', release_tag):
  print('::set-output name=release_type::prerelease')
  print('::set-output name=is_release::true')
  print('::set-output name=is_prerelease::true')
  print(f'::set-output name=release_tag::{release_tag}')
  exit(0)

if re.match(r'^[0-9]+\.[0-9]+\.[0-9]+$', release_tag):
  print('::set-output name=release_type::official')
  print('::set-output name=is_release::true')
  print('::set-output name=is_prerelease::false')
  print(f'::set-output name=release_tag::{release_tag}')
  exit(0)

print('::set-output name=release_type::none')
print('::set-output name=is_release::false')
print('::set-output name=is_prerelease::false')
print(f'::set-output name=release_tag::{release_tag}')
exit(0)
