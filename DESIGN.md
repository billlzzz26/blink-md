# DESIGN SYSTEM

## 1. DESIGN PHILOSOPHY
The `notion-rs` project follows a "Clean, Functional, and Aesthetic" philosophy, mirroring the minimalist and flexible nature of Notion. The design aims for high legibility and a structure that feels both organic and organized.

## 2. COLOR PALETTE & ROLES

### Core Roles
- **Primary**: `#37352F` (Deep Charcoal) - Used for primary text and brand identity.
- **Secondary**: `#9B9A97` (Stone Gray) - Used for secondary text and subtle elements.
- **Surface**: `#FFFFFF` (White) - Main background for documents.
- **Background**: `#F7F6F3` (Soft Gray) - App-level background and sidebar.
- **Accent**: `#2383E2` (Notion Blue) - Interactive elements, links, and focus states.
- **Border**: `#E9E9E7` (Soft Border) - Separators and component borders.

### Semantic Status
- **Success**: `#448361` (Notion Green)
- **Warning**: `#CB912F` (Notion Yellow)
- **Error**: `#D44C47` (Notion Red)
- **Info**: `#337EA9` (Notion Blue)

### Platform Colors (Notion Palette)
| Color | Text | Background |
|-------|------|------------|
| Default | `#37352F` | Transparent |
| Gray | `#9B9A97` | `#F1F1EF` |
| Brown | `#64473A` | `#F4EEEE` |
| Orange| `#D9730D` | `#FBECDD` |
| Yellow| `#CB912F` | `#FBF3DB` |
| Green | `#448361` | `#EDF3EC` |
| Blue  | `#337EA9` | `#E7F3F8` |
| Purple| `#9065B0` | `#F4F0F7` |
| Pink  | `#C14C8A` | `#F9EEF3` |
| Red   | `#D44C47` | `#FFEBE9` |

## 3. TYPOGRAPHY

### Font Families
- **Sans (Default)**: `ui-sans-serif, -apple-system, system-ui, sans-serif`
- **Serif**: `Lyon-Text, Georgia, ui-serif, serif`
- **Mono**: `iawriter-mono, Menlo, Monaco, monospace`

### Typographic Scale
- **H1**: 40px / 1.2 Line Height / Bold
- **H2**: 30px / 1.2 Line Height / Semi-bold
- **H3**: 24px / 1.3 Line Height / Semi-bold
- **Body**: 16px / 1.5 Line Height / Regular
- **Small**: 14px / 1.5 Line Height / Regular
- **Caption**: 12px / 1.5 Line Height / Regular

## 4. SPACING & GRID
Using a 4px base unit.

- **4px**: Tightest spacing (labels, icons)
- **8px**: Small spacing (buttons, lists)
- **12px**: Medium-small (paragraphs, card padding)
- **16px**: Medium (standard sections)
- **24px**: Large (between sections)
- **32px**: Extra Large (header spacing)

## 5. LAYOUT PRINCIPLES
- **Max Width**: 900px for central document content.
- **Grid**: Flexible 12-column grid for complex layouts (Columns Block).
- **Gaps**: 24px standard gutter between columns.

## 6. DEPTH & ELEVATION
- **Flat**: Most elements are flat with subtle borders.
- **Shadow-SM**: `0 1px 2px rgba(0,0,0,0.05)` (Buttons, Input)
- **Shadow-MD**: `0 4px 6px rgba(0,0,0,0.1)` (Modals, Popovers)

## 7. RADIUS
- **Radius-SM**: 4px (Buttons, Inputs)
- **Radius-MD**: 8px (Cards, Modals)
- **Radius-LG**: 16px (Image borders)
- **Full**: 9999px (Pill buttons, Avatars)
