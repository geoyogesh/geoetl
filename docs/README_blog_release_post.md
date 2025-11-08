## 📄 Guidelines: Writing an Open-Source Release Blog Post

### 1\. Purpose of This Document

This document provides official guidelines for creating blog posts announcing new software releases for [Your Project Name]. The goal is to ensure all release announcements are clear, engaging, informative, and consistent.

A release post is more than just a list of changes; it is our primary tool for:

  * **Communicating value** to our users.
  * **Celebrating and crediting** our contributors.
  * **Building community confidence** and excitement.
  * **Driving adoption** of the new version.

### 2\. Core Principles

Before writing, keep these principles in mind:

1.  **Focus on User Value:** Don't just list features. Explain **why** they matter. Always answer the user's question: "What problem does this solve for me?"
2.  **Credit the Community:** Open-source is a collaborative effort. Publicly thanking contributors is non-negotiable. It encourages retention and new contributions.
3.  **Be Clear and Honest:** Be transparent, especially about breaking changes. Trust is our most valuable asset. Hiding or downplaying a breaking change erodes trust faster than the change itself.
4.  **Show, Don't Tell:** Use visuals. Screenshots, animated GIFs, and code snippets are far more powerful than a paragraph of text.
5.  **End with a Clear Action:** The reader should know exactly what to do next (e.g., download, read docs, report bugs).

### 3\. Pre-Writing Checklist

Before you begin writing the post, gather the following assets:

  * `[ ]` **Final Version Number:** (e.g., `v3.1.0`)
  * `[ ]` **Release Theme:** A 1-sentence summary of the release's main focus (e.g., "This release is all about performance and stability.").
  * `[ ]` **Link to Full Changelog:** The canonical `CHANGELOG.md` or GitHub/GitLab release notes page.
  * `[ ]` **Headline Features (1-3):** The "big ticket" items. For each one:
      * `[ ]` A clear explanation of the user-facing benefit.
      * `[ ]` Visual assets (screenshot, GIF, or code block).
  * `[ ]` **List of Breaking Changes:** (If any).
      * `[ ]` A clear "what changed" and "how to migrate" snippet for each.
  * \`[ ]Other Notable Changes:\*\* A bulleted list of smaller fixes and improvements.
  * `[ ]` **Contributor List:** A list of GitHub/GitLab handles for everyone who contributed to this release (especially first-time contributors).
  * `[ ]` **Installation/Update Snippets:** The exact commands (e.g., `npm`, `docker pull`, `pip install`) for users to upgrade.

### 4\. Content and Structure Guidelines

A release post must follow this structure for clarity and scannability.

#### ✅ **1. Title**

  * **Do:** Be direct and informative. Include the project name and version number.
  * **Formats:**
      * `Announcing [Project Name] v[Version Number]`
      * `[Project Name] [Version Number] is here: [Key Feature Highlight]`

#### ✅ **2. Introduction (The Hook)**

  * **Do:** Start with enthusiasm. Immediately state that the new version is available.
  * **Do:** Provide the high-level "theme" of the release.
  * **Don't:** Start with a long history of the project. Get to the point.

#### ✅ **3. Headline Features (The "Why")**

  * **Do:** Dedicate a sub-section (with a `##` or `###` heading) to each major feature (1-3 features maximum).
  * **Do:** Use the **Problem -\> Solution -\> Value** framework:
      * **Problem:** "Previously, syncing large datasets was slow..."
      * **Solution:** "We've introduced a new parallel processing engine..."
      * **Value:** "...which means your sync jobs now complete up to 5x faster."
  * **Do:** Embed visuals (screenshots, GIFs) or "Before/After" code snippets. This is the most important place for them.

#### ✅ **4. Other Improvements & Fixes (The "What Else")**

  * **Do:** Use a scannable bulleted list.
  * **Do:** Group related items (e.g., `Performance`, `Bug Fixes`, `Documentation`).
  * **Don't:** Copy-paste the entire changelog.
  * **Do:** **Always** provide a link to the full, detailed changelog for those who want it.

#### ✅ **5. Breaking Changes (The "Warning")**

  * **This section is mandatory if breaking changes exist.**
  * **Do:** Make this section highly visible. Use a blockquote (`>`) and a clear heading (e.g., `⚠️ Heads Up: Breaking Changes`).
  * **Don't:** Hide this at the very end of the post.
  * **Do:** For each breaking change, clearly explain:
    1.  **What** changed.
    2.  **Who** is affected.
    3.  **How** to migrate (provide code examples if possible).
  * **Do:** Link to a more detailed migration guide in the documentation if one exists.

#### ✅ **6. Community & Contributors (The "Thank You")**

  * **This section is mandatory.**
  * **Do:** Acknowledge that the release was a community effort.
  * **Do:** Give a special shout-out to first-time contributors by name (@[handle]). This is a powerful way to build community.
  * **Don't:** Just say "thanks to all contributors." Make it personal.

#### ✅ **7. The Future (The Roadmap)**

  * **Do:** Briefly mention what the team is planning for the next release (e.g., `v3.2`).
  * **Why:** This builds confidence and shows the project has momentum and a clear direction.
  * **Do:** Link to the public roadmap or project board if available.

#### ✅ **8. Call to Action (The "Now What?")**

  * **Do:** End with clear, direct, and actionable steps.

  * **Do:** Provide copy-pasteable code snippets for installation or upgrade.

    > **Example:**

    > ```bash
    > # For npm
    > npm install [project]@[version]
    > ```

    > # For Docker

    > docker pull [project/image]:[version]

    > ```
    > ```

  * **Do:** Provide links for getting help or giving feedback:

      * "Read the full documentation for [New Feature]"
      * "Found a bug? Report an issue on GitHub"
      * "Have questions? Join our Discord/Slack"

### 5\. Tone and Style

  * **Tone:** Enthusiastic, appreciative, professional, and helpful.
  * **Clarity:** Write in simple, direct language. Avoid overly technical jargon in the main feature descriptions (save the deep details for the docs).
  * **Scannability:** Use **bolding** for key terms, `code_snippets` for commands, headings, and bullet points. **Do not** write a "wall of text."

### 6\. Post-Publish Checklist

The job isn't done when you hit "publish."

  * `[ ]` **Social Media:** Announce the post on all project channels (Twitter/X, LinkedIn, Mastodon, etc.).
  * `[ ]` **Community Channels:** Post the link in your Discord, Slack, Discourse, etc.
  * `[ ]` **News Aggregators:** Share the post on relevant subreddits (e.g., r/opensource, r/programming, r/[language]), Hacker News, and other communities.
  * `[ ]` **Monitor:** Watch for comments and questions on the blog and social media. Be prepared to engage and answer questions for the first 24-48 hours.
