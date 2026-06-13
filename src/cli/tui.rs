use anyhow::Result;
use blink_md::models::block::Block as NotionBlock;
use blink_md::models::common::User;
use blink_md::models::database::Database;
use blink_md::models::page::Page;
use blink_md::NotionClient;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block as WidgetBlock, Borders, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};
use std::io;

#[derive(PartialEq, Eq)]
enum TabsState {
    Users,
    Pages,
    BlocksTree,
    Databases,
    Search,
}

impl TabsState {
    fn titles() -> [&'static str; 5] {
        ["Users", "Pages", "Blocks", "DBs", "Search"]
    }

    fn index(&self) -> usize {
        match self {
            TabsState::Users => 0,
            TabsState::Pages => 1,
            TabsState::BlocksTree => 2,
            TabsState::Databases => 3,
            TabsState::Search => 4,
        }
    }

    fn from_index(i: usize) -> Self {
        match i {
            0 => TabsState::Users,
            1 => TabsState::Pages,
            2 => TabsState::BlocksTree,
            3 => TabsState::Databases,
            4 => TabsState::Search,
            _ => TabsState::Users,
        }
    }
}

struct App {
    client: NotionClient,
    tab: TabsState,

    users: Vec<User>,
    users_selected: usize,

    pages: Vec<Page>,
    pages_selected: usize,

    block_tree: Vec<TreeNode>,
    block_tree_selected: usize,
    current_page_id: Option<String>,

    databases: Vec<Database>,
    db_selected: usize,

    search_query: String,
    search_results: Vec<serde_json::Value>,
    search_selected: usize,

    detail_text: String,
    status_message: String,
    is_loading: bool,
    show_help: bool,
    flattened_cache: Vec<TreeNode>,
    needs_reflatten: bool,
}

#[derive(Clone)]
struct TreeNode {
    indent: usize,
    block: NotionBlock,
    expanded: bool,
    children: Vec<TreeNode>,
}

impl App {
    fn new(client: NotionClient) -> Self {
        Self {
            client,
            tab: TabsState::Users,
            users: vec![],
            users_selected: 0,
            pages: vec![],
            pages_selected: 0,
            block_tree: vec![],
            block_tree_selected: 0,
            current_page_id: None,
            databases: vec![],
            db_selected: 0,
            search_query: String::new(),
            search_results: vec![],
            search_selected: 0,
            detail_text: String::new(),
            status_message: String::from("Ready"),
            is_loading: false,
            show_help: false,
            flattened_cache: vec![],
            needs_reflatten: true,
        }
    }

    fn set_status(&mut self, msg: &str) {
        self.status_message = msg.to_string();
    }

    async fn load_users(&mut self) -> Result<()> {
        self.is_loading = true;
        self.set_status("Loading users...");
        let res = self.client.list_users().await;
        self.is_loading = false;
        match res {
            Ok(users) => {
                self.users = users;
                if !self.users.is_empty() {
                    self.users_selected = 0;
                    self.show_user_detail();
                }
                self.set_status("Users loaded");
            }
            Err(e) => self.set_status(&format!("Error: {}", e)),
        }
        Ok(())
    }

    async fn load_pages(&mut self) -> Result<()> {
        self.is_loading = true;
        self.set_status("Loading pages...");
        let res = self.client.search(None, None, None, None, None).await;
        self.is_loading = false;
        match res {
            Ok(results) => {
                self.pages = results
                    .results
                    .into_iter()
                    .filter_map(|v| serde_json::from_value::<Page>(v).ok())
                    .collect();
                if !self.pages.is_empty() {
                    self.pages_selected = 0;
                    self.show_page_detail();
                }
                self.set_status("Pages loaded");
            }
            Err(e) => self.set_status(&format!("Error: {}", e)),
        }
        Ok(())
    }

    async fn load_block_tree(&mut self, page_id: &str) -> Result<()> {
        self.is_loading = true;
        self.set_status("Loading blocks...");
        let res = self.client.get_block_children(page_id, None, None).await;
        self.is_loading = false;
        match res {
            Ok(list) => {
                self.block_tree = list
                    .results
                    .into_iter()
                    .map(|b| TreeNode {
                        indent: 0,
                        block: b,
                        expanded: false,
                        children: vec![],
                    })
                    .collect();
                self.block_tree_selected = 0;
                self.needs_reflatten = true;
                self.show_block_detail();
                self.current_page_id = Some(page_id.to_string());
                self.set_status("Blocks loaded");
            }
            Err(e) => self.set_status(&format!("Error: {}", e)),
        }
        Ok(())
    }

    async fn toggle_expand(&mut self, index: usize) -> Result<()> {
        let (block_id, is_expanded, has_children, indent) = {
            let flat = self.get_flattened();
            if index >= flat.len() {
                return Ok(());
            }
            let node = &flat[index];
            (
                node.block.id.clone(),
                node.expanded,
                node.block.has_children,
                node.indent,
            )
        };

        if is_expanded {
            if let Some(node) = self.find_node_mut(index) {
                node.expanded = false;
                node.children.clear();
                self.needs_reflatten = true;
            }
        } else if has_children {
            let list = self
                .client
                .get_block_children(&block_id, None, None)
                .await?;
            let children: Vec<TreeNode> = list
                .results
                .into_iter()
                .map(|b| TreeNode {
                    indent: indent + 1,
                    block: b,
                    expanded: false,
                    children: vec![],
                })
                .collect();

            if let Some(node) = self.find_node_mut(index) {
                node.children = children;
                node.expanded = true;
                self.needs_reflatten = true;
            }
        } else {
            if let Some(node) = self.find_node_mut(index) {
                node.expanded = true;
                self.needs_reflatten = true;
            }
        }
        Ok(())
    }

    fn find_node_mut(&mut self, target_idx: usize) -> Option<&mut TreeNode> {
        let mut current_idx = 0;
        Self::find_node_recursive_mut(&mut self.block_tree, target_idx, &mut current_idx)
    }

    fn find_node_recursive_mut<'a>(
        nodes: &'a mut [TreeNode],
        target_idx: usize,
        current_idx: &mut usize,
    ) -> Option<&'a mut TreeNode> {
        for node in nodes {
            if *current_idx == target_idx {
                return Some(node);
            }
            *current_idx += 1;
            if node.expanded {
                if let Some(found) =
                    Self::find_node_recursive_mut(&mut node.children, target_idx, current_idx)
                {
                    return Some(found);
                }
            }
        }
        None
    }

    fn get_flattened(&mut self) -> &[TreeNode] {
        if self.needs_reflatten {
            let mut flat = vec![];
            Self::flatten_recursive(&self.block_tree, &mut flat);
            self.flattened_cache = flat;
            self.needs_reflatten = false;
        }
        &self.flattened_cache
    }

    fn flatten_recursive(nodes: &[TreeNode], out: &mut Vec<TreeNode>) {
        for node in nodes {
            out.push(node.clone());
            if node.expanded {
                Self::flatten_recursive(&node.children, out);
            }
        }
    }

    fn flatten_tree<'a>(&'a self, nodes: &'a [TreeNode]) -> Vec<&'a TreeNode> {
        let mut flat = vec![];
        for node in nodes {
            flat.push(node);
            if node.expanded {
                flat.extend(self.flatten_tree(&node.children));
            }
        }
        flat
    }

    fn show_user_detail(&mut self) {
        if let Some(user) = self.users.get(self.users_selected) {
            self.detail_text = format!(
                "Name: {}\nID: {}\nType: {:?}\nAvatar: {:?}",
                user.name.as_deref().unwrap_or("N/A"),
                user.id,
                user.user_type,
                user.avatar_url
            );
        }
    }

    fn show_page_detail(&mut self) {
        if let Some(page) = self.pages.get(self.pages_selected) {
            self.detail_text = format!(
                "ID: {}\nURL: {}\nTrashed: {}\nTitle: {}",
                page.id,
                page.url,
                page.in_trash,
                page.title_from_properties()
            );
        }
    }

    fn show_block_detail(&mut self) {
        let idx = self.block_tree_selected;
        if let Some(node) = self.get_flattened().get(idx) {
            self.detail_text = format!(
                "ID: {}\nType: {}\nHas children: {}\nTrashed: {}",
                node.block.id,
                node.block.type_str(),
                node.block.has_children,
                node.block.in_trash
            );
        }
    }

    async fn load_databases(&mut self) -> Result<()> {
        self.is_loading = true;
        self.set_status("Loading databases...");
        let res = self
            .client
            .search(
                None,
                Some(serde_json::json!({
                    "property": "object",
                    "value": "database"
                })),
                None,
                None,
                None,
            )
            .await;
        self.is_loading = false;
        match res {
            Ok(results) => {
                self.databases = results
                    .results
                    .into_iter()
                    .filter_map(|v| serde_json::from_value(v).ok())
                    .collect();
                if !self.databases.is_empty() {
                    self.db_selected = 0;
                    self.detail_text = format!("Database ID: {}", self.databases[0].id);
                }
                self.set_status("Databases loaded");
            }
            Err(e) => self.set_status(&format!("Error: {}", e)),
        }
        Ok(())
    }

    async fn search(&mut self) -> Result<()> {
        if self.search_query.is_empty() {
            return Ok(());
        }
        self.is_loading = true;
        self.set_status("Searching...");
        let res = self
            .client
            .search(Some(self.search_query.clone()), None, None, None, None)
            .await;
        self.is_loading = false;
        match res {
            Ok(results) => {
                self.search_results = results.results;
                if !self.search_results.is_empty() {
                    self.search_selected = 0;
                    self.detail_text = format!("Found {} results", self.search_results.len());
                } else {
                    self.detail_text = "No results".into();
                }
                self.set_status("Search complete");
            }
            Err(e) => self.set_status(&format!("Error: {}", e)),
        }
        Ok(())
    }

    fn list_len(&mut self) -> usize {
        match &self.tab {
            TabsState::Users => self.users.len(),
            TabsState::Pages => self.pages.len(),
            TabsState::BlocksTree => self.get_flattened().len(),
            TabsState::Databases => self.databases.len(),
            TabsState::Search => self.search_results.len(),
        }
    }

    fn selected_index(&self) -> usize {
        match &self.tab {
            TabsState::Users => self.users_selected,
            TabsState::Pages => self.pages_selected,
            TabsState::BlocksTree => self.block_tree_selected,
            TabsState::Databases => self.db_selected,
            TabsState::Search => self.search_selected,
        }
    }

    fn set_selected_index(&mut self, idx: usize) {
        match &mut self.tab {
            TabsState::Users => self.users_selected = idx,
            TabsState::Pages => self.pages_selected = idx,
            TabsState::BlocksTree => self.block_tree_selected = idx,
            TabsState::Databases => self.db_selected = idx,
            TabsState::Search => self.search_selected = idx,
        }
    }

    fn next_item(&mut self) {
        let len = self.list_len();
        if len > 0 {
            self.set_selected_index((self.selected_index() + 1) % len);
            self.update_detail();
        }
    }

    fn prev_item(&mut self) {
        let len = self.list_len();
        if len > 0 {
            let idx = self.selected_index();
            self.set_selected_index(if idx == 0 { len - 1 } else { idx - 1 });
            self.update_detail();
        }
    }

    fn update_detail(&mut self) {
        match &self.tab {
            TabsState::Users => self.show_user_detail(),
            TabsState::Pages => self.show_page_detail(),
            TabsState::BlocksTree => self.show_block_detail(),
            TabsState::Databases => {
                if let Some(db) = self.databases.get(self.db_selected) {
                    self.detail_text =
                        format!("Database ID: {}\nTitle: {}", db.id, db.title_text());
                }
            }
            TabsState::Search => {
                if let Some(val) = self.search_results.get(self.search_selected) {
                    self.detail_text = serde_json::to_string_pretty(val).unwrap_or_default();
                }
            }
        }
    }
}

pub async fn run_tui(client: NotionClient) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(client);
    app.load_users().await?;

    let res = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    res
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()>
where
    B::Error: std::error::Error + Send + Sync + 'static,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        if app.show_help {
                            app.show_help = false;
                        } else {
                            return Ok(());
                        }
                    }
                    KeyCode::Char('?') => app.show_help = !app.show_help,
                    KeyCode::Tab => {
                        let idx = (app.tab.index() + 1) % TabsState::titles().len();
                        app.tab = TabsState::from_index(idx);
                        match app.tab {
                            TabsState::Users => app.load_users().await?,
                            TabsState::Pages => app.load_pages().await?,
                            TabsState::BlocksTree => {
                                if let Some(id) = app.current_page_id.clone() {
                                    app.load_block_tree(&id).await?;
                                }
                            }
                            TabsState::Databases => app.load_databases().await?,
                            _ => {}
                        }
                    }
                    KeyCode::BackTab => {
                        let idx = (app.tab.index() + TabsState::titles().len() - 1)
                            % TabsState::titles().len();
                        app.tab = TabsState::from_index(idx);
                        match app.tab {
                            TabsState::Users => app.load_users().await?,
                            TabsState::Pages => app.load_pages().await?,
                            TabsState::BlocksTree => {
                                if let Some(id) = app.current_page_id.clone() {
                                    app.load_block_tree(&id).await?;
                                }
                            }
                            TabsState::Databases => app.load_databases().await?,
                            _ => {}
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => app.next_item(),
                    KeyCode::Up | KeyCode::Char('k') => app.prev_item(),
                    KeyCode::Backspace => {
                        if app.tab == TabsState::BlocksTree {
                            app.tab = TabsState::Pages;
                        } else if app.tab == TabsState::Search {
                            app.search_query.pop();
                        }
                    }
                    KeyCode::Enter => match &app.tab {
                        TabsState::Pages => {
                            if let Some(page) = app.pages.get(app.pages_selected) {
                                let id = page.id.clone();
                                app.load_block_tree(&id).await?;
                                app.tab = TabsState::BlocksTree;
                            }
                        }
                        TabsState::BlocksTree => {
                            let flat = app.flatten_tree(&app.block_tree);
                            if let Some(_node) = flat.get(app.block_tree_selected) {
                                app.toggle_expand(app.block_tree_selected).await?;
                            }
                        }
                        TabsState::Search => {
                            app.search().await?;
                        }
                        _ => match app.tab {
                            TabsState::Users => app.load_users().await?,
                            TabsState::Pages => app.load_pages().await?,
                            TabsState::Databases => app.load_databases().await?,
                            _ => {}
                        },
                    },
                    KeyCode::Char(c) if app.tab == TabsState::Search => {
                        app.search_query.push(c);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let [tabs_area, main_area, footer_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(f.area());

    let theme = crate::cli::theme::Theme::notion();
    let titles: Vec<Line> = TabsState::titles()
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let style = if i == app.tab.index() {
                Style::default()
                    .fg(theme.accent_text)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.stone_gray)
            };
            Line::from(Span::styled(*t, style))
        })
        .collect();

    let title_prefix = if app.is_loading { "⟳ " } else { "" };
    let tabs = Tabs::new(titles)
        .block(
            WidgetBlock::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border))
                .title(format!("{}notion-rs TUI", title_prefix)),
        )
        .select(app.tab.index())
        .highlight_style(
            Style::default()
                .fg(theme.accent_text)
                .bg(theme.accent)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, tabs_area);

    let [list_area, detail_area] =
        Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
            .areas(main_area);

    let list_items: Vec<ListItem> = match &app.tab {
        TabsState::Users => app
            .users
            .iter()
            .map(|u| {
                let name = u.name.clone().unwrap_or_default();
                ListItem::new(Line::from(Span::styled(
                    name,
                    Style::default().fg(theme.primary_text),
                )))
            })
            .collect(),
        TabsState::Pages => app
            .pages
            .iter()
            .map(|p| {
                let title = p.title_from_properties();
                ListItem::new(Line::from(Span::styled(
                    title,
                    Style::default().fg(theme.primary_text),
                )))
            })
            .collect(),
        TabsState::BlocksTree => {
            let flat = app.get_flattened();
            flat.iter()
                .map(|node| {
                    let prefix =
                        "  ".repeat(node.indent) + if node.expanded { "▼ " } else { "▶ " };
                    let text = format!("{}{}", prefix, node.block.type_str());
                    ListItem::new(Line::from(Span::styled(
                        text,
                        Style::default().fg(theme.primary_text),
                    )))
                })
                .collect()
        }
        TabsState::Databases => app
            .databases
            .iter()
            .map(|db| {
                let title = db.title_text();
                ListItem::new(Line::from(Span::styled(
                    title,
                    Style::default().fg(theme.primary_text),
                )))
            })
            .collect(),
        TabsState::Search => app
            .search_results
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let id_str = v.get("id").and_then(|id| id.as_str());
                let text = match id_str {
                    Some(s) => s.to_string(),
                    None => format!("result {}", i),
                };
                ListItem::new(Line::from(Span::styled(
                    text,
                    Style::default().fg(theme.primary_text),
                )))
            })
            .collect(),
    };

    let list = List::new(list_items)
        .block(
            WidgetBlock::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border))
                .title("Items"),
        )
        .highlight_style(Style::default().fg(theme.surface).bg(theme.accent))
        .highlight_symbol("> ");
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(app.selected_index()));
    f.render_stateful_widget(list, list_area, &mut list_state);

    let mut detail = app.detail_text.clone();
    if app.tab == TabsState::Search {
        detail = format!("Search: {}\n\n{}", app.search_query, detail);
    }
    let detail_style = if detail.contains("Error") || detail.contains("error") {
        Style::default().fg(theme.error)
    } else if detail.contains("Success") || detail.contains("success") {
        Style::default().fg(theme.success)
    } else {
        Style::default().fg(theme.primary_text)
    };
    let detail_para = Paragraph::new(detail)
        .block(
            WidgetBlock::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border))
                .title("Detail"),
        )
        .style(detail_style)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(detail_para, detail_area);

    // Footer
    let footer_text = format!(
        " [q]uit | [tab] next tab | [j/k] move | [enter] select | Status: {}",
        app.status_message
    );
    let footer = Paragraph::new(Line::from(Span::styled(
        footer_text,
        Style::default().fg(theme.stone_gray),
    )));
    f.render_widget(footer, footer_area);

    if app.show_help {
        show_help_popup(f, theme);
    }
}

fn show_help_popup(f: &mut Frame, theme: crate::cli::theme::Theme) {
    use ratatui::widgets::Clear;

    let block = WidgetBlock::default()
        .title(" Keyboard Shortcuts ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent));

    let help_text = vec![
        Line::from(vec![
            Span::styled("[q/Esc]   ", Style::default().fg(theme.accent)),
            Span::raw("Quit program or close popup"),
        ]),
        Line::from(vec![
            Span::styled("[Tab]     ", Style::default().fg(theme.accent)),
            Span::raw("Switch to next tab"),
        ]),
        Line::from(vec![
            Span::styled("[BTab]    ", Style::default().fg(theme.accent)),
            Span::raw("Switch to previous tab"),
        ]),
        Line::from(vec![
            Span::styled("[j/k]     ", Style::default().fg(theme.accent)),
            Span::raw("Move selection Up/Down"),
        ]),
        Line::from(vec![
            Span::styled("[Enter]   ", Style::default().fg(theme.accent)),
            Span::raw("Select item or expand/collapse block"),
        ]),
        Line::from(vec![
            Span::styled("[Backspc] ", Style::default().fg(theme.accent)),
            Span::raw("Go back to Pages from Blocks"),
        ]),
        Line::from(vec![
            Span::styled("[?]       ", Style::default().fg(theme.accent)),
            Span::raw("Toggle this help screen"),
        ]),
    ];

    let paragraph = Paragraph::new(help_text).block(block);
    let area = centered_rect(60, 40, f.area());
    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
