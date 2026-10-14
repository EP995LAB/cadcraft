//! MIRROR with MIRRTEXT 0: a TEXT or MTEXT stays readable but takes the box its mirror image
//! would occupy, measured with its style's font (#1169). Observed black-box in AutoCAD 2027 for
//! Mac for a left-justified TEXT mirrored about a vertical line: the copy's insertion point lands
//! at −(x + width), the far end of the mirrored box.

use cadcraft_engine::Session;
use cadcraft_engine::doc::{EntityKind, HAlign, Handle};
use cadcraft_engine::geom::{Bounds2, Mat3, Vec2};
use cadcraft_engine::render;
use serde_json::{Value, json};

fn handle(v: &Value) -> Handle {
    Handle::parse_hex(v.as_str().unwrap()).unwrap()
}

/// MIRROR `h` about the line `p1`–`p2`, keeping the source; the copy's handle.
fn mirror(s: &mut Session, h: Handle, p1: [f64; 2], p2: [f64; 2]) -> Handle {
    let r = s.execute("mirror", &json!({ "handles": [h.hex()], "p1": p1, "p2": p2 })).unwrap();
    handle(&r["handles"][0])
}

/// The box a text draws in, measured like the renderer draws it.
fn drawn_box(s: &Session, h: Handle) -> Bounds2 {
    let d = s.doc().unwrap();
    match &d.entity(h).unwrap().kind {
        EntityKind::Text(t) => render::place_text_entity(d, t, &t.value).1,
        EntityKind::MText(t) => render::layout_mtext_entity(d, t).bounds,
        k => panic!("not a text: {k:?}"),
    }
}

fn mirrored(b: &Bounds2, p1: [f64; 2], p2: [f64; 2]) -> Bounds2 {
    let m = Mat3::mirror(Vec2::new(p1[0], p1[1]), Vec2::new(p2[0], p2[1]));
    Bounds2::from_points(b.corners().map(|c| m.apply(c)))
}

fn same_box(a: &Bounds2, b: &Bounds2) -> bool {
    a.min.near(b.min, 1e-6) && a.max.near(b.max, 1e-6)
}

fn text_of(s: &Session, h: Handle) -> cadcraft_engine::doc::Text {
    match &s.doc().unwrap().entity(h).unwrap().kind {
        EntityKind::Text(t) => t.clone(),
        k => panic!("not a TEXT: {k:?}"),
    }
}

#[test]
fn text_mirrored_about_a_vertical_line_takes_its_mirrored_extent() {
    let mut s = Session::new();
    let r = s.execute("text", &json!({ "at": [10, 10], "text": "ABC", "height": 5 })).unwrap();
    let src = handle(&r["handle"]);
    let width = drawn_box(&s, src).width();
    assert!(width > 1.0, "the text has a width: {width}");
    let copy = mirror(&mut s, src, [0.0, 0.0], [0.0, 100.0]);
    let t = text_of(&s, copy);
    // Readable (rotation 0), still left-justified, starting at the far end of the mirrored box.
    assert!(t.rotation.abs() < 1e-9, "rotation {}", t.rotation);
    assert_eq!(t.halign, HAlign::Left);
    assert!(t.insert.xy().near(Vec2::new(-(10.0 + width), 10.0), 1e-6), "insertion {:?}, width {width}", t.insert);
    assert!(same_box(&drawn_box(&s, copy), &mirrored(&drawn_box(&s, src), [0.0, 0.0], [0.0, 100.0])));
    // The source is untouched.
    assert!(text_of(&s, src).insert.xy().near(Vec2::new(10.0, 10.0), 1e-9));
}

#[test]
fn justified_text_mirrored_about_a_vertical_line_keeps_its_box() {
    for j in ["R", "C", "TL", "BR", "M"] {
        let mut s = Session::new();
        let r = s.execute("text", &json!({ "at": [10, 10], "text": "ABC", "height": 5, "justify": j })).unwrap();
        let src = handle(&r["handle"]);
        let before = text_of(&s, src);
        let copy = mirror(&mut s, src, [0.0, 0.0], [0.0, 100.0]);
        let t = text_of(&s, copy);
        assert!(t.rotation.abs() < 1e-9, "{j}: rotation {}", t.rotation);
        assert_eq!((t.halign, t.valign), (before.halign, before.valign), "{j}: justification kept");
        let want = mirrored(&drawn_box(&s, src), [0.0, 0.0], [0.0, 100.0]);
        let got = drawn_box(&s, copy);
        assert!(same_box(&got, &want), "{j}: drawn {got:?}, mirrored box {want:?}");
    }
}

#[test]
fn text_mirrored_about_a_horizontal_line_keeps_its_box() {
    // The readable text sits in the mirrored box, below the line: its baseline moves down by the
    // box height, its rotation stays 0.
    let mut s = Session::new();
    let r = s.execute("text", &json!({ "at": [10, 10], "text": "ABC", "height": 5 })).unwrap();
    let src = handle(&r["handle"]);
    let copy = mirror(&mut s, src, [0.0, 0.0], [100.0, 0.0]);
    let t = text_of(&s, copy);
    assert!(t.rotation.abs() < 1e-9, "rotation {}", t.rotation);
    assert!(t.insert.y < -10.0, "the baseline moves below the mirrored insertion point: {:?}", t.insert);
    assert!((t.insert.x - 10.0).abs() < 1e-9);
    assert!(same_box(&drawn_box(&s, copy), &mirrored(&drawn_box(&s, src), [0.0, 0.0], [100.0, 0.0])));
}

#[test]
fn text_mirrored_about_a_slanted_line_turns_with_its_box_and_reads_upward() {
    // Mirrored about the 45° line through the origin, the box of a horizontal text stands
    // upright; the text reads from the bottom (rotation 90°), never upside down.
    let mut s = Session::new();
    let r = s.execute("text", &json!({ "at": [10, 10], "text": "ABC", "height": 5 })).unwrap();
    let src = handle(&r["handle"]);
    let copy = mirror(&mut s, src, [0.0, 0.0], [100.0, 100.0]);
    let t = text_of(&s, copy);
    assert!((t.rotation - std::f64::consts::FRAC_PI_2).abs() < 1e-9, "rotation {}", t.rotation.to_degrees());
    assert!(same_box(&drawn_box(&s, copy), &mirrored(&drawn_box(&s, src), [0.0, 0.0], [100.0, 100.0])));
}

#[test]
fn mtext_mirrored_about_a_vertical_line_stays_readable_in_its_mirrored_box() {
    let mut s = Session::new();
    let r = s.execute("mtext", &json!({ "at": [10, 30], "text": "MTEXT", "height": 5, "width": 50 })).unwrap();
    let src = handle(&r["handle"]);
    let copy = mirror(&mut s, src, [0.0, 0.0], [0.0, 100.0]);
    let d = s.doc().unwrap();
    let EntityKind::MText(t) = &d.entity(copy).unwrap().kind else { panic!("expected an MTEXT") };
    assert!(t.rotation.abs() < 1e-9, "readable, not upside down: rotation {}", t.rotation.to_degrees());
    assert_eq!(t.attach, 1, "attachment kept");
    // Top-left of the mirrored 50-wide frame: x from −60 to −10.
    assert!(t.insert.xy().near(Vec2::new(-60.0, 30.0), 1e-6), "insertion {:?}", t.insert);
    assert!(same_box(&drawn_box(&s, copy), &mirrored(&drawn_box(&s, src), [0.0, 0.0], [0.0, 100.0])));
}

#[test]
fn mirror_preview_shows_the_text_where_it_lands() {
    // The drag preview transforms texts one by one (the GPU mirrors the rest): it must place
    // them exactly where the command does.
    let mut s = Session::new();
    let r = s.execute("text", &json!({ "at": [10, 10], "text": "ABC", "height": 5 })).unwrap();
    let src = handle(&r["handle"]);
    for t in ["mirror", "all", "", "0,0"] {
        s.cmdline(t).unwrap();
    }
    let (_, extra) = s.drag(Vec2::new(0.0, 100.0)).expect("MIRROR drags its selection");
    let EntityKind::Text(shown) = &extra[1].kind else { panic!("expected the text") };
    let shown = shown.clone();
    s.cmdline("0,100").unwrap();
    s.cmdline("").unwrap();
    let copy = s.doc().unwrap().model.iter().map(|e| e.handle).filter(|h| *h != src).last().unwrap();
    let t = text_of(&s, copy);
    assert!(shown.insert.xy().near(t.insert.xy(), 1e-9), "preview {:?}, result {:?}", shown.insert, t.insert);
    assert!(shown.insert.x < -10.0, "the preview takes the mirrored extent: {:?}", shown.insert);
}
