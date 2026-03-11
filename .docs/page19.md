[Prev](./page18.md) | [Next](./page20.md)

# 3-Month Strategic Plan: Solving the "Static vs. Dynamic" Paradox

**Goal:** Develop "Workflow" – a high-velocity, audio-first replacement for the current spreadsheet system. The primary objective is to provide a frictionless way to track daily actions (Day Parts 1, 2, and 3) without disrupting cognitive focus, enabling a seamless transition between planning, execution, and routine tasks.

**Problem Statement:** The current spreadsheet-based workflow fails in high-pressure, dynamic scenarios. In a non-linear development environment, it is nearly impossible to define all "next actions" upfront. The mechanical overhead of navigating to a browser, locating a sheet, and manual data entry creates a disruptive context-switch that breaks deep focus (Day Part 2). This system is unsuitable for real-world scenarios where meetings, Slack interruptions, and unexpected bugs require instant, eyes-free task capture to prevent cognitive overload. (See [Page 20](./page20.md) for detailed analysis).

## Developer Growth & Learning Outcomes
This project serves as a high-intensity "Real World" Rust curriculum. Completing it transitions a developer from "Tutorial Knowledge" to "Production Proficiency."

### Core Benefits
*   **Full-Stack Rust Mastery:** Gaining rare proficiency in using Rust for *both* Systems Programming (Backend) and Web UI (Frontend/WASM), breaking the "Rust is only for CLIs" myth.
*   **Async Runtime Expertise:** Deep understanding of Tokio, Futures, and how strictly typed async code differs from JavaScript Promises.
*   **Architectural Discipline:** Learning to structure applications where "Correctness" is enforced by the compiler (Type System, Borrow Checker), resulting in highly robust software design habits that transfer to any language.
*   **Modern Systems Integration:** Experience integrating Database Pools (SQLx), External APIs (Gemini), and Native Browser APIs (Web-sys) in a type-safe environment.

### Measuring the Learning Curve
Success is defined by the *speed and confidence* of implementation over time:
1.  **Month 1 (The Struggle):**
    *   *Metric:* Time spent fighting the Borrow Checker vs. writing logic.
    *   *Goal:* Reduce compilation errors per hour by 50%.
2.  **Month 2 (The Click):**
    *   *Metric:* Ability to refactor code (e.g., splitting `handlers.rs`) without breaking the build.
    *   *Goal:* Implement a new feature (like "Weekly Planning") primarily using established patterns, with minimal consultation of basic syntax docs.
3.  **Month 3 (Fluency):**
    *   *Metric:* Complexity of features vs. bugs introduced.
    *   *Goal:* Deploy advanced features (Search, Auth) with zero runtime panics and < 5% logic bugs in QA.

---

# 3-Month Strategic Plan: Solving the "Static vs. Dynamic" Paradox

## Month 1: Friction Elimination (The "Quick Capture" Core)
**Focus:** Replace the Spreadsheet's "Capture" phase with an instant, low-latency entry point.
**Success Metric:** "Time to Capture" reduced from ~45 seconds (open sheet, scroll, type) to < 3 seconds.

### Phase 1.1: The Foundation (Completed)
*   **Infrastructure:** Set up Rust Backend (Axum), Database (Postgres), and Frontend (Leptos).
*   **Audio Capture:** Implemented one-click voice recording to capture thoughts without typing.
*   **Intelligence:** Integrated Gemini 2.0 to auto-transcribe and title these audio notes.
*   **Basic Organization:** Created "Day Part" groupings to loosely categorize inputs.

### Phase 1.2: Dynamic "Inbox" Management (In Progress)
**Objective:** Create a unified "Inbox" where captured items (audio/text) land immediately, ready for sorting *later* (Day Part 1 work), not *now*.
*   **Text Quick Capture:** Add a CLI command (`workflow add "Fix bug"`) or global hotkey web input to allow capturing text tasks as fast as audio.
*   **Inbox View:** A consolidated list of all unassigned recordings and text notes.
*   **Drag-and-Drop Sorting:** Allow rapid movement of items from "Inbox" to "Day Part 1/2/3" buckets without reloading or manual typing.

---

## Month 2: Strategic Alignment (Context & Planning)
**Focus:** Bridge the gap between "Day Part 1" (Strategy) and "Day Part 2" (Execution).<br>
**Success Metric:** Daily Plan generation takes < 5 minutes and remains relevant despite interruptions.

### Phase 2.1: The "Added Value" Engine
**Objective:** Connect daily execution to high-level strategic goals assigned in VCP (Value Change Plus).
*   **VCP API Integration:** Authenticate and fetch "Assigned Added Values" for the current user from the external VCP system.
*   **AI Execution Roadmap:**
    *   **Input:** User selects an Assigned Value (e.g., "Implement Feature X").
    *   **Process:** Gemini generates a comprehensive, step-by-step execution plan following the strict SDLC: *Requirements -> Backend/Frontend Breakdown -> QA/Testing -> PR/Feedback -> Staging Deployment -> Production Release.*
    *   **Output:** A master list of actionable steps stored in a dedicated "Project Space."
*   **Weekly Commitment:** User selects specific steps from this Master Roadmap to populate the "Weekly Plan."
*   **Reference Linking:** Add a "Reference" field to daily recordings/tasks, linking them back to a specific item in the Weekly Plan. This closes the loop for reporting (Planned vs. Actual Work).
*   **User Review & Correction:**
    *   **Strategic Level:** After Gemini proposes the SDLC roadmap, the user can manually edit, add, or delete steps to ensure the plan perfectly matches the reality of the task.
    *   **Daily Level:** Provide a "Review Mode" for audio transcriptions where the user can correct Gemini's output or add missed details before final submission.

### Phase 2.2: Contextual Reflection (The "AI Coach")
**Objective:** Prevent "Drift" during the day.
*   **Smart Recording:** When recording an update, the system injects the current "Selected Added Value" into the prompt.
*   **Feedback Loop:** After transcription, Gemini reflects: *"You said you were working on Value X, but this task seems unrelated. Is this a distraction?"*
*   **Progress Report:** A visual indicator showing % of tasks completed related to the Weekly Goal vs. "Maintenance/Noise."

---

## Month 3: Flow Mastery (Search & Optimization)
**Focus:** Transform the repository of tasks into a searchable Knowledge Base, eliminating "Data Silos."<br>
**Success Metric:** Zero manual synchronization between "Daily" and "Main" lists.

### Phase 3.1: The "Recall" System
**Objective:** Instant access to past context.
*   **Full-Text Search:** Implement `pg_search` to find "That bug I talked about last Tuesday."
*   **Semantic Search:** Allow natural language queries ("What did I decide about the API structure?") using vector embeddings.

### Phase 3.2: Analytics & Reporting
**Objective:** Visualize the "Day Part" balance.
*   **Energy Audit:** Reports showing time spent in Part 1 (Admin) vs. Part 2 (Deep Work) vs. Part 3 (Routine).
*   **Weekly Roll-up:** Auto-generate the "Weekly Report" by aggregating all "Day Part 2" accomplishments, ready for copy-pasting to stakeholders.

### Phase 3.3: Production Deployment
**Objective:** Reliability and availability.
*   **Security:** Implement Authentication to protect the personal workflow data.
*   **Deployment:** Containerize (Docker) and deploy to a cloud provider for access from any device (Mobile/Desktop).

[Prev](./page18.md) | [Next](./page20.md)
