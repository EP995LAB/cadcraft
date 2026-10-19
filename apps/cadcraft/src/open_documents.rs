//! Drawings opened from Finder (double-click, Open With), a drop on the Dock icon and
//! `open -a CADCraft plan.dwg`, and quit requests from the system (#158).
//!
//! None of these put the path in argv: LaunchServices sends the running (or just-launched) app a
//! `kAEOpenDocuments` ('odoc') Apple event. winit 0.30 doesn't handle it and owns the
//! `NSApplicationDelegate`, so AppKit answered "CADCraft cannot open files in the "Drawing"
//! format". Handling it needs an Objective-C class, i.e. `unsafe`, which this workspace forbids:
//! the pinned `fmv-macos-events` crate holds exactly that (an `NSAppleEventManager` handler for
//! 'odoc' and 'quit', registered before the launch event arrives, that leaves winit's delegate
//! alone) behind a safe main-thread API. What the app does with the events is here and is the
//! same on every platform, so it is tested everywhere; only [`mac`] is macOS-only.

use std::path::PathBuf;

use cadcraft_ui_egui::CadApp;

/// What the system asked for.
#[derive(Debug)]
pub enum DocEvent {
    /// Open these files, in this order.
    Open(Vec<PathBuf>),
    /// Quit (Dock ▸ Quit, logging out, `osascript -e 'quit app "CADCraft"'`).
    Quit,
}

/// The paths to hand to OPEN: UTF-8 and non-empty, each once, in the order they arrived. A second
/// copy of the same path (a drop can repeat one) would open the drawing twice.
fn openable(paths: impl IntoIterator<Item = PathBuf>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for p in paths {
        if let Some(s) = p.to_str().filter(|s| !s.is_empty())
            && !out.iter().any(|o| o == s)
        {
            out.push(s.to_string());
        }
    }
    out
}

/// Open the files through the same path as File ▸ Open (a failure is reported on the command line
/// and in the status bar), and handle a quit as the window's close button does: with unsaved
/// drawings the Save / Don't Save / Cancel prompt asks first. Returns whether the window may close
/// now.
pub fn dispatch(events: impl IntoIterator<Item = DocEvent>, app: &mut CadApp) -> bool {
    let mut quit = false;
    for e in events {
        match e {
            DocEvent::Open(paths) => {
                for p in openable(paths) {
                    app.open_path(&p);
                }
            }
            DocEvent::Quit => quit = true,
        }
    }
    quit && !app.request_quit()
}

/// The Apple-event bridge.
#[cfg(target_os = "macos")]
pub mod mac {
    use super::DocEvent;
    use cadcraft_ui_egui::CadApp;
    use fmv_macos_events::{Event, Inbox, Registration};

    /// Keeps the Apple-event handlers registered; hold it until the event loop returns.
    pub struct AppleEvents {
        _registration: Registration,
        inbox: Inbox,
    }

    impl AppleEvents {
        /// Register the handlers. Call on the main thread (`main`) before the event loop starts,
        /// so the event that launched the app (a Finder double-click) is caught too.
        pub fn install() -> Self {
            let (registration, inbox) = Registration::install();
            Self { _registration: registration, inbox }
        }

        /// The queue the app drains every frame ([`poll`]); events arriving later wake `ctx`.
        pub fn connect(&self, ctx: &egui::Context) -> Inbox {
            let ctx = ctx.clone();
            self.inbox.set_wake(move || ctx.request_repaint());
            self.inbox.clone()
        }
    }

    /// Handle what arrived since the last frame; a quit with nothing unsaved closes the window.
    pub fn poll(inbox: &Inbox, app: &mut CadApp, ctx: &egui::Context) {
        let events = inbox.drain().into_iter().map(|e| match e {
            Event::Open(paths) => DocEvent::Open(paths),
            Event::Quit => DocEvent::Quit,
        });
        if super::dispatch(events, app) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DocEvent, dispatch, openable};
    use cadcraft_engine::Session;
    use cadcraft_ui_egui::closing::Choice;
    use cadcraft_ui_egui::{CadApp, Services};
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    static NEXT_DIR: AtomicUsize = AtomicUsize::new(0);

    /// A directory of its own for each test, removed afterwards.
    struct TestDir(PathBuf);

    impl TestDir {
        fn new() -> Self {
            let n = NEXT_DIR.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!("cadcraft-open-documents-{}-{n}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn app() -> CadApp {
        crate::install_io();
        CadApp::new(Session::new(), Services::default())
    }

    /// Wait for the background reads to finish and add the drawings.
    fn settle(app: &mut CadApp) {
        let deadline = Instant::now() + Duration::from_secs(20);
        while app.opening.busy() && Instant::now() < deadline {
            app.finish_opening();
            std::thread::sleep(Duration::from_millis(5));
        }
        app.finish_opening();
        assert!(!app.opening.busy(), "the background reads finished");
    }

    #[test]
    fn keeps_order_and_drops_duplicates_and_empty_paths() {
        let paths = ["/a/plan.dxf", "", "/b/site.dwg", "/a/plan.dxf"].map(PathBuf::from);
        assert_eq!(openable(paths), ["/a/plan.dxf", "/b/site.dwg"]);
        assert!(openable(Vec::new()).is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn skips_paths_that_are_not_utf8() {
        use std::os::unix::ffi::OsStringExt;
        let bad = PathBuf::from(std::ffi::OsString::from_vec(vec![b'/', 0xff, b'.', b'd', b'x', b'f']));
        assert_eq!(openable([bad, PathBuf::from("/ok.dxf")]), ["/ok.dxf"]);
    }

    /// A Finder open of a DWG and a DXF (names with spaces and decomposed accents, as Finder
    /// sends them) opens each once beside the unsaved drawing, which stays as it was.
    #[test]
    fn opens_dwg_and_dxf_beside_an_unsaved_drawing() {
        let dir = TestDir::new();
        let dxf = dir.0.join("pianta con spazi.dxf");
        let dwg = dir.0.join("Documentac\u{327}a\u{303}o.dwg");
        let bytes = b"0\nSECTION\n2\nENTITIES\n0\nLINE\n8\n0\n10\n0\n20\n0\n11\n10\n21\n10\n0\nENDSEC\n0\nEOF\n";
        let drawing = cadcraft_io::read(bytes, "synthetic.dxf").unwrap();
        std::fs::write(&dxf, bytes).unwrap();
        std::fs::write(&dwg, cadcraft_io::write(&drawing, "synthetic.dwg").unwrap()).unwrap();

        let mut app = app();
        app.run("line", serde_json::json!({"points": [[0, 0], [1, 1]]})).unwrap();
        let original = app.session.docs[0].uid;
        assert!(app.session.docs[0].is_dirty());
        assert!(!dispatch([DocEvent::Open(vec![dxf.clone(), dwg.clone(), dxf.clone(), PathBuf::new()])], &mut app));
        settle(&mut app);

        assert_eq!(app.session.docs.len(), 3, "each file opened once beside the unsaved drawing");
        assert_eq!(app.session.docs[0].uid, original);
        assert!(app.session.docs[0].is_dirty());
        for path in [&dxf, &dwg] {
            assert!(app.session.docs.iter().any(|d| d.path.as_deref() == path.to_str()), "{path:?}");
        }
    }

    #[test]
    fn a_failed_open_reports_the_path_and_keeps_the_drawings() {
        let dir = TestDir::new();
        let missing = dir.0.join("missing drawing.dwg");
        let mut app = app();
        let original = app.session.docs[0].uid;
        assert!(!dispatch([DocEvent::Open(vec![missing.clone()])], &mut app));
        settle(&mut app);
        assert_eq!(app.session.docs.len(), 1);
        assert_eq!(app.session.docs[0].uid, original);
        assert!(app.session.log.iter().any(|line| line.contains(missing.to_str().unwrap())), "{:?}", app.session.log);
    }

    #[test]
    fn a_quit_asks_about_unsaved_work_first_and_can_be_cancelled() {
        let mut app = app();
        app.run("line", serde_json::json!({"points": [[0, 0], [1, 1]]})).unwrap();
        assert!(!dispatch([DocEvent::Quit], &mut app), "no window close before the answer");
        assert!(app.close_prompt().is_some());
        assert!(!app.quit_confirmed && !app.quit_requested);
        app.answer_close(Choice::Cancel);
        assert!(app.close_prompt().is_none());
        assert!(app.session.docs[0].is_dirty());
        assert!(!app.quit_requested);
    }

    #[test]
    fn a_quit_closes_a_session_with_nothing_unsaved() {
        let mut app = app();
        assert!(dispatch([DocEvent::Quit], &mut app));
        assert!(app.close_prompt().is_none());
    }
}
