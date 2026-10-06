//! Community links: Help menu, About dialog and home screen open the
//! Split Happens pages (no Discord, no upstream promo).

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use printcraft_engine::links;
use printcraft_ui_egui::{Dialog, PrintCraftApp};

fn harness(setup: impl FnOnce(&mut PrintCraftApp) + 'static) -> Harness<'static, PrintCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PrintCraftApp::new();
        setup(&mut app);
        app
    });
    h.run_steps(4);
    h
}

#[test]
fn no_discord_or_upstream_links_anywhere() {
    for l in links::LINKS {
        assert!(!l.url.contains("discord"), "{}", l.url);
        assert!(!l.url.contains("storytold"), "{}", l.url);
        assert!(!l.url.contains("getartcraft"), "{}", l.url);
    }
}

#[test]
fn home_screen_links() {
    for l in links::LINKS {
        let mut h = harness(|_| {});
        h.get_by_label("About Split Happens");
        h.get_by_label(l.label).click();
        h.run_steps(2);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(l.url), "{}", l.label);
    }
}

#[test]
fn about_dialog_shows_the_brand_and_links() {
    let pdf = b"%PDF-1.7\n1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj\n3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] >> endobj\ntrailer << /Root 1 0 R >>\n%%EOF";
    // With a document open, so the home screen's own links are not on screen.
    let mut h = harness(move |app| {
        app.open_bytes("one.pdf", None, pdf.to_vec()).unwrap();
        app.dialog = Some(Dialog::About);
    });
    h.get_by_label("Split Happens");
    // Brand attribution (not the upstream trademark) is shown.
    h.get_by_label("Based on PrintCraft");
}

#[test]
fn help_commands_open_each_link() {
    for l in links::LINKS {
        let mut h = harness(|_| {});
        assert!(h.state_mut().execute(l.command), "{}", l.command);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(l.url));
        assert_eq!(printcraft_engine::commands::command(l.command).unwrap().menu, Some("Help"));
    }
}
