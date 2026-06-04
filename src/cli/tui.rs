use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use notion_rs::models::block::Block as NotionBlock;
use notion_rs::models::common::User;
use notion_rs::models::database::Database;
use notion_rs::models::page::Page;
use notion_rs::NotionClient;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
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
        }
    }

    async fn load_users(&mut self) -> Result<()> {
        self.users = self.client.list_users().await?;
        if !self.users.is_empty() {
            self.users_selected = 0;
            self.show_user_detail();
        }
        Ok(())
    }

    async fn load_pages(&mut self) -> Result<()> {
        let results = self.client.search(None, None, None, None).await?;
        self.pages = results
            .results
            .into_iter()
            .filter_map(|v| serde_json::from_value::<Page>(v).ok())
            .collect();
        if !self.pages.is_empty() {
            self.pages_selected = 0;
            self.show_page_detail();
        }
        Ok(())
    }

    async fn load_block_tree(&mut self, page_id: &str) -> Result<()> {
        let list = self.client.get_block_children(page_id, None, None).await?;
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
        self.show_block_detail();
        self.current_page_id = Some(page_id.to_string());
        Ok(())
    }

    async fn toggle_expand(&mut self, index: usize) -> Result<()> {
        if let Some(node) = self.block_tree.get_mut(index) {
            if node.expanded {
                node.expanded = false;
                node.children.clear();
            } else {
                if node.block.has_children {
                    let list = self.client.get_block_children(&node.block.id, None, None).await?;
                    node.children = list
                        .results
                        .into_iter()
                        .map(|b| TreeNode {
                            indent: node.indent + 1,
                            block: b,
                            expanded: false,
                            children: vec![],
                        })
                        .collect();
                }
                node.expanded = true;
            }
        }
        Ok(())
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
        let flat = self.flatten_tree(&self.block_tree);
        if let Some(node) = flat.get(self.block_tree_selected) {
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
        let results = self
            .client
            .search(
                None,
                Some(serde_json::json!({
                    "property": "object",
                    "value": "database"
                })),
                None,
                None,
            )
            .await?;
        self.databases = results
            .results
            .into_iter()
            .filter_map(|v| serde_json::from_value(v).ok())
            .collect();
        if !self.databases.is_empty() {
            self.db_selected = 0;
            self.detail_text = format!("Database ID: {}", self.databases[0].id);
        }
        Ok(())
    }

    async fn search(&mut self) -> Result<()> {
        if self.search_query.is_empty() {
            return Ok(());
        }
        let results = self
            .client
            .search(Some(self.search_query.clone()), None, None, None)
            .await?;
        self.search_results = results.results;
        if !self.search_results.is_empty() {
            self.search_selected = 0;
            self.detail_text = format!("Found {} results", self.search_results.len());
        } else {
            self.detail_text = "No results".into();
        }
        Ok(())
    }

    fn list_len(&self) -> usize {
        match &self.tab {
            TabsState::Users => self.users.len(),
            TabsState::Pages => self.pages.len(),
            TabsState::BlocksTree => self.flatten_tree(&self.block_tree).len(),
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
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
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
                    KeyCode::Backspace if app.tab == TabsState::Search => {
                        app.search_query.pop();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let [tabs_area, main_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(f.area());

    let titles: Vec<Line> = TabsState::titles()
        .iter()
        .map(|t| Line::from(Span::styled(*t, Style::default().fg(Color::White))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(
            WidgetBlock::default()
                .borders(Borders::ALL)
                .title("notion-rs TUI"),
        )
        .select(app.tab.index())
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    f.render_widget(tabs, tabs_area);

    let [list_area, detail_area] =
        Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
            .areas(main_area);

    let list_items: Vec<ListItem> = match &app.tab {
        TabsState::Users => app
            .users
            .iter()
            .map(|u| ListItem::new(u.name.clone().unwrap_or_default()))
            .collect(),
        TabsState::Pages => app
            .pages
            .iter()
            .map(|p| ListItem::new(p.title_from_properties()))
            .collect(),
        TabsState::BlocksTree => {
            let flat = app.flatten_tree(&app.block_tree);
            flat.iter()
                .map(|node| {
                    let prefix =
                        "  ".repeat(node.indent) + if node.expanded { "▼ " } else { "▶ " };
                    let text = format!("{}{}", prefix, node.block.type_str());
                    ListItem::new(text)
                })
                .collect()
        }
        TabsState::Databases => app
            .databases
            .iter()
            .map(|db| ListItem::new(db.title_text()))
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
                ListItem::new(text)
            })
            .collect(),
    };

    let list = List::new(list_items)
        .block(WidgetBlock::default().borders(Borders::ALL).title("Items"))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .highlight_symbol("> ");
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(app.selected_index()));
    f.render_stateful_widget(list, list_area, &mut list_state);

    let mut detail = app.detail_text.clone();
    if app.tab == TabsState::Search {
        detail = format!("Search: {}\n\n{}", app.search_query, detail);
    }
    let detail_para = Paragraph::new(detail)
        .block(WidgetBlock::default().borders(Borders::ALL).title("Detail"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(detail_para, detail_area);
}
