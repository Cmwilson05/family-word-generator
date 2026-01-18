# How to Add New Word Lists

This application now supports multiple word lists that can be dynamically loaded.

## Creating a New Word List

1. **Create a new JavaScript file** (e.g., `family-words.js`, `general-words.js`, etc.)

2. **Define your words array** using the same format (IMPORTANT: use `window.variableName`):
   ```javascript
   window.myWords = [
       { "word": "Example", "topic": "Sample", "difficulty": "Easy" },
       { "word": "Test", "topic": "Sample", "difficulty": "Medium" },
       // ... more words
   ];
   ```

   **⚠️ IMPORTANT:** Use `window.variableName` instead of `const variableName` to ensure the variable is accessible globally when the script is dynamically loaded.

3. **Register the word list** in `index.html`:
   - Open `index.html`
   - Find the `WORD_LISTS` array near the top of the `<script>` section
   - Add your new word list:
   ```javascript
   const WORD_LISTS = [
       { name: "Music Words", file: "words.js", variable: "w" },
       { name: "Family Words", file: "family-words.js", variable: "myWords" },
       // Add more here...
   ];
   ```

## Word List Configuration

Each word list entry has three properties:
- **name**: The display name shown in the dropdown
- **file**: The filename of the JavaScript file containing the words
- **variable**: The variable name used in that JavaScript file

## Example

If you create a file called `action-words.js` with:
```javascript
window.actionList = [
    { "word": "Jump", "topic": "Actions", "difficulty": "Easy" },
    { "word": "Sprint", "topic": "Actions", "difficulty": "Medium" }
];
```

Add to `WORD_LISTS`:
```javascript
{ name: "Action Words", file: "action-words.js", variable: "actionList" }
```

## Benefits

- **Single UI codebase**: Maintain the interface in one place (`index.html`)
- **Multiple word sets**: Support different contexts (music, family, general, etc.)
- **Easy switching**: Users can switch between word lists with the dropdown
- **Modular**: Each word list is a separate file for easy management
