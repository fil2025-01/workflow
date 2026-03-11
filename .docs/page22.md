[Prev](./page21.md) | [Next](./page23.md)

# Plan: UI Overhaul - "Dark Mode & Modern Polish"

**Goal:** Transform the current utilitarian interface into a modern, visually striking application inspired by the **React Aria** aesthetic (see `Screenshot 2026-01-28...`).

**Design Philosophy:** "Content-First, Dark & Vibrant."
We will move away from the "Admin Dashboard" look (light gray backgrounds, standard tables) to a "Developer Tool" aesthetic (dark mode, high contrast, gradients).

---

## Visual Language Guide

### 1. Color Palette
*   **Background:** Deep Charcoal / Black (e.g., `#0f1115` or `#111827`).
*   **Surface (Cards/Sidebar):** Slightly lighter dark grey (`#1f2937`) with subtle borders (`#374151`).
*   **Text:**
    *   Primary: White (`#ffffff`).
    *   Secondary: Cool Grey (`#9ca3af`).
*   **Accent:**
    *   **Primary:** Vibrant Pink/Magenta Gradient (inspired by the "styles" text in the screenshot).
    *   **Secondary:** Electric Blue (for focus states).

### 2. Typography
*   **Font:** System UI / Inter.
*   **Headings:** Large, bold, tracking-tight.
*   **Body:** Clean, legible, slightly relaxed line-height.

### 3. Shape & Depth
*   **Borders:** Thin, subtle borders (`1px solid rgba(255,255,255,0.1)`) instead of heavy shadows.
*   **Radius:** `8px` to `12px` for cards and buttons.
*   **Glassmorphism:** Slight translucency on the sidebar or sticky headers.

---

## Implementation Phases

### Phase 1: Foundation (Theming)
*   **Action:** Rewrite `static/style.css` from scratch using CSS Variables for the new palette.
*   **Variables:** Define `--bg-app`, `--bg-surface`, `--text-main`, `--text-muted`, `--accent-primary`.
*   **Global Reset:** Ensure dark mode is the default state.

### Phase 2: Layout Structure
*   **Sidebar:** Make it a sleek, dark column with subtle icon highlights (removing the heavy "cadetblue").
*   **Main Container:** Center content with max-width, allowing "breathing room" (negative space).
*   **Header:** Remove the "cadetblue" bar. Use a transparent/glass header for filters and actions.

### Phase 3: The "Recording List" Transformation
The current HTML `<table>` looks dated. We will redesign it:
*   **From:** A dense data table.
*   **To:** A **"List of Cards"** or **"Modern Grid"**.
    *   **Parent Row:** A distinct card with the Title, Status badge, and Audio Player.
    *   **Child Rows (Exploded Tasks):** Indented "sub-items" connected by a visual line (tree structure), similar to a code editor's file tree.
    *   **Edit Mode:** When editing a title, the input should look like a code editor text field (dark bg, no border until focus).

### Phase 4: Polish & Micro-Interactions
*   **Buttons:** Replace standard buttons with "Ghost" buttons (transparent bg, text only) or "Gradient Pills" for primary actions.
*   **Focus States:** accessible rings (like the purple ring in the screenshot).
*   **Animations:** Smooth transitions when deleting items or expanding groups.

## Immediate Next Steps
1.  Update `static/style.css` with the new Dark Mode variables.
2.  Refactor `RecordingList` to use a cleaner DOM structure (if needed) or style the existing table to look like a modern list.

[Prev](./page21.md) | [Next](./page23.md)