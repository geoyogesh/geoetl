## Architecture Decision Records (ADRs): The Complete Guide

An **Architecture Decision Record (ADR)** is a lightweight text file that captures a single, significant architectural decision, along with its context and consequences.

-----

### 📋 Format (The Michael Nygard Template)

This is the most popular and effective template. Store this as a Markdown file (e.g., `001-use-postgresql-database.md`) in your project's repository (e.g., `/docs/adrs/`).

```markdown
# 1. Title (e.g., "Use PostgreSQL for Primary Database")

* **Status:** [Proposed | Accepted | Deprecated | Superseded]
* **Date:** [YYYY-MM-DD]
* **Deciders:** [List of people who agreed on the decision]

## Context

* What is the problem we are solving?
* What are the technical, business, or other constraints?
* This section explains *why* a decision is needed.

## Decision

* What is the decision we made?
* Be specific and clear. (e.TEST: "We will use PostgreSQL 15 as the primary relational database...")

## Consequences

* What are the positive and negative outcomes of this decision?
* This is the most critical section for future readers.
* **Positive:** (e.g., "Gains strong JSONB support," "Uses a familiar, open-source technology.")
* **Negative:** (e.g., "Adds a new technology to the stack," "Requires training for the ops team.")

## Alternatives Considered

* [Option 1] - (e.g., "Use MySQL")
    * **Reason for rejection:** (e.g., "Less mature JSON support compared to PostgreSQL.")
* [Option 2] - (e.g., "Use MongoDB")
    * **Reason for rejection:** (e.g., "Our data is highly relational and requires strong transactional consistency, making a NoSQL-first approach risky.")
* [Option 3] - (e.g., "Do Nothing / Use current system")
    * **Reason for rejection:** (e.g., "The current SQLite implementation will not scale for production use.")
```

-----

### 🏅 Best Practices

  * **Store with Code:** ADRs **must** live in the same version control repository as the code they apply to. This keeps them in sync, discoverable, and versioned.
  * **Focus on the "Why":** The most valuable parts of an ADR are the **Context** and **Consequences** sections. The `Decision` is *what* you did; these sections are *why* you did it, which is what future teams need to know.
  * **Immutable Records:** Once an ADR is "Accepted," it should **never** be changed. If the decision is reversed, you create a *new* ADR that "Supersedes" the old one (e.g., `005-migrate-to-dynamodb` would supersede `001-use-postgresql-database`).
  * **One Decision per ADR:** Each ADR should capture a *single* significant decision. If you're deciding on a database *and* a caching layer, that's two ADRs.
  * **Define "Significant":** Don't write ADRs for trivial things (like a linter library). A decision is "significant" if it's high-cost, high-impact, or hard to reverse (e.g., choice of framework, database, a core algorithm, a third-party API integration).
  * **Use Lightweight Tooling:** Markdown and Git are all you need. Avoid heavy, external tools (like Confluence or Word docs) that separate the decisions from the code and add friction.

-----

### ✅ Do's and ❌ Don'ts

| Do's | Don'ts |
| :--- | :--- |
| **Do** write ADRs for significant, hard-to-change decisions. | **Don't** write ADRs for trivial or "no-brainer" choices. |
| **Do** store ADRs in the project's Git repository. | **Don't** store them in a separate wiki, document server, or email. |
| **Do** focus on *why* you made the decision (Context, Consequences). | **Don't** just state *what* you decided without justification. |
| **Do** record the alternatives you rejected and *why*. | **Don't** forget to list alternatives; this prevents future debates. |
| **Do** keep them concise and to the point. | **Don't** write a 10-page novel. Link to external docs if needed. |
| **Do** use them as a tool for team discussion (using the "Proposed" status). | **Don't** use ADRs as a "top-down" tool to force decisions on a team. |
| **Do** supersede old ADRs with new ones to show historical evolution. | **Don't** go back and edit an "Accepted" ADR. It's a historical record. |

-----

### 👻 Anti-Patterns (Common Pitfalls)

  * **The ADR Graveyard:** The team writes ADRs but no one ever reads them. They become a "write-only" exercise.
      * **Fix:** Integrate them. When a new team member asks "Why did we use X?", the answer should be "Let's check the ADR for it."
  * **The "Bureaucracy" Anti-Pattern:** Requiring ADRs for every small change. This weaponizes the process, slows down development, and makes people hate ADRs.
      * **Fix:** Be very clear as a team on what is "significant" (see Best Practices).
  * **The "Ivory Tower" ADR:** A senior architect writes an ADR in isolation and hands it down to the team. This skips the most valuable part: collaborative discussion.
      * **Fix:** Use the "Proposed" status to socialize the ADR with the team and gather feedback *before* it's "Accepted."
  * **The "Missing Why":** An ADR that only has a `Title` and `Decision` section. This is useless, as it provides no context or rationale.
      * **Fix:** Always enforce that `Context`, `Consequences`, and `Alternatives` are the most important sections.
  * **The "External Storage" Anti-Pattern:** Storing ADRs in Confluence, SharePoint, or Google Docs. They will instantly be lost, fall out of date, and be disconnected from the codebase they describe.
      * **Fix:** Put them in `docs/adrs/` in your Git repo. No exceptions.
