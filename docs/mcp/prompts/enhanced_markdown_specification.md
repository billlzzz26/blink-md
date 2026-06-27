# Notion-flavored Markdown Specification

Text format supporting all Notion Block and Rich Text types.

## Characters Requiring Escape

Escape with backslash: `\ * ~ ` $ [ ] < > { } | ^`

## Block Types

Blocks use `{color="Color"}` attribute for block-level color.

### Text
```
Rich text {color="Color"}
```

### Headings
```
# Rich text {color="Color"}
## Rich text {color="Color"}
### Rich text {color="Color"}
#### Rich text {color="Color"}
```
H5-H6 convert to H4 (Notion limitation).

### Lists
```
- Rich text {color="Color"}
    Children

1. Rich text {color="Color"}
    Children
```
List items should contain inline rich text; empty items render awkwardly.

### Empty Line
```
<empty-block/>
```
Required for intentional blank lines. Strips otherwise.

### Quote
```
> Rich text {color="Color"}
    Children
```
Multi-line quote uses `<br>` for line breaks:
```
> Line 1<br>Line 2<br>Line 3 {color="Color"}
```

### To-do
```
- [ ] Rich text {color="Color"}
    Children
- [x] Rich text {color="Color"}
    Children
```

### Divider
```
---
```

### Code Block
```
```language
Code
```
```
No escaping inside code blocks. Language is required (e.g., mermaid).

### Table
```
<table fit-page-width="true|false" header-row="true|false" header-column="true|false">
    <colgroup>
        <col color="Color">
        <col color="Color">
    </colgroup>
    <tr color="Color">
        <td>Data cell</td>
        <td color="Color">Data cell</td>
    </tr>
</table>
```
Cells contain rich text only. No HTML tags inside cells.

### Equation
```
$$
Equation
$$
```

## Rich Text Formatting

| Format | Syntax |
|--------|--------|
| Bold | `**Rich text**` |
| Italic | `*Rich text*` |
| Strikethrough | `~~Rich text~~` |
| Underline | `<span underline="true">Rich text</span>` |
| Inline code | `` `Code` `` |
| Link | `[Link text](URL)` |
| Citation | `[^URL]` |
| Inline color | `<span color="Color">Rich text</span>` |
| Inline math | `$Equation$` |
| Line break | `<br>` |

## Colors

Text colors: `gray, brown, orange, yellow, green, blue, purple, pink, red`

Background colors: `gray_bg, brown_bg, orange_bg, yellow_bg, green_bg, blue_bg, purple_bg, pink_bg, red_bg`

## Compound Blocks (Page Content Only)

### Toggle
```
<details color="Color">
<summary>Rich text</summary>
    Children
</details>
```
Toggle heading uses `{toggle="true"}`:
```
# Rich text {toggle="true" color="Color"}
    Children
```

### Callout
```
<callout icon="emoji or Notion Icon" color="Color">
    Rich text
    Children
</callout>
```

### Columns
```
<columns>
    <column>
        Children
    </column>
    <column>
        Children
    </column>
</columns>
```

### Page (Sub-page)
```
<page url="{{URL}}" color="Color">Title</page>
```
WARNING: Existing page URLs will MOVE that page as a child. Use `<mention-page>` to reference instead.

### Database
```
<database url="{{URL}}" inline="true|false" icon="Emoji" color="Color" data-source-url="{{URL}}" wiki="true|false">Title</database>
```
- `url`: moves existing database as child
- `data-source-url`: creates linked database view
- `inline`: true = visible on page, false = sub-page

### Media
```
![Caption](URL) {color="Color"}
<audio src="{{URL}}" color="Color">Caption</audio>
<file src="{{URL}}" color="Color">Caption</file>
<pdf src="{{URL}}" color="Color">Caption</pdf>
<video src="{{URL}}" color="Color">Caption</video>
```

## Mentions

```
<mention-user url="{{URL}}">User name</mention-user>
<mention-user url="{{URL}}"/>
<mention-page url="{{URL}}">Page title</mention-page>
<mention-page url="{{URL}}"/>
<mention-database url="{{URL}}">Database name</mention-database>
<mention-database url="{{URL}}"/>
<mention-data-source url="{{URL}}">Data source name</mention-data-source>
<mention-agent url="{{URL}}">Agent name</mention-agent>
<mention-date start="YYYY-MM-DD"/>
<mention-date start="YYYY-MM-DD" startTime="HH:mm" timeZone="IANA_TIMEZONE"/>
<mention-date start="YYYY-MM-DD" startTime="HH:mm" end="YYYY-MM-DD" endTime="HH:mm" timeZone="IANA_TIMEZONE"/>
```

## Advanced Blocks

### Synced Block
```
<synced_block url="{{URL}}">
    Children
</synced_block>
```
Omit `url` when creating new synced blocks.

### Synced Block Reference
```
<synced_block_reference url="{{URL}}" notice="{{OPTIONAL_NOTICE}}">
    Children
</synced_block_reference>
```

### Meeting Notes
```
<meeting-notes>
    Rich text (meeting title)
    <summary>
        AI-generated summary
    </summary>
    <notes>
        User notes
    </notes>
</meeting-notes>
```
- Create without `<summary>` and `<transcript>` tags
- Only add `<notes>` when user requests note content
- `<transcript>` cannot be edited (generated from audio)

### Table of Contents
```
<table_of_contents color="Color"/>
```

### Unknown Block
```
<unknown url="{{URL}}" alt="Alt"/>
```