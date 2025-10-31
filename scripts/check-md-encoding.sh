#!/bin/bash
# Check that all .md files contain only ASCII characters (UTF-8 encoded)
# This ensures GitHub and other markdown renderers can display them properly

set -e

echo "Checking all .md files for non-ASCII characters..."

# Use Python for fast, reliable checking
python3 << 'EOF'
import glob
import sys

errors = []
for md_file in glob.glob('**/*.md', recursive=True):
    with open(md_file, 'rb') as f:
        for line_num, line in enumerate(f, 1):
            # Check for any byte > 127 (non-ASCII)
            non_ascii = [i for i, b in enumerate(line) if b > 127]
            if non_ascii:
                errors.append(f'{md_file}:{line_num}')
                # Only report first occurrence per file
                break

if errors:
    print('ERROR: Non-ASCII characters found in markdown files:')
    print('These files will appear as binary on GitHub!')
    print()
    for error in errors:
        print(f'  {error}')
    print()
    print('See docs/process.md section "Markdown File Encoding" for guidance.')
    print('Use ASCII equivalents: us instead of µs, <= instead of ≤, etc.')
    sys.exit(1)

print('✓ All .md files contain only ASCII characters')
EOF

exit 0
