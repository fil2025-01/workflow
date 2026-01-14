[Prev](./page8.md)

# Template-Based Rendering

This document details the refactoring of the recording list rendering logic to use HTML `<template>` elements.

## Overview
To improve code maintainability and separate structure from logic, the manual creation of DOM elements in `script.ts` has been replaced with standard HTML templates.

## Changes

### 1. HTML Templates (`index.html`)
Three templates were added to define the UI structure:
- **`table-template`**: Defines the overall `<table>` structure, including headers.
- **`row-template`**: Defines a single `<tr>` with specific classes for column targeting (`col-no`, `col-name`, etc.).
- **`empty-template`**: Defines the placeholder view when no recordings are found.

### 2. Logic Updates (`script.ts`)
The `loadRecordings` function now utilizes the [Template API](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/template):
- Uses `document.importNode(template.content, true)` to instantiate the UI.
- Targets specific elements using CSS classes instead of relying on DOM order.
- Populates data (audio sources, transcript text) via standard DOM properties.

## Benefits
- **Separation of Concerns**: Structure is defined in HTML, styling in CSS, and logic in TypeScript.
- **Performance**: Cloning templates is generally more efficient than repeated `document.createElement` and `innerHTML` assignments.
- **Maintainability**: Designers or developers can modify the table layout in `index.html` without touching the TypeScript logic.

[Prev](./page8.md)
