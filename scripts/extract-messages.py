#!/usr/bin/env python3
"""Conservative Rust literal extraction for interface strings and declarative tables."""
import ast
import json
from pathlib import Path
import re

root = Path(__file__).resolve().parents[1]
messages = {}
for path in (root / 'src').rglob('*.rs'):
    if path.name.endswith('tests.rs'):
        continue
    for number, line in enumerate(path.read_text().splitlines(), 1):
        for literal in re.findall(r'"(?:[^"\\]|\\.)*"', line):
            try:
                value = ast.literal_eval(literal)
            except (ValueError, SyntaxError):
                continue
            if value and (any(ord(c) > 127 for c in value) or (' ' in value and value[0].isupper())):
                messages.setdefault(value, []).append(f'{path.relative_to(root)}:{number}')
output = ['msgid ""\nmsgstr ""\n"Content-Type: text/plain; charset=UTF-8\\n"\n"Language: \\n"\n']
for value, refs in sorted(messages.items()):
    output.append('#: ' + ' '.join(refs) + '\nmsgid ' + json.dumps(value, ensure_ascii=False) + '\nmsgstr ""\n')
(root / 'po/monolito-settings.pot').write_text('\n'.join(output))
