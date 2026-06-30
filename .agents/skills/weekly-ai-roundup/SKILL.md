---
name: Weekly AI Roundup
description: Instructions for updating the Weekly AI Roundup data for AI PulseQ
---

# Weekly AI Roundup Updating Skill

This skill provides instructions on how to update the `frontend/public/weekly_roundups.json` file every week.

## Schedule
You should perform this update every **Friday**. 

## Definition of a Week
A week is defined as **Monday to Sunday**, with no weekends excluded (full 7 days included).

## Instructions

1. **Fetch Data:** Obtain the weekly roundup data from the official Gemini share link: `https://share.gemini.google/bjPbwIavu9qM`
   - *Note: Since the share link is a single-page app, you may need to use a browser subagent to read the fully rendered text.*

2. **Parse Data:** The text will have new sections for each week (e.g., "Week 4: July 15 - July 21, 2024").

3. **Format Data:** For each new week, prepare a JSON object matching the existing schema in `frontend/public/weekly_roundups.json`:
   ```json
   {
     "id": "weekX",
     "title": "Week X: Date - Date, Year",
     "title_bn": "সপ্তাহ X: ...",
     "summary": "Brief English summary of the week's top stories.",
     "summary_bn": "Brief Bengali summary of the week's top stories.",
     "content": "### Topic 1\nEnglish details...\n\n### Topic 2\nEnglish details...",
     "content_bn": "### Topic 1\nBengali details...\n\n### Topic 2\nBengali details..."
   }
   ```

4. **Translate:** You must manually translate the `title`, `summary`, and `content` into Bengali (`title_bn`, `summary_bn`, `content_bn`) exactly as formatted in the English version. Keep any markdown like `###` intact.

5. **Update JSON:** Read `frontend/public/weekly_roundups.json` and prepend or insert the new week's JSON object into the array.

6. **Deploy:** Compile the frontend and push the changes to Git.
