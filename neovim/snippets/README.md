# Adding Snippets

To add new snippets, create or edit the JSON files in this directory:
- `typescript.json` - for .ts files
- `typescriptreact.json` - for .tsx files
- `javascript.json` - for .js files
- `javascriptreact.json` - for .jsx files

## Snippet Format

```json
{
  "Snippet Name": {
    "prefix": "trigger",
    "body": [
      "line 1",
      "line 2 with ${1:placeholder}",
      "line 3 with ${2:another placeholder}"
    ],
    "description": "What this snippet does"
  }
}
```

## Variables
- `${1}`, `${2}`, etc. - Tab stops
- `${1:default}` - Tab stop with default text
- `${1}` can appear multiple times to mirror input

## Usage
1. Type the prefix (e.g., "rfc")
2. Press Tab to expand (or select from autocomplete menu)
3. Press Tab to jump to next placeholder
4. Press Shift+Tab to jump to previous placeholder

Alternative keybindings:
- Ctrl+n to expand/jump forward
- Ctrl+p to jump backward