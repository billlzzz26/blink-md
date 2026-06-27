use anyhow::Result;
use std::path::Path;

const PERSONAS_TOML: &str = include_str!("../registry/personas.toml");
const RECIPES_TOML: &str = include_str!("../registry/recipes.toml");

#[derive(serde::Deserialize)]
struct PersonaRegistry {
    personas: Vec<PersonaEntry>,
}

#[derive(serde::Deserialize)]
struct PersonaEntry {
    name: String,
    title: String,
    description: String,
    services: Vec<String>,
    workflows: Vec<String>,
    instructions: Vec<String>,
    #[serde(default)]
    tips: Vec<String>,
}

#[derive(serde::Deserialize)]
struct RecipeRegistry {
    recipes: Vec<RecipeEntry>,
}

#[derive(serde::Deserialize)]
struct RecipeEntry {
    name: String,
    title: String,
    description: String,
    category: String,
    services: Vec<String>,
    steps: Vec<String>,
    #[serde(default)]
    caution: Option<String>,
}

struct SkillIndexEntry {
    name: String,
    description: String,
    category: String,
}

/// Entry point for `blink-md generate-skills`
pub async fn handle_generate_skills(args: &[String]) -> Result<()> {
    let output_dir = parse_output_dir(args);
    let output_path = Path::new(&output_dir);

    let mut index: Vec<SkillIndexEntry> = Vec::new();

    // Generate personas
    if let Ok(registry) = toml::from_str::<PersonaRegistry>(PERSONAS_TOML) {
        eprintln!(
            "Generating skills for {} personas...",
            registry.personas.len()
        );
        for persona in registry.personas {
            let skill_name = format!("persona-{}", persona.name);
            let md = render_persona_skill(&persona);
            write_skill(output_path, &skill_name, &md)?;
            index.push(SkillIndexEntry {
                name: skill_name.clone(),
                description: truncate_desc(&persona.description),
                category: "persona".to_string(),
            });
        }
    } else {
        eprintln!("WARNING: Failed to parse personas.toml");
    }

    // Generate recipes
    if let Ok(registry) = toml::from_str::<RecipeRegistry>(RECIPES_TOML) {
        eprintln!(
            "Generating skills for {} recipes...",
            registry.recipes.len()
        );
        for recipe in registry.recipes {
            let skill_name = format!("recipe-{}", recipe.name);
            let md = render_recipe_skill(&recipe);
            write_skill(output_path, &skill_name, &md)?;
            index.push(SkillIndexEntry {
                name: skill_name.clone(),
                description: truncate_desc(&recipe.description),
                category: "recipe".to_string(),
            });
        }
    } else {
        eprintln!("WARNING: Failed to parse recipes.toml");
    }

    // Write skills index
    write_skills_index(&index)?;

    eprintln!("\nDone. Skills written to {}/", output_dir);
    Ok(())
}

fn parse_output_dir(args: &[String]) -> String {
    for (i, arg) in args.iter().enumerate() {
        if arg == "--output-dir" {
            if let Some(val) = args.get(i + 1) {
                return val.clone();
            }
        }
    }
    "skills".to_string()
}

fn write_skill(base: &Path, name: &str, content: &str) -> Result<()> {
    let dir = base.join(name);
    std::fs::create_dir_all(&dir)?;
    let path = dir.join("SKILL.md");
    std::fs::write(&path, content)?;
    Ok(())
}

fn render_persona_skill(persona: &PersonaEntry) -> String {
    let skills_list: Vec<String> = persona
        .services
        .iter()
        .map(|s| format!("`{}-mcp`", s))
        .collect();
    let skills_yaml: Vec<String> = persona
        .services
        .iter()
        .map(|s| format!("        - {}-mcp", s))
        .collect();

    let mut out = String::new();

    out.push_str(&format!(
        r#"---
name: persona-{name}
description: "{desc}"
metadata:
  version: {ver}
  openclaw:
    category: "persona"
    requires:
      bins:
        - blink-md
      skills:
{skills}
---

# {title}

> **PREREQUISITE:** Load these skills: {skills_inline}

{description}

## Relevant Workflows

{workflows}

## Instructions

"#,
        name = persona.name,
        desc = truncate_desc(&persona.description),
        ver = env!("CARGO_PKG_VERSION"),
        skills = skills_yaml.join("\n"),
        title = persona.title,
        skills_inline = skills_list.join(", "),
        description = persona.description,
        workflows = persona
            .workflows
            .iter()
            .map(|w| format!("- `recipe-{w}`", w = w))
            .collect::<Vec<_>>()
            .join("\n"),
    ));

    for inst in &persona.instructions {
        out.push_str(&format!("- {inst}\n"));
    }
    out.push('\n');

    if !persona.tips.is_empty() {
        out.push_str("## Tips\n");
        for tip in &persona.tips {
            out.push_str(&format!("- {tip}\n"));
        }
        out.push('\n');
    }

    out
}

fn render_recipe_skill(recipe: &RecipeEntry) -> String {
    let skills_yaml: Vec<String> = recipe
        .services
        .iter()
        .map(|s| format!("        - {}-mcp", s))
        .collect();

    let mut out = String::new();

    out.push_str(&format!(
        r#"---
name: recipe-{name}
description: "{desc}"
metadata:
  version: {ver}
  openclaw:
    category: "recipe"
    domain: "{category}"
    requires:
      bins:
        - blink-md
      skills:
{skills}
---

# {title}

> **PREREQUISITE:** Load skills: {skills_inline}

{description}
"#,
        name = recipe.name,
        desc = truncate_desc(&recipe.description),
        ver = env!("CARGO_PKG_VERSION"),
        category = recipe.category,
        skills = skills_yaml.join("\n"),
        title = recipe.title,
        skills_inline = recipe
            .services
            .iter()
            .map(|s| format!("`{s}-mcp`"))
            .collect::<Vec<_>>()
            .join(", "),
        description = recipe.description,
    ));

    if let Some(caution) = &recipe.caution {
        out.push_str(&format!("> [!CAUTION]\n> {}\n\n", caution));
    }

    out.push_str("## Steps\n\n");
    for (i, step) in recipe.steps.iter().enumerate() {
        out.push_str(&format!("{}. {}\n", i + 1, step));
    }
    out.push('\n');

    out
}

fn truncate_desc(desc: &str) -> String {
    let mut s = desc.replace('"', "'").trim().to_string();
    if !s.ends_with('.') {
        s.push('.');
    }
    s
}

fn write_skills_index(entries: &[SkillIndexEntry]) -> Result<()> {
    let mut out = String::new();
    out.push_str("# Skills Index\n\n");
    out.push_str("> Auto-generated by `blink-md generate-skills`.\n\n");

    for (cat, heading, subtitle) in [
        ("persona", "## Personas", "Role-based skill bundles."),
        ("recipe", "## Recipes", "Multi-step task sequences."),
    ] {
        let items: Vec<&SkillIndexEntry> = entries.iter().filter(|e| e.category == cat).collect();
        if items.is_empty() {
            continue;
        }
        out.push_str(&format!("{heading}\n\n{subtitle}\n\n"));
        out.push_str("| Skill | Description |\n|-------|-------------|\n");
        for item in &items {
            out.push_str(&format!(
                "| [{}](../skills/{}/SKILL.md) | {} |\n",
                item.name, item.name, item.description
            ));
        }
        out.push('\n');
    }

    let path = Path::new("docs/skills.md");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(path, &out)?;
    eprintln!("Skills index written to docs/skills.md");
    Ok(())
}
